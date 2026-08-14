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

**Claude Code** — hai lệnh, không cần mở terminal:

```
/plugin marketplace add mantrandev/mkit
/plugin install mkit@mkit
```

Rồi mở dự án của bạn và gõ `/mkit:init` một lần. Lệnh này cũng cài luôn phần
cho Codex và Pi.

<details>
<summary>Không dùng Claude Code — một lệnh trong terminal</summary>

```bash
curl -fsSL https://raw.githubusercontent.com/mantrandev/mkit/main/install.sh | bash
```

Cài vào thư mục hiện tại; thêm đường dẫn phía sau để cài chỗ khác. Chạy lại chỉ
cập nhật khối hướng dẫn, không đụng gì bạn đã viết.

Cách này cài luật và workflow cho Codex/Pi, không cài slash command — Claude Code
lấy lệnh từ plugin.

</details>

## Ba agent, một bộ luật

| Agent | Đọc gì | Dùng thế nào |
| --- | --- | --- |
| **Claude Code** | `CLAUDE.md` → `AGENTS.md` | Gõ `/mkit:plan`, `/mkit:fix`… |
| **Codex** | `AGENTS.md` | Nói tiếng thường: *"bàn về X"*, *"bị lỗi rồi"* |
| **Pi** | `AGENTS.md` | Nói tiếng thường |

Luật nằm **một chỗ duy nhất** trong `AGENTS.md`. `CLAUDE.md` chỉ có một dòng
`@AGENTS.md` chứ không chép lại — hai bản song song chắc chắn sẽ lệch nhau sau
vài lần sửa, và lúc đó không ai biết bản nào đúng.

Slash command chỉ là lớp tiện cho Claude Code. Codex và Pi tìm đúng workflow qua
bảng ở cuối khối `AGENTS.md`, trỏ vào `.mkit/workflows/`.

## Sáu lệnh

| Lệnh | Làm gì |
| --- | --- |
| `/mkit:init` | Cài chỗ chứa tài liệu, hỏi sản phẩm của bạn làm gì |
| `/mkit:plan` | Bàn một việc, chốt những gì cần chốt — **không sửa code** |
| `/mkit:implement` | Làm thật, tới khi có thứ bạn tự bấm thử được |
| `/mkit:fix` | Sửa lỗi — tái hiện trước, sửa sau |
| `/mkit:continue` | Hôm trước làm tới đâu rồi |
| `/mkit:ha` | Nói lại kiểu khác khi bạn không hiểu |

Trên Codex và Pi thì không gõ lệnh — nói bằng tiếng thường, agent tự tìm đúng
workflow.

`/mkit:ha` là lệnh quan trọng nhất trong sáu cái. Không hiểu mà gật cho qua là
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

## Những thứ bạn không nhìn thấy

Bạn không đọc được code, nên có một loạt thứ hỏng mà bạn **không có cách nào
phát hiện**. mkit bắt agent tự canh chúng:

- **Viết ít nhất có thể.** Không thêm tính năng bạn không xin. Không dựng cấu
  trúc phức tạp cho thứ dùng một lần. 200 dòng mà 50 dòng đủ thì viết lại.
- **Chỉ đụng thứ buộc phải đụng.** Không "dọn dẹp" code xung quanh, không sửa
  format, không đổi thứ đang chạy tốt. Mỗi dòng bị sửa phải truy ngược được về
  yêu cầu của bạn.
- **Không xoá, không reset, không force-push** trừ khi bạn bảo làm đúng thế.
- **Thấy cách đơn giản hơn thì phải nói**, trước khi làm. Bạn không có ai khác
  phản biện hộ.
- **Cái gì chưa làm, chưa chạy, chưa thử thì phải khai ra.** Bạn không tự kiểm
  được, nên im lặng sẽ bị hiểu là đã kiểm hết.

## Đường lùi

Bạn không cần biết git. Agent tự lưu mốc giúp bạn — trước mỗi lần dừng lại hỏi,
và mỗi lần một việc xong.

Muốn quay lại thì nói *"quay về lúc nãy"*. Bạn sẽ được hỏi kiểu này:

```
Quay về lúc nào?
1. Trước khi tôi sửa nút Thanh toán — 10 phút trước
2. Trước khi thêm đăng nhập Google — hôm qua
```

Không có mã băm, không có tên nhánh, không có thuật ngữ. Chọn số là xong.

Agent **không bao giờ tự đẩy code đi đâu** — lưu mốc là việc của nó, còn gửi
code ra ngoài là quyết định của bạn.

## Dựa trên

- [`hoangnb24/repository-harness`](https://github.com/hoangnb24/repository-harness) — cổng thẩm quyền, phân loại công việc, chuẩn hoàn thành, cấu trúc `decision.md`
- [`mattpocock/skills`](https://github.com/mattpocock/skills) — pattern `grilling`

Cả hai đều MIT. Chi tiết kế thừa gì, sửa gì: [`NOTICE`](./NOTICE).

## Giấy phép

MIT
