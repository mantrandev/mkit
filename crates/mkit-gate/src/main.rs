use std::env;
use std::fs;
use std::io::{IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const GATE_ITEMS: [&str; 6] = [
    "numbers",
    "money",
    "personal-data",
    "deletion",
    "third-party",
    "permissions",
];

const OK: u8 = 0;
const BLOCKED: u8 = 2;
const USAGE: u8 = 64;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("turn") => run_turn(&args[1..]),
        Some("declare") => run_declare(&args[1..]),
        Some("check") => run_check(&args[1..]),
        Some("--version") | Some("-V") => {
            println!("mkit-gate {}", env!("CARGO_PKG_VERSION"));
            ExitCode::from(OK)
        }
        Some(other) => usage(&format!("unknown command '{other}'")),
        None => usage("expected one of: turn, declare, check"),
    }
}

fn usage(reason: &str) -> ExitCode {
    eprintln!("mkit-gate: {reason}");
    eprintln!("usage: mkit-gate turn");
    eprintln!("       mkit-gate declare --touches <none|item,item> [--decision <path>]");
    eprintln!("       mkit-gate check");
    eprintln!("items: {}", GATE_ITEMS.join(", "));
    ExitCode::from(USAGE)
}

fn blocked(reason: &str) -> ExitCode {
    eprintln!("mkit-gate: {reason}");
    ExitCode::from(BLOCKED)
}

fn run_turn(rest: &[String]) -> ExitCode {
    if !rest.is_empty() {
        return usage("turn takes no arguments");
    }
    let dir = match gate_dir() {
        Ok(value) => value,
        Err(_) => return ExitCode::from(OK),
    };
    if let Err(error) = fs::create_dir_all(&dir) {
        return blocked(&format!("cannot create {}: {error}", dir.display()));
    }
    let marker = dir.join("marker");
    if marker.exists() {
        if let Err(error) = fs::remove_file(&marker) {
            return blocked(&format!("cannot clear the previous request: {error}"));
        }
    }
    let id = new_turn_id();
    if let Err(error) = write_atomic(&dir.join("current"), &format!("{id}\n")) {
        return blocked(&format!("cannot open a new request: {error}"));
    }
    ExitCode::from(OK)
}

fn run_declare(rest: &[String]) -> ExitCode {
    let mut touches: Option<String> = None;
    let mut decision: Option<String> = None;
    let mut index = 0;
    while index < rest.len() {
        let flag = rest[index].as_str();
        let value = match rest.get(index + 1) {
            Some(value) if !value.starts_with("--") => value.clone(),
            _ => return usage(&format!("{flag} needs a value")),
        };
        match flag {
            "--touches" => touches = Some(value),
            "--decision" => decision = Some(value),
            other => return usage(&format!("unknown option '{other}'")),
        }
        index += 2;
    }

    let touches = match touches {
        Some(value) => value,
        None => return usage("declare needs --touches"),
    };

    let items = match parse_items(&touches) {
        Ok(value) => value,
        Err(reason) => return usage(&reason),
    };

    let root = match repo_root() {
        Ok(value) => value,
        Err(reason) => return blocked(&reason),
    };

    let decision = match (items.is_empty(), decision) {
        (true, Some(_)) => {
            return usage("--touches none cannot carry --decision");
        }
        (true, None) => String::new(),
        (false, None) => {
            return blocked(&format!(
                "this request touches {} and no decision was named. Stop and ask the user, then record the answer in docs/decisions/ before editing files",
                items.join(", ")
            ));
        }
        (false, Some(path)) => {
            if let Err(reason) = check_relative_path(&path) {
                return usage(&reason);
            }
            if !root.join(&path).is_file() {
                return blocked(&format!("decision file not found: {path}"));
            }
            path
        }
    };

    let dir = root.join(".mkit").join("gate");
    let turn = match fs::read_to_string(dir.join("current")) {
        Ok(value) => value.trim().to_string(),
        Err(_) => {
            return blocked(
                "no request is open. The mkit turn hook is not installed, so the gate cannot be trusted",
            )
        }
    };
    if turn.is_empty() {
        return blocked("the open request has no identity");
    }

    let body = format!(
        "turn={turn}\ntouches={}\ndecision={decision}\n",
        items.join(",")
    );
    if let Err(error) = write_atomic(&dir.join("marker"), &body) {
        return blocked(&format!("cannot record the gate result: {error}"));
    }
    ExitCode::from(OK)
}

fn run_check(rest: &[String]) -> ExitCode {
    if !rest.is_empty() {
        return usage("check takes no arguments");
    }
    let root = match repo_root() {
        Ok(value) => value,
        Err(reason) => return blocked(&reason),
    };
    if writes_a_decision(&root) {
        return ExitCode::from(OK);
    }
    let dir = root.join(".mkit").join("gate");
    let turn = match fs::read_to_string(dir.join("current")) {
        Ok(value) => value.trim().to_string(),
        Err(_) => return blocked("no request is open. Refusing to change files"),
    };
    if turn.is_empty() {
        return blocked("the open request has no identity. Refusing to change files");
    }
    let marker = match fs::read_to_string(dir.join("marker")) {
        Ok(value) => value,
        Err(_) => {
            return blocked(
                "the authority gate has not run for this request. Check it against the six items, then run: mkit-gate declare --touches <none|items> [--decision <path>]",
            )
        }
    };
    match field(&marker, "turn") {
        Some(value) if value == turn => ExitCode::from(OK),
        Some(_) => {
            blocked("the authority gate ran for an earlier request. Run it again for this one")
        }
        None => blocked("the gate record is unreadable. Refusing to change files"),
    }
}

fn writes_a_decision(root: &Path) -> bool {
    let mut payload = String::new();
    if std::io::stdin().is_terminal() {
        return false;
    }
    if std::io::stdin().read_to_string(&mut payload).is_err() {
        return false;
    }
    let target = match json_string(&payload, "file_path") {
        Some(value) => value,
        None => return false,
    };
    let candidate = if Path::new(&target).is_absolute() {
        PathBuf::from(&target)
    } else {
        root.join(&target)
    };
    let decisions = root.join("docs").join("decisions");
    candidate.starts_with(&decisions)
        && !target.contains("..")
        && candidate.extension().and_then(|value| value.to_str()) == Some("md")
}

fn json_string(body: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let after_key = &body[body.find(&needle)? + needle.len()..];
    let after_colon = &after_key[after_key.find(':')? + 1..];
    let start = after_colon.find('"')? + 1;
    let bytes = after_colon.as_bytes();
    let mut value = String::new();
    let mut index = start;
    while index < bytes.len() {
        match bytes[index] {
            b'"' => return Some(value),
            b'\\' => {
                index += 1;
                match bytes.get(index)? {
                    b'n' => value.push('\n'),
                    b't' => value.push('\t'),
                    other => value.push(*other as char),
                }
            }
            other => value.push(other as char),
        }
        index += 1;
    }
    None
}

fn parse_items(raw: &str) -> Result<Vec<String>, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("--touches cannot be empty".to_string());
    }
    if trimmed == "none" {
        return Ok(Vec::new());
    }
    let mut items: Vec<String> = Vec::new();
    for part in trimmed.split(',') {
        let item = part.trim();
        if item.is_empty() {
            return Err("--touches has an empty item".to_string());
        }
        if item == "none" {
            return Err("none cannot be combined with other items".to_string());
        }
        if !GATE_ITEMS.contains(&item) {
            return Err(format!(
                "unknown item '{item}'. Expected: {}",
                GATE_ITEMS.join(", ")
            ));
        }
        if items.iter().any(|existing| existing == item) {
            return Err(format!("item '{item}' repeats"));
        }
        items.push(item.to_string());
    }
    Ok(items)
}

fn check_relative_path(value: &str) -> Result<(), String> {
    let invalid = value.is_empty()
        || value.starts_with('/')
        || value.ends_with('/')
        || value.contains('\\')
        || value.contains('\0')
        || value
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..");
    if invalid {
        return Err(format!(
            "--decision must be a path inside the repository: {value}"
        ));
    }
    Ok(())
}

fn field<'a>(body: &'a str, key: &str) -> Option<&'a str> {
    body.lines()
        .find_map(|line| line.strip_prefix(key)?.strip_prefix('='))
}

fn gate_dir() -> Result<PathBuf, String> {
    Ok(repo_root()?.join(".mkit").join("gate"))
}

fn repo_root() -> Result<PathBuf, String> {
    if let Some(value) = env::var_os("MKIT_ROOT") {
        let path = PathBuf::from(value);
        if path.is_dir() {
            return Ok(path);
        }
        return Err(format!("MKIT_ROOT is not a directory: {}", path.display()));
    }
    let start = env::current_dir()
        .map_err(|error| format!("cannot read the current directory: {error}"))?;
    let mut cursor: &Path = start.as_path();
    loop {
        if cursor.join(".git").exists() {
            return Ok(cursor.to_path_buf());
        }
        match cursor.parent() {
            Some(parent) => cursor = parent,
            None => return Err("not inside a repository".to_string()),
        }
    }
}

fn write_atomic(target: &Path, body: &str) -> Result<(), String> {
    let parent = target
        .parent()
        .ok_or_else(|| "target has no parent directory".to_string())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    let temporary = parent.join(format!(
        ".{}.{}",
        target
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("mkit"),
        std::process::id()
    ));
    fs::write(&temporary, body).map_err(|error| error.to_string())?;
    fs::rename(&temporary, target).map_err(|error| error.to_string())
}

fn new_turn_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|value| value.as_nanos())
        .unwrap_or(0);
    format!("{nanos:x}-{:x}", std::process::id())
}
