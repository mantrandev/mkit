# Classify answers from grill-me

Started: 2026-08-14
Related `spec.md` entry: `grill-me` workflow and `docs/decisions/`

## Desired outcome

`docs/active/` chỉ chứa công việc thực sự đang diễn ra và cần tiếp tục qua cuộc
trò chuyện hiện tại. Lựa chọn đã chốt cập nhật `spec.md` khi nó mô tả sản phẩm
hiện tại; chỉ tạo decision khi cần giữ một lý do hoặc luật lâu dài.

## Completed

- [x] So sánh workflow và template hiện tại của `repository-harness` · 2026-08-14
- [x] Phát hiện `plan` từng gọi `grill-me` trước khi có chỗ theo dõi công việc · 2026-08-14
- [x] Sửa thứ tự workflow và tiêu chí phân loại câu trả lời · 2026-08-14
- [x] Cài source hiện tại vào repo tạm và kiểm tra file được tạo · 2026-08-14
- [x] Thay mô hình mọi câu trả lời đều vào active bằng work record thích ứng · 2026-08-14
- [x] Ghi ranh giới tài liệu mới trong decision 0010 · 2026-08-14
- [x] Xác nhận planning bounded không tạo active record · 2026-08-14
- [x] Xác nhận product truth đã chốt được cập nhật vào `spec.md` · 2026-08-14
- [x] Xác nhận chỉ lasting rationale mới tạo decision record · 2026-08-14
- [x] Xác nhận heading và status label cố định vẫn giữ tiếng Anh · 2026-08-14

## Remaining

- Không còn.

## Acceptance

Trong repo thử nghiệm, agent đã bàn một thay đổi bounded qua nhiều câu hỏi. Khi
chưa chốt, agent không tạo file. Sau khi chốt, `docs/active/` vẫn trống,
`spec.md` chứa product truth với status `⬜ not started`, và decision mới chỉ giữ
lý do cùng ràng buộc lâu dài.

## Task decisions

- 2026-08-14: [Decision 0010](../decisions/0010-active-records-track-durable-work.md) thay thế rule mọi câu trả lời đều bắt đầu trong active plan.
- 2026-08-14: `spec.md` giữ product truth hiện tại; decision giữ lasting rationale; active record chỉ giữ công việc đang diễn ra cần bộ nhớ lâu dài.

## Technical notes

Đã kiểm tra parity giữa `AGENTS.md` và `core/AGENTS.block.md`, giữa hai active
template, bản cài lặp lại, source workflow đã cài và hành vi của agent mới trong
repo tạm.
