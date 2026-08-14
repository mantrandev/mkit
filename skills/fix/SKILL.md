---
name: fix
description: Sửa một lỗi — tái hiện trước, sửa sau, nghiệm thu bằng chính các bước tái hiện. Dùng khi người dùng báo có thứ đang hỏng.
disable-model-invocation: true
---

Luật duy nhất không được phá:

> **Chưa tái hiện được thì không sửa.**

Sửa khi chưa thấy lỗi tận mắt là đoán. Người dùng không đủ khả năng phát hiện
bạn vừa đoán, nên họ sẽ tin là đã xong.

## 1. Lấy các bước

Hỏi người dùng làm gì để thấy nó hỏng. Hỏi bằng thao tác, không hỏi bằng mô tả
kỹ thuật:

> Bạn bấm những gì để nó hỏng? Kể tôi nghe từng bước, kể cả bước nhỏ nhất.

Nếu họ kể thiếu, hỏi tiếp cho tới khi có đủ một chuỗi thao tác chạy lại được.
Không đoán hộ.

## 2. Tự làm theo

Làm đúng các bước đó. Có ba kết quả:

**Thấy hỏng** → tái hiện được, đi tiếp.

**Không thấy hỏng** → không sửa. Nói thẳng, rồi hỏi thứ giúp thu hẹp: máy nào,
trình duyệt nào, tài khoản nào, lúc nào thì bị. Ghi các bước đã thử vào
`docs/active/` để lần sau không thử lại từ đầu.

**Không chạy được app** → đó là vấn đề khác và lớn hơn. Xử lý nó trước, nói rõ
với người dùng là đang xử lý chuyện khác.

## 3. Cổng chốt

Sửa lỗi vẫn qua cổng chốt. Rất nhiều lỗi khi sửa lại đẻ ra chính sách mới —
"chặn bao nhiêu lần", "giữ dữ liệu cũ hay xoá", "báo lỗi ra sao". Chạm sáu mục
thì `mkit:grill-me`.

## 4. Sửa

Sửa nhỏ nhất đủ hết lỗi. Không dọn code xung quanh, không đổi thứ không liên quan.

Người dùng không đọc được diff nên không thấy bạn vừa đổi thêm gì. Mọi thay đổi
ngoài phạm vi đều là rủi ro họ không nhìn thấy.

## 5. Nghiệm thu

Đưa lại **đúng các bước ở bước 1** và nói phải thấy gì:

> Làm lại đúng các bước lúc nãy — mở giỏ hàng, bấm Thanh toán, chọn Momo. Lần
> này phải sang được trang thanh toán thay vì đứng im.

Không cần nghĩ kịch bản mới. Người dùng đã tự làm nó một lần rồi.

## 6. Đóng

Xác nhận hết lỗi thì chuyển file sang `docs/done/`, ghi lại các bước tái hiện
trong đó. Lỗi cũ tái phát sau này sẽ cần đúng thông tin đó.

`spec.md` chỉ sửa nếu lỗi này làm một dòng đang ghi `✅ chạy` thật ra không chạy.
