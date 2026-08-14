# 0001 Chưng cất thay vì fork repository-harness

Ngày: 2026-08-14

## Trạng thái

Accepted

## Vì sao phải quyết

Ý tưởng "harness cho AI agent" đã có sẵn bản triển khai tốt là
`hoangnb24/repository-harness`. Nếu mkit chỉ là bản chép lại thì không đáng tồn
tại. Phải xác định mkit khác ở chỗ nào, và khác đủ nhiều để không nên fork.

## Quyết định

mkit là repo độc lập, lấy **ý tưởng** chứ không lấy mã nguồn: cổng thẩm quyền,
phân loại công việc, chuẩn hoàn thành, cấu trúc `decision.md`.

Khác biệt duy nhất nhưng đủ lớn: repository-harness giả định người dùng đọc được
code. mkit thì không. Vì khác biệt nằm ở **câu chữ mà agent đọc đầu tiên**, nó
không phủ lên được — phải viết lại chính những câu đó.

## Ràng buộc kỹ thuật

Không phụ thuộc repository-harness ở bất kỳ mức nào. Không Rust, không SQLite.
Toàn bộ mkit là markdown và skill.

Giữ nguyên bản quyền MIT của cả `hoangnb24/repository-harness` và
`mattpocock/skills` trong `NOTICE`, ghi rõ kế thừa gì và sửa gì.

## Đã cân nhắc gì khác

1. **Fork Rust của repository-harness** — thừa kế 21k dòng Rust của phần mà chính
   tác giả đã đóng băng ở decision 0022 và tuyên bố không phải luồng mặc định.
2. **Skill pack phủ lên bản cài mặc định của repository-harness** — nhưng lớp
   dịch phải sửa chính `AGENTS.md` của core, không chen bên cạnh được. Thêm nữa
   upstream ra 4 decision trong 4 ngày, phủ lên nền đang dịch chuyển thì mất thời
   gian đuổi theo thay vì xây.

## Đánh đổi

mkit nhỏ hơn nhiều so với "làm lại repository-harness". Sau khi upstream đã tự
bỏ SQLite, risk lane và scoring, khoảng cách còn lại gần như chỉ là lớp dịch
ngôn ngữ. Chấp nhận phạm vi hẹp, đổi lấy wedge sạch và không phải bảo trì mã
nguồn của người khác.
