# 0004 Lệnh chọn ý định, không chọn mức an toàn

Ngày: 2026-08-14

## Trạng thái

Accepted

## Vì sao phải quyết

mkit có bốn lệnh sửa file. Nếu mỗi lệnh mang một mức kiểm soát khác nhau thì
người dùng vô tình chọn được mức an toàn cho chính mình — bằng cách gõ nhầm lệnh.

Cặp `plan` và `implement` nguy hiểm nhất: nó bắt người dùng tự phán *"yêu cầu của
tôi đã đủ rõ để bỏ qua bàn bạc chưa?"*, câu mà muốn trả lời được thì phải biết
trước cái mình chưa biết.

## Quyết định

Cổng chốt chạy ở **mọi lệnh có sửa file**. Không lệnh nào tắt được nó.

`plan` và `implement` khác nhau ở **điểm dừng**, không khác mức an toàn: `plan`
chốt xong rồi dừng, `implement` chốt xong rồi làm tiếp. Người dùng phát biểu được
khác biệt này bằng lời của họ — "bàn thôi" và "làm luôn".

Gõ nhầm lệnh chỉ tốn thêm vài câu hỏi, không thủng hàng rào.

## Ràng buộc kỹ thuật

`plan`, `implement`, `fix` đều gọi cùng một checklist trước khi sửa file đầu tiên.
`plan` cam kết không sửa file code nào — phải giữ đúng, vì đó là lệnh an toàn để
người dùng thăm dò.

## Đã cân nhắc gì khác

1. **Một cửa vào duy nhất, tự phân loại bên trong** — đã đề xuất và bị bác. `fix`
   và `continue` là thứ Tier B tự nhận ra được ("nó đang lỗi", "làm tiếp"), gộp
   lại chỉ làm khó thêm.
2. **`implement` thật sự bỏ qua grill cho nhanh** — bán đứng đúng người nó phục
   vụ: người gõ `implement` để đi nhanh chính là người ít khả năng phát hiện
   agent vừa tự bịa chính sách nhất.

## Đánh đổi

Người dùng biết rõ mình muốn gì vẫn phải trả lời vài câu khi việc chạm sáu mục.
Chấp nhận, vì cách duy nhất để bỏ qua là để họ tự đánh giá rủi ro — thứ họ không
làm được.
