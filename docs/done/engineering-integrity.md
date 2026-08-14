# Giữ code đúng boundary của từng repo

Started: 2026-08-14
Related `spec.md` entry: Agent rules · Engineering integrity gate

## Desired outcome

MKit phải giữ code sạch và đúng kiến trúc của từng repo, nhưng không ép mọi dự
án dùng Clean Architecture. Repo đã có source được mô tả lại đúng cấu trúc đang
có; repo mới chỉ ghi kiến trúc sau khi tính năng đầu tiên tạo ra boundary thật.

## Completed

- [x] Chốt hai nhánh `mkit:init` cho repo có source và repo mới · 2026-08-14
- [x] Ghi luật kiến trúc theo repo, không áp đặt Clean Architecture · 2026-08-14
- [x] Cập nhật luật dùng chung và các workflow `init`, `plan`, `implement`, `fix`, `continue` · 2026-08-14
- [x] Thêm `docs/architecture.md`, template tương ứng và luồng cài đặt · 2026-08-14
- [x] Kiểm tra installer hai lần liên tiếp trong repo tạm · 2026-08-14
- [x] Chạy `mkit:init` bằng agent mới trên repo có source và repo mới · 2026-08-14
- [x] Chạy lần implement đầu tiên bằng agent mới trên repo trống · 2026-08-14
- [x] Tăng Claude plugin lên `0.1.2` và Codex cachebuster lên `0.1.0+codex.20260814163303` · 2026-08-14

## Remaining

- Không còn.

## Acceptance

Trong repo có source, `mkit:init` giữ nguyên product source, mô tả đúng chiều
phụ thuộc application → domain và ghi lệnh kiểm tra đang hỏng vào `Known gaps`
thay vì sửa ngoài yêu cầu. Trong repo trống, `mkit:init` không tạo source,
layer hay `docs/architecture.md`. Lần implement đầu tiên chỉ tạo một file Node.js
và lúc đó mới ghi kiến trúc một-file; chạy `node greet.js Linh` in đúng
`Hello, Linh!`.

## Task decisions

- 2026-08-14: [Decision 0011](../decisions/0011-project-architecture-is-observed-not-imposed.md) quy định quan sát kiến trúc từng repo thay vì áp đặt một mô hình.
- 2026-08-14: Repo có source được mô tả mà không bị đổi kiến trúc khi init.
- 2026-08-14: Repo mới không có layer hay tài liệu kiến trúc giả; lần implement đầu tiên mới thiết lập cấu trúc.

## Technical notes

Forward-test dùng ba repo tạm độc lập: existing JavaScript module, empty init và
empty first implementation. Không forward-test trên hệ thống thật.
