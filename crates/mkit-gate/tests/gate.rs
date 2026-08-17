use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const BIN: &str = env!("CARGO_BIN_EXE_mkit-gate");
const OK: i32 = 0;
const BLOCKED: i32 = 2;
const USAGE: i32 = 64;

static COUNTER: AtomicU32 = AtomicU32::new(0);

struct Lab {
    root: PathBuf,
}

impl Lab {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let unique = COUNTER.fetch_add(1, Ordering::SeqCst);
        let root = std::env::temp_dir().join(format!("mkit-gate-{nanos:x}-{unique}"));
        fs::create_dir_all(root.join(".git")).unwrap();
        fs::create_dir_all(root.join("docs/decisions")).unwrap();
        fs::write(root.join("docs/decisions/0001-x.md"), "x").unwrap();
        Self { root }
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .env("MKIT_ROOT", &self.root)
            .current_dir(&self.root)
            .output()
            .unwrap()
    }

    fn code(&self, args: &[&str]) -> i32 {
        self.run(args).status.code().unwrap()
    }

    fn gate(&self) -> PathBuf {
        self.root.join(".mkit/gate")
    }
}

impl Drop for Lab {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn stderr_of(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

#[test]
fn check_blocks_before_any_request_is_open() {
    let lab = Lab::new();
    assert_eq!(lab.code(&["check"]), BLOCKED);
}

#[test]
fn check_blocks_until_the_gate_is_declared() {
    let lab = Lab::new();
    assert_eq!(lab.code(&["turn"]), OK);
    assert_eq!(lab.code(&["check"]), BLOCKED);
    assert_eq!(lab.code(&["declare", "--touches", "none"]), OK);
    assert_eq!(lab.code(&["check"]), OK);
    assert_eq!(lab.code(&["check"]), OK);
}

#[test]
fn a_new_request_requires_the_gate_again() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    lab.run(&["declare", "--touches", "none"]);
    assert_eq!(lab.code(&["check"]), OK);
    assert_eq!(lab.code(&["turn"]), OK);
    assert_eq!(lab.code(&["check"]), BLOCKED);
}

#[test]
fn touching_a_gate_item_requires_a_decision_that_exists() {
    let lab = Lab::new();
    lab.run(&["turn"]);

    let output = lab.run(&["declare", "--touches", "money"]);
    assert_eq!(output.status.code().unwrap(), BLOCKED);
    assert!(stderr_of(&output).contains("money"));

    assert_eq!(
        lab.code(&[
            "declare",
            "--touches",
            "money",
            "--decision",
            "docs/decisions/nope.md"
        ]),
        BLOCKED
    );
    assert_eq!(lab.code(&["check"]), BLOCKED);

    assert_eq!(
        lab.code(&[
            "declare",
            "--touches",
            "money",
            "--decision",
            "docs/decisions/0001-x.md"
        ]),
        OK
    );
    assert_eq!(lab.code(&["check"]), OK);
}

#[test]
fn every_gate_item_is_accepted() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    for item in [
        "numbers",
        "money",
        "personal-data",
        "deletion",
        "third-party",
        "permissions",
    ] {
        assert_eq!(
            lab.code(&[
                "declare",
                "--touches",
                item,
                "--decision",
                "docs/decisions/0001-x.md"
            ]),
            OK,
            "item {item} was rejected"
        );
    }
    assert_eq!(
        lab.code(&[
            "declare",
            "--touches",
            "money,personal-data,deletion",
            "--decision",
            "docs/decisions/0001-x.md",
        ]),
        OK
    );
}

#[test]
fn decision_paths_may_not_leave_the_repository() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    for path in [
        "../../etc/passwd",
        "/etc/passwd",
        "docs\\decisions",
        "",
        "./x",
        "docs/",
    ] {
        assert_eq!(
            lab.code(&["declare", "--touches", "money", "--decision", path]),
            USAGE,
            "path {path:?} was not rejected"
        );
    }
}

#[test]
fn malformed_arguments_are_refused() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    let cases: Vec<Vec<&str>> = vec![
        vec!["declare", "--touches", "bribery"],
        vec![
            "declare",
            "--touches",
            "none",
            "--decision",
            "docs/decisions/0001-x.md",
        ],
        vec!["declare", "--touches", ""],
        vec![
            "declare",
            "--touches",
            "money,money",
            "--decision",
            "docs/decisions/0001-x.md",
        ],
        vec![
            "declare",
            "--touches",
            "money,none",
            "--decision",
            "docs/decisions/0001-x.md",
        ],
        vec![
            "declare",
            "--touches",
            "money,",
            "--decision",
            "docs/decisions/0001-x.md",
        ],
        vec!["declare", "--touches"],
        vec!["declare", "--touches", "none", "--colour", "red"],
        vec!["declare"],
        vec!["summon"],
        vec!["check", "--force"],
        vec!["turn", "now"],
        vec![],
    ];
    for case in cases {
        assert_eq!(
            lab.code(&case),
            USAGE,
            "arguments {case:?} were not refused"
        );
    }
}

#[test]
fn version_is_reported() {
    let lab = Lab::new();
    let output = lab.run(&["--version"]);
    assert_eq!(output.status.code().unwrap(), OK);
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("mkit-gate "));
}

#[test]
fn corrupted_state_never_opens_the_gate() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    lab.run(&["declare", "--touches", "none"]);
    assert_eq!(lab.code(&["check"]), OK);

    fs::write(lab.gate().join("marker"), "garbage\n").unwrap();
    assert_eq!(lab.code(&["check"]), BLOCKED);

    fs::write(lab.gate().join("marker"), "turn=someone-elses-request\n").unwrap();
    assert_eq!(lab.code(&["check"]), BLOCKED);

    fs::write(lab.gate().join("current"), "").unwrap();
    assert_eq!(lab.code(&["check"]), BLOCKED);

    fs::remove_file(lab.gate().join("current")).unwrap();
    assert_eq!(lab.code(&["check"]), BLOCKED);
}

#[test]
fn declaring_without_an_open_request_is_refused() {
    let lab = Lab::new();
    assert_eq!(lab.code(&["declare", "--touches", "none"]), BLOCKED);
}

#[test]
fn an_unusable_root_blocks() {
    let lab = Lab::new();
    let missing = lab.root.join("not-a-directory");
    let output = Command::new(BIN)
        .arg("check")
        .env("MKIT_ROOT", &missing)
        .output()
        .unwrap();
    assert_eq!(output.status.code().unwrap(), BLOCKED);
}

#[test]
fn a_directory_outside_any_repository_blocks() {
    let outside = std::env::temp_dir().join(format!(
        "mkit-gate-outside-{:x}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&outside).unwrap();
    let output = Command::new(BIN)
        .arg("check")
        .current_dir(&outside)
        .env_remove("MKIT_ROOT")
        .output()
        .unwrap();
    let _ = fs::remove_dir_all(&outside);
    assert_eq!(output.status.code().unwrap(), BLOCKED);
}

fn check_with_payload(lab: &Lab, payload: &str) -> i32 {
    use std::io::Write;
    use std::process::Stdio;
    let mut child = Command::new(BIN)
        .arg("check")
        .env("MKIT_ROOT", &lab.root)
        .current_dir(&lab.root)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(payload.as_bytes())
        .unwrap();
    child.wait().unwrap().code().unwrap()
}

#[test]
fn recording_a_decision_is_never_blocked() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    assert_eq!(lab.code(&["check"]), BLOCKED);

    let allowed = format!(
        r#"{{"tool_name":"Write","tool_input":{{"file_path":"{}/docs/decisions/0013-new.md"}}}}"#,
        lab.root.display()
    );
    assert_eq!(check_with_payload(&lab, &allowed), OK);

    assert_eq!(
        check_with_payload(
            &lab,
            r#"{"tool_input":{"file_path":"docs/decisions/0013-new.md"}}"#
        ),
        OK
    );
}

#[test]
fn the_decision_exemption_cannot_be_abused() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    for payload in [
        r#"{"tool_input":{"file_path":"docs/decisions/../../etc/passwd"}}"#,
        r#"{"tool_input":{"file_path":"docs/decisions/evil.rs"}}"#,
        r#"{"tool_input":{"file_path":"src/main.rs"}}"#,
        r#"{"tool_input":{"file_path":"docs/decisionsX/0013.md"}}"#,
        r#"{"tool_input":{"path":"docs/decisions/0013.md"}}"#,
        r#"{"tool_input":{"file_path":"#,
        "not json at all",
        "",
    ] {
        assert_eq!(
            check_with_payload(&lab, payload),
            BLOCKED,
            "payload {payload:?} slipped through"
        );
    }
}

#[test]
fn opening_a_request_outside_a_repository_never_blocks_the_user() {
    let outside = std::env::temp_dir().join(format!(
        "mkit-gate-turn-{:x}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&outside).unwrap();
    let output = Command::new(BIN)
        .arg("turn")
        .current_dir(&outside)
        .env_remove("MKIT_ROOT")
        .output()
        .unwrap();
    let _ = fs::remove_dir_all(&outside);
    assert_eq!(output.status.code().unwrap(), OK);
}

#[test]
fn the_gate_is_found_from_a_nested_directory() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    lab.run(&["declare", "--touches", "none"]);
    let nested = lab.root.join("crates/deep/src");
    fs::create_dir_all(&nested).unwrap();
    let output = Command::new(BIN)
        .arg("check")
        .current_dir(&nested)
        .env_remove("MKIT_ROOT")
        .output()
        .unwrap();
    assert_eq!(output.status.code().unwrap(), OK);
}

impl Lab {
    fn ledger(&self) -> String {
        fs::read_to_string(self.root.join(".mkit/ledger.jsonl")).unwrap_or_default()
    }
}

#[test]
fn the_gate_records_what_happened() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    assert!(lab.ledger().contains("\"kind\":\"request\""));

    lab.run(&["declare", "--touches", "money"]);
    assert!(lab.ledger().contains("\"kind\":\"refused\""));
    assert!(lab.ledger().contains("\"touches\":[\"money\"]"));

    lab.run(&[
        "declare",
        "--touches",
        "money",
        "--decision",
        "docs/decisions/0001-x.md",
    ]);
    let ledger = lab.ledger();
    assert!(ledger.contains("\"kind\":\"gate\""));
    assert!(ledger.contains("\"decision\":\"0001\""));
    assert!(ledger.contains("\"mkit\":\""));
}

#[test]
fn a_blocked_request_is_recorded_once_however_often_it_retries() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    for _ in 0..10 {
        assert_eq!(lab.code(&["check"]), BLOCKED);
    }
    let blocks = lab.ledger().matches("\"kind\":\"blocked\"").count();
    assert_eq!(blocks, 1, "one refusal per request, not one per attempt");

    lab.run(&["turn"]);
    assert_eq!(lab.code(&["check"]), BLOCKED);
    assert_eq!(lab.ledger().matches("\"kind\":\"blocked\"").count(), 2);
}

#[test]
fn agents_can_record_their_own_signals() {
    let lab = Lab::new();
    assert_eq!(
        lab.code(&["log", "--kind", "ha", "--after", "implement"]),
        OK
    );
    assert_eq!(
        lab.code(&["log", "--kind", "rework", "--after", "fix", "--round", "3"]),
        OK
    );
    assert_eq!(
        lab.code(&[
            "log",
            "--kind",
            "collision",
            "--tags",
            "restraint,architecture"
        ]),
        OK
    );
    let ledger = lab.ledger();
    assert!(ledger.contains("\"kind\":\"ha\",\"after\":\"implement\""));
    assert!(ledger.contains("\"round\":3"));
    assert!(ledger.contains("\"tags\":[\"restraint\",\"architecture\"]"));
}

#[test]
fn the_ledger_refuses_prose_and_secrets() {
    let lab = Lab::new();
    let cases: Vec<Vec<&str>> = vec![
        vec!["log", "--kind", "the user asked about refunds"],
        vec![
            "log",
            "--kind",
            "ha",
            "--after",
            "please fix my billing code",
        ],
        vec!["log", "--kind", "ha", "--tags", "sk-proj-8Kd9SECRETkey"],
        vec!["log", "--kind", "ha", "--tags", "hello world"],
        vec!["log", "--kind", "ha", "--tags", "user@example.com"],
        vec!["log", "--kind", "ha", "--tags", "src/billing/charge.ts"],
        vec!["log", "--kind", "ha", "--tags", "\"quoted\""],
        vec!["log", "--kind", "ha", "--tags", "a,b,c,d,e,f,g,h,i"],
        vec!["log", "--kind", "ha", "--round", "many"],
        vec!["log", "--after", "implement"],
        vec!["log"],
    ];
    for case in cases {
        assert_eq!(lab.code(&case), USAGE, "input {case:?} reached the ledger");
    }
    assert_eq!(lab.ledger(), "", "nothing should have been written");
}

#[test]
fn every_recorded_line_is_machine_shaped() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    lab.run(&[
        "declare",
        "--touches",
        "personal-data,money",
        "--decision",
        "docs/decisions/0001-x.md",
    ]);
    lab.run(&["log", "--kind", "rework", "--after", "plan", "--round", "2"]);

    for line in lab.ledger().lines() {
        assert!(
            line.starts_with('{') && line.ends_with('}'),
            "not one object: {line}"
        );
        assert!(line.len() <= 1024, "line too long to append atomically");
        assert!(
            !line.chars().any(|c| c.is_uppercase()),
            "an identifier with uppercase reached the ledger: {line}"
        );
        assert!(
            !line.contains(", ") && !line.contains(": "),
            "prose spacing reached the ledger: {line}"
        );
    }
}

#[test]
fn a_ledger_that_cannot_be_written_never_blocks_an_edit() {
    let lab = Lab::new();
    lab.run(&["turn"]);
    lab.run(&["declare", "--touches", "none"]);
    fs::remove_file(lab.root.join(".mkit/ledger.jsonl")).unwrap();
    fs::create_dir(lab.root.join(".mkit/ledger.jsonl")).unwrap();
    assert_eq!(lab.code(&["turn"]), OK);
    assert_eq!(lab.code(&["declare", "--touches", "none"]), OK);
    assert_eq!(lab.code(&["check"]), OK);
}

#[test]
fn rule_text_is_not_copied_into_the_binary() {
    let source =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("src/main.rs")).unwrap();
    for phrase in [
        "Numbers and thresholds",
        "Irreversible deletion",
        "Third-party calls",
        "decision gate",
    ] {
        assert!(
            !source.contains(phrase),
            "rule text {phrase:?} leaked into the binary; rules belong in core/AGENTS.block.md"
        );
    }
}
