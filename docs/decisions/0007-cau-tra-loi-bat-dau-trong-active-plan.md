# 0007 Câu trả lời bắt đầu trong active plan

Ngày: 2026-08-14

## Trạng thái

Accepted

## Vì sao phải quyết

Luồng `plan` gọi `grill-me` trước khi tạo file trong `docs/active/`. Vì chưa có
chỗ ghi cho task hiện tại, câu trả lời dễ bị biến thành một file riêng trong
`docs/decisions/`, kể cả khi nó chỉ phục vụ việc đang bàn.

## Quyết định

Theo cách phân tầng của `repository-harness`: mọi câu trả lời trong lúc planning
bắt đầu ở active plan của task. Chỉ nâng thành decision riêng khi lựa chọn đó có
hiệu lực với các task tương lai và thay đổi đáng kể product, architecture, data
ownership, security hoặc recovery policy, public compatibility, validation
requirements, hay source-of-truth/default workflow.

Việc một câu hỏi chạm sáu mục của decision gate chỉ quyết định rằng agent phải
hỏi. Nó không tự động biến câu trả lời thành luật dài hạn.

## Ràng buộc kỹ thuật

`plan` phải tạo `docs/active/<task>.md` trước khi gọi `grill-me`.

`grill-me` phải ghi mọi câu trả lời vào `Quyết định trong việc này` trước. Chỉ
khi đủ cả hai điều kiện về hiệu lực tương lai và loại thay đổi lâu dài mới tạo
thêm file trong `docs/decisions/`.

## Đã cân nhắc gì khác

1. **Mỗi câu trả lời tạo một decision riêng** — dễ tìm nhưng làm thư mục chứa
   luật dài hạn bị lẫn với chi tiết của từng task.
2. **Chỉ dùng câu hỏi “task khác tuần sau có dùng lại không”** — quá rộng; nhiều
   tiêu chí nghiệm thu của một tính năng vẫn đúng tuần sau nhưng không đáng trở
   thành luật cho toàn dự án.

## Đánh đổi

Một lựa chọn được nâng thành luật chung sẽ xuất hiện ở active plan và decision
riêng. Chấp nhận sự lặp có chủ đích này để plan giữ đủ bối cảnh, còn các task sau
vẫn tìm được luật dài hạn mà không phải đọc lại plan cũ.
