# 0006 Agent tự lưu mốc, không bao giờ push

Ngày: 2026-08-14

## Trạng thái

Accepted

## Vì sao phải quyết

Cơ chế "dừng giữa chừng để hỏi" chỉ an toàn nếu có đường quay lui. Đường quay lui
là commit. Nhưng người dùng không biết git nên sẽ không bao giờ yêu cầu commit —
mà mặc định thông thường của agent là chỉ commit khi được bảo.

Giữ mặc định đó ở đây nghĩa là người dùng vĩnh viễn không có mốc nào để quay về.

## Quyết định

Agent **tự commit** ở đúng ba thời điểm: trước khi dừng lại hỏi, khi một việc đã
nghiệm thu xong, và trước thao tác khó hoàn tác.

Agent **không bao giờ push**, không force-push, không reset/revert/discard trừ khi
người dùng yêu cầu đúng thứ đó. Lưu mốc là an toàn cục bộ và là việc của agent;
đẩy code ra ngoài là quyết định của người dùng.

Khi người dùng muốn quay lui, liệt kê mốc bằng **mô tả và thời gian**, không bao
giờ hiện hash hay tên nhánh.

## Ràng buộc kỹ thuật

Message commit: tiền tố Conventional Commits, phần mô tả viết bằng ngôn ngữ của
người dùng, tả thứ đổi **với họ** chứ không tả thứ đổi trong code.

```
feat: người dùng đăng nhập được bằng Google
fix: nút Thanh toán không còn đứng im
chore: mốc lưu trước khi hỏi về mức chặn spam
```

Tiền tố là phần duy nhất trong message dành cho máy đọc. Phần còn lại sẽ xuất
hiện nguyên văn trong danh sách "quay về lúc nào", nên phải là tiếng người.

## Đã cân nhắc gì khác

1. **Chờ người dùng yêu cầu commit** — mặc định thông thường, và ở đây tương
   đương không có checkpoint nào.
2. **Hoàn tác sạch trước khi hỏi thay vì commit rồi hỏi** — nghe "tôi vừa xoá hết
   những gì đã làm, chờ bạn trả lời" thì người dùng hoảng và trả lời quấy quá cho
   xong.

## Đánh đổi

Lịch sử git sẽ có nhiều commit checkpoint hơn một repo do người viết. Chấp nhận,
vì với người dùng này danh sách mốc dày là tính năng chứ không phải rác — mỗi mốc
là một chỗ họ quay về được.
