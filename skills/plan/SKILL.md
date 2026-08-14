---
name: plan
description: Bàn về một việc muốn làm, chốt những gì cần chốt, ghi lại — nhưng chưa sửa code. Dùng khi người dùng muốn thăm dò trước, chưa muốn động vào sản phẩm.
disable-model-invocation: true
---

Bàn thôi, không làm. Kết thúc skill này **không có file code nào bị sửa**.

Đây là lệnh an toàn để người dùng thăm dò. Giữ đúng lời hứa đó.

## 1. Hiểu đúng chưa

Viết lại mong muốn bằng đúng lời người dùng, không dịch sang tiếng kỹ thuật.
Đưa họ đọc và hỏi thẳng: đúng chưa.

Sai thì sửa và hỏi lại. Không đi tiếp khi chưa có "đúng rồi" — mọi câu hỏi sau
đó sẽ là câu hỏi cho một bài toán không phải của họ.

## 2. Cổng chốt

Đọc `docs/decisions/` trước. Đối chiếu yêu cầu với sáu mục trong khối `MKIT` ở
`AGENTS.md`.

Chạm mục nào mà chưa có quyết định thì chạy `mkit:grill-me` cho từng mục một.

## 3. Ghi xuống

Tạo `docs/active/<tên-việc>.md` theo `core/templates/active.md`. Điền `Muốn gì`
và `Còn gì`. `Xong gì` để trống.

Thêm dòng tương ứng vào `spec.md` với trạng thái `⬜ chưa làm`, hoặc `⏳ đang làm`
kèm link sang file vừa tạo nếu người dùng nói sẽ làm ngay.

## 4. Dừng

Báo lại ba thứ, ngắn gọn, bằng tiếng người:

- đã hiểu bạn muốn gì
- đã chốt những gì
- muốn làm thật thì gõ `/mkit:implement`

Không tự chuyển sang làm. Người dùng gõ `plan` là đã nói rõ họ chưa muốn.
