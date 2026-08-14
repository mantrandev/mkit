# 0003 Cổng chốt dùng checklist cố định

Ngày: 2026-08-14

## Trạng thái

Accepted

## Vì sao phải quyết

Cổng chốt chỉ có giá trị nếu nó nổ đúng lúc. Nổ sai thì sáu bước còn lại của
luồng vô nghĩa. Phải chọn cơ chế phát hiện.

## Quyết định

Sáu mục cố định, đối chiếu từng mục: **con số/ngưỡng · tiền · dữ liệu cá nhân ·
xoá không hồi phục · gọi bên thứ ba · phân quyền**.

Sáu mục này là sàn, không phải trần — trong lúc làm còn năm dấu hiệu dừng thêm.

Cân bằng sai/sót ở Tier B lệch hẳn một bên: sót thì người dùng chỉ biết khi khách
hàng kêu; thừa thì tốn 30 giây.

## Ràng buộc kỹ thuật

Checklist phải kiểm tra được bằng test: yêu cầu chứa "xoá" mà không dừng là fail.

Mỗi mục kèm sẵn cách hỏi bằng tiếng người, nếu không agent sẽ tự chế câu hỏi bằng
thuật ngữ.

## Đã cân nhắc gì khác

1. **Agent tự phán đoán theo nguyên tắc bằng lời** — cách upstream đang làm. Chạy
   tốt ở đó vì dev tự phát hiện được khi agent bịa. Tier B thì không, nên bê
   nguyên sang là đẩy rủi ro cho người ít khả năng gánh nhất.
2. **Mặc định luôn hỏi, trừ danh sách an toàn** — chết vì nhiễu. Hỏi cả việc hiển
   nhiên thì người dùng bắt đầu gật bừa.

Ghi nhận ngược chiều: decision 0019 của repository-harness viết *"Sensitive
terminology alone is not an automatic approval gate when expected behavior is
explicit"* — tức tác giả đã thử và bác cách bắt theo từ khoá. Bối cảnh khác:
người dùng của họ nói được "expected behavior explicit", Tier B thì gần như
không bao giờ.

## Đánh đổi

Danh sách cố định sẽ bỏ sót loại chính sách không nằm trong sáu mục. Năm dấu
hiệu dừng giữa chừng bù phần đó, nhưng không bù hết. Chấp nhận, vì recall cao và
kiểm tra được quan trọng hơn bao phủ hoàn hảo.
