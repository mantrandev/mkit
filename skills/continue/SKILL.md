---
name: continue
description: Xem đang làm dở những gì và làm tiếp. Dùng khi người dùng mở phiên mới và không nhớ hôm trước làm tới đâu.
disable-model-invocation: true
---

Trả lời đúng một câu hỏi: **hôm trước tôi đang làm gì, tới đâu rồi.**

## 1. Đọc

Đọc mọi file trong `docs/active/`. Với mỗi file lấy: tên việc, `Muốn gì`, số
bước trong `Xong gì` và `Còn gì`, ngày bắt đầu.

Không có file nào thì đọc `spec.md` xem có dòng `⏳ đang làm` nào bị bỏ lại
không — nếu có thì `docs/active/` đã bị xoá nhầm, nói cho người dùng biết.

## 2. Báo cáo

Nói bằng tiếng người, không liệt kê tên file:

> Bạn đang dở 2 việc:
>
> **1. Thêm đăng nhập Google** — bắt đầu 11/08, xong 3/5 bước.
>    Còn lại: lấy khoá từ Google, thử đăng nhập thật.
>
> **2. Sửa trang giỏ hàng** — bắt đầu hôm qua, mới ghi mong muốn, chưa làm gì.
>
> Tiếp cái nào?

Một việc thì hỏi thẳng: tiếp hay không.

Không việc nào thì nói vậy, và gợi ý `/mkit:plan` hoặc `/mkit:implement`.

## 3. Làm tiếp

Người dùng chọn xong, đọc lại toàn bộ file của việc đó, gồm cả
`Quyết định trong việc này`, rồi chạy tiếp theo `/mkit:implement` từ bước 3.

Không hỏi lại những gì đã ghi trong file. Đó là lý do file đó tồn tại.

## 4. Việc bị bỏ quá lâu

Việc bắt đầu quá hai tuần trước mà không động tới, hỏi trước khi làm tiếp:

> Việc này bỏ dở từ 2 tuần trước. Trong lúc đó sản phẩm đã đổi vài chỗ. Bạn còn
> muốn làm không, hay bỏ luôn?

Bỏ thì chuyển file sang `docs/done/` với ghi chú là đã huỷ, và xoá dòng
`⏳ đang làm` tương ứng trong `spec.md`. Không để trạng thái treo — người dùng
đọc `spec.md` sẽ tưởng nó đang được làm.
