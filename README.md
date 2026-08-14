# mkit

Bộ kit harness workflow cho AI agent, dành cho người **không đọc được code**.

## Vấn đề

Các harness cho AI agent hiện có đều tốt — và đều giả định người dùng là lập
trình viên. Chúng dừng lại đúng lúc để hỏi con người, nhưng hỏi bằng thứ tiếng
mà người không biết code không trả lời được:

> *Add rate limiting* — thiếu quota, trusted key, enforcement topology, response
> contract. Dừng.

Người dùng gõ *"làm sao cho web đừng bị spam"*. Họ đứng hình. Không phải vì lười
— vì họ không có khái niệm để trả lời.

Bỏ luôn câu hỏi đó thì tệ hơn: agent tự chọn một con số, ghi vào code, và người
dùng **không đủ khả năng phát hiện**. Họ chỉ biết khi khách hàng kêu.

## mkit làm gì

mkit không gỡ cổng chặn. mkit đứng ở cổng làm **người phiên dịch hai chiều**.

**Dịch xuống** — câu hỏi kỹ thuật thành hậu quả cảm được:

```
❓ Cần bạn chốt — Chặn người bấm quá nhanh ở mức nào?

A. Chặt tay — 20 lần/phút
   → Được: gần như hết tài khoản rác
   → Mất: khách thật bấm nhanh giờ cao điểm có thể bị chặn nhầm, đợi 1 phút

B. Lỏng tay — 100 lần/phút
   → Được: khách thật gần như không bị chặn nhầm
   → Mất: mỗi ngày vài chục tài khoản rác phải dọn tay

➡️ Tôi nghiêng về A vì dọn rác rất tốn công, còn khách bị chặn nhầm chỉ đợi 1 phút.
```

**Dịch lên** — bằng chứng kỹ thuật thành thao tác tự bấm được:

```
✗ 23 tests passed, coverage 87%
✓ Mở /dang-ky, bấm Gửi 21 lần liên tiếp. Lần thứ 21 phải hiện "Thử lại sau 1 phút".
```

Người dùng vẫn là người quyết định. Chỉ có ngôn ngữ đổi.

## Cài

```bash
curl -fsSL https://raw.githubusercontent.com/mantrandev/MKit/main/install.sh | bash
```

Cài vào thư mục hiện tại. Muốn cài chỗ khác thì thêm đường dẫn:

```bash
curl -fsSL https://raw.githubusercontent.com/mantrandev/MKit/main/install.sh | bash -s -- /duong/dan/du-an
```

Chạy lại lần nữa chỉ cập nhật khối hướng dẫn, không đụng gì bạn đã viết.

## Sáu lệnh

| Lệnh | Làm gì |
| --- | --- |
| `/mkit-init` | Cài chỗ chứa tài liệu, hỏi sản phẩm của bạn làm gì |
| `/mkit-plan` | Bàn một việc, chốt những gì cần chốt — **không sửa code** |
| `/mkit-implement` | Làm thật, tới khi có thứ bạn tự bấm thử được |
| `/mkit-fix` | Sửa lỗi — tái hiện trước, sửa sau |
| `/mkit-continue` | Hôm trước làm tới đâu rồi |
| `/mkit-ha` | Nói lại kiểu khác khi bạn không hiểu |

`/mkit-ha` là lệnh quan trọng nhất trong sáu cái. Không hiểu mà gật cho qua là
cách hỏng phổ biến nhất — lệnh này biến "tôi không hiểu" thành một thứ gõ ra được.

## Bốn tài liệu

| File | Trả lời |
| --- | --- |
| `spec.md` | Sản phẩm làm được gì — bảng tổng, trạng thái từng dòng |
| `docs/decisions/` | Luật áp cho mọi việc sau |
| `docs/active/` | Đang làm gì, tới đâu, còn gì |
| `docs/done/` | Đã làm gì, chứng minh ra sao |

Chỉ `spec.md` là thứ bạn cần đọc. Mỗi dòng tự khai trạng thái:

```markdown
- [x] Đăng nhập bằng email    ✅ chạy · 2026-08-02
- [ ] Đăng nhập Google        ⏳ đang làm · docs/active/google-login.md
- [ ] Quên mật khẩu           ⬜ chưa làm
```

## Cổng chốt

mkit dừng lại hỏi khi việc chạm một trong sáu thứ sau:

**con số/ngưỡng · tiền · dữ liệu cá nhân · xoá không hồi phục · gọi bên thứ ba · phân quyền**

Ngoài sáu thứ đó, agent tự quyết hết — tên biến, chia file, chọn thư viện, dựng
giao diện. Cổng chốt được thiết kế để **hiếm khi nổ**. Đổi màu một cái nút không
tốn câu hỏi nào.

Nguyên tắc phân chia: agent tự quyết khi **sai thì phát hiện được và sửa rẻ**.
Phải hỏi khi **sai thì bạn không phát hiện được**.

## Dựa trên

- [`hoangnb24/repository-harness`](https://github.com/hoangnb24/repository-harness) — cổng thẩm quyền, phân loại công việc, chuẩn hoàn thành, cấu trúc `decision.md`
- [`mattpocock/skills`](https://github.com/mattpocock/skills) — pattern `grilling`

Cả hai đều MIT. Chi tiết kế thừa gì, sửa gì: [`NOTICE`](./NOTICE).

## Giấy phép

MIT
