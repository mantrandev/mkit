# 0002 Người dùng mục tiêu là Tier B

Ngày: 2026-08-14

## Trạng thái

Accepted

## Vì sao phải quyết

"Non-tech" trải rộng nhiều bậc năng lực, và mỗi bậc cho ra một sản phẩm khác
hẳn. Không chốt bậc nào thì mọi quyết định sau đều lơ lửng.

## Quyết định

mkit phục vụ người **mở được terminal, dán được lệnh, không đọc được code**.
PM, designer, founder. Họ nhìn lỗi ở mức "đỏ là hỏng".

Họ không đọc được diff, không đọc được log test, không tự biết một thay đổi có
an toàn hay không.

## Ràng buộc kỹ thuật

Mọi tính năng phải chạy được trong một giao diện dòng lệnh hoặc khung chat. Không
xây GUI.

Mọi chuỗi gửi cho người dùng phải qua bộ lọc từ vựng trong mục `Language` của
khối `MKIT` — cấm thuật ngữ, thay bằng hậu quả.

## Đã cân nhắc gì khác

1. **Bậc thấp hơn — không mở nổi terminal.** Cần GUI hoặc web app; mkit không
   còn là kit mà thành công ty phần mềm.
2. **Bậc cao hơn — đọc được code, không tự viết.** `repository-harness` bản gốc
   đã phục vụ họ ổn, mkit không thêm được giá trị đáng kể.

## Đánh đổi

Bỏ qua nhóm người dùng hoàn toàn không dùng terminal, vốn đông hơn nhiều. Đổi
lại mkit vá được vấn đề thật bằng một lớp text, không cần hạ tầng.
