# Phân loại câu trả lời của grill-me

Bắt đầu: 2026-08-14
Liên quan `spec.md`: Workflow `grill-me` và `docs/decisions/`

## Muốn gì

Khi dùng `grill-me` trong lúc planning, không cập nhật tất cả câu trả lời vào
`docs/decisions/`. Làm theo cách của `repository-harness`: ghi lựa chọn của task
vào active plan và chỉ nâng lựa chọn lâu dài thành decision riêng.

## Xong gì

- [x] Đối chiếu workflow và template hiện tại của `repository-harness` · 2026-08-14
- [x] Xác định `plan` đang gọi `grill-me` trước khi tạo active plan · 2026-08-14
- [x] Đổi thứ tự và tiêu chí phân loại trong workflow nguồn · 2026-08-14
- [x] Cài working tree vào repo tạm và kiểm tra các file sinh ra · 2026-08-14

## Còn gì

- [ ] Chạy một phiên planning mới và xác nhận câu trả lời riêng của task không tạo decision

## Cách tự kiểm

Trong một repo thử, chạy planning cho một tính năng có hai câu cần chốt: một lựa
chọn chỉ ảnh hưởng cách làm task đó và một lựa chọn thay đổi chính sách chung.
Sau khi trả lời, `docs/active/<task>.md` phải chứa cả hai; `docs/decisions/` chỉ
có thêm lựa chọn thay đổi chính sách chung.

## Quyết định trong việc này

- 2026-08-14: Dùng ranh giới của `repository-harness`: task-local nằm trong active plan; lựa chọn lâu dài mới được nâng thành decision riêng.

## Ghi chú kỹ thuật

Kiểm tra parity giữa `AGENTS.md` và `core/AGENTS.block.md`, giữa hai active
template, rồi kiểm tra bản cài trong thư mục tạm.
