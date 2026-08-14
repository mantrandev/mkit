# 0005 spec.md là bảng tổng có trạng thái từng dòng

Ngày: 2026-08-14

## Trạng thái

Accepted

## Vì sao phải quyết

`spec.md` là tài liệu duy nhất người dùng thật sự đọc. Câu họ mang tới nó luôn
là *"tôi đã xây được cái gì rồi"*. Nếu nó lẫn giữa thứ đang chạy và thứ mới lên
kế hoạch, họ không có cách nào phân biệt.

## Quyết định

`spec.md` là bảng tổng của cả dự án — gồm cả thứ đã chạy và chưa chạy. Nhưng
**trạng thái khai ở mức từng dòng**, không phải mức cả file:

```
- [x] Đăng nhập bằng email    ✅ chạy · 2026-08-02
- [ ] Đăng nhập Google        ⏳ đang làm · docs/active/google-login.md
- [ ] Quên mật khẩu           ⬜ chưa làm
```

Người đọc thấy trạng thái ngay tại chỗ đọc, không phải nhớ nó thuộc mục nào.

Chỉ dòng `✅ chạy` mới có nghĩa là kiểm được thật, và chỉ được đánh dấu khi việc
đã chuyển sang `docs/done/`.

## Ràng buộc kỹ thuật

Dòng `⏳ đang làm` phải kèm đường dẫn tới file trong `docs/active/`. Không được
để trạng thái treo: việc bị huỷ thì xoá dòng `⏳`, nếu không người đọc tưởng nó
vẫn đang được làm.

## Đã cân nhắc gì khác

1. **Chỉ chứa thứ đã chạy** — người dùng phải mở hai nơi mới ghép được bức tranh,
   và họ sẽ không mở.
2. **Hai mục "Đang có" / "Sắp có" trong cùng file** — chuyển gánh nặng phân biệt
   sang người đọc, mà người đọc là người ít khả năng phân biệt nhất. Đọc lướt,
   nhớ nhầm.

## Đánh đổi

Mỗi tính năng phải ghi vào `spec.md` hai lần — lúc chốt xong thì thêm dòng
`⬜ chưa làm`, lúc xong thì đổi thành `✅ chạy` kèm ngày. Chấp nhận thêm một
thao tác, đổi lấy việc người dùng không bao giờ hứa với khách một tính năng chưa
tồn tại.
