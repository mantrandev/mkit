---
name: implement
description: Làm thật một việc — chốt những gì cần chốt rồi sửa code cho tới khi có thứ người dùng tự bấm thử được. Dùng khi người dùng muốn làm luôn.
disable-model-invocation: true
---

Làm cho tới khi có thứ người dùng tự nhìn thấy được.

Cổng chốt vẫn chạy đủ như `/mkit:plan`. Lệnh này khác ở **điểm dừng**, không
khác ở mức an toàn. Người dùng gõ `implement` thay vì `plan` không có nghĩa là
họ cho phép bỏ qua câu hỏi nào.

## 1. Hiểu đúng chưa

Viết lại mong muốn bằng lời người dùng. Hỏi đúng chưa. Chưa đúng thì sửa, không
đi tiếp.

Nếu `docs/active/` đã có file cho việc này (do `/mkit:plan` tạo trước đó), đọc
nó thay vì hỏi lại từ đầu.

## 2. Cổng chốt

Đọc `docs/decisions/`. Đối chiếu sáu mục. Thiếu quyết định nào thì
`mkit:grill-me` cho mục đó.

Không có mục nào bị chạm thì không hỏi câu nào — sang thẳng bước 3.

## 3. Làm

Vừa làm vừa cập nhật `Xong gì` trong `docs/active/<tên-việc>.md`. Người dùng có
thể đóng máy bất cứ lúc nào; file đó là thứ duy nhất giúp họ quay lại.

**Dừng giữa chừng** khi gặp một trong năm dấu hiệu ở khối `MKIT`. Khi dừng:

1. commit một checkpoint
2. nói đã sửa gì, app còn chạy được không
3. nói cách quay lui
4. rồi mới hỏi

Không bao giờ tự chọn "phương án an toàn nhất" rồi đi tiếp.

## 4. Chuẩn hoàn thành

Viết kịch bản thao tác người dùng tự bấm được, kèm thứ họ phải thấy. Cụ thể tới
mức bấm theo được mà không cần hỏi thêm.

Không báo số test, không báo coverage, không liệt kê file đã sửa.

Viết không nổi kịch bản đó thì chưa xong — nói thẳng như vậy và nói còn thiếu gì.

## 5. Đóng việc

Người dùng xác nhận thấy đúng:

- điền `Cách tự kiểm` vào file trong `docs/active/`
- chuyển file sang `docs/done/`
- đổi dòng tương ứng trong `spec.md` thành `✅ chạy` kèm ngày hôm nay

Người dùng nói không thấy đúng: quay lại **bước 1**, không phải bước 3. Phần lớn
trường hợp là hiểu lệch mong muốn ban đầu, không phải code sai.
