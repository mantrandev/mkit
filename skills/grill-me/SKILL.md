---
name: grill-me
description: Lấy quyết định ra khỏi đầu người không đọc được code. Hỏi từng câu một, mỗi lựa chọn nêu rõ được gì mất gì, rồi ghi câu trả lời xuống thành luật. Dùng khi cổng chốt dừng lại vì thiếu quyết định của người dùng.
---

Bản fork của `grilling` (Matt Pocock, MIT). Giữ nguyên cây thiết kế và nguyên
tắc "tìm dữ kiện là việc của agent, quyết định là việc của người dùng". Khác ở
hai chỗ, vì người dùng ở đây không đọc được code.

## Hai luật không được vi phạm

**Một câu mỗi lần.** Bản gốc hỏi cả frontier trong một vòng. Ở đây không. Hỏi
một câu, đợi trả lời, rồi mới tính câu tiếp theo. Người dùng nhận bốn câu cùng
lúc sẽ trả lời qua loa ba câu cuối.

**Mỗi lựa chọn phải nêu hậu quả cụ thể.** Không có hậu quả thì người dùng gật
theo gợi ý mà không hiểu mình vừa chọn gì. Đó là quyết định giả — trên giấy là
của họ, thực tế là của bạn.

## Trước khi hỏi

Tìm hết dữ kiện tự tìm được. Đọc code, đọc `docs/decisions/`, chạy thử, đo đạc.
Không bao giờ hỏi người dùng thứ bạn tra được.

Đọc `docs/decisions/` trước tiên. Câu hỏi đã có quyết định thì không hỏi lại —
đó là lý do các file đó tồn tại.

## Định dạng câu hỏi

```
❓ **Cần bạn chốt** — <câu hỏi, một dòng, không thuật ngữ>

<Một đoạn ngắn: vì sao tôi không được tự quyết chuyện này.>

**A.** <lựa chọn>
   → Được: <điều tốt, cụ thể>
   → Mất: <điều đánh đổi, cụ thể>

**B.** <lựa chọn>
   → Được: <điều tốt, cụ thể>
   → Mất: <điều đánh đổi, cụ thể>

➡️ **Tôi nghiêng về A** vì <lý do, nói bằng hậu quả>.

Không hiểu chỗ nào thì gõ `/mkit:ha`.
```

Hai tới ba lựa chọn. Bốn là quá nhiều.

"Được" và "Mất" phải là thứ người dùng hình dung được: khách hàng gặp gì, mất
bao lâu, tốn bao nhiêu tiền, hỏng thì cứu được không. Không phải "hiệu năng tốt
hơn" hay "dễ bảo trì".

## Ví dụ

```
❓ **Cần bạn chốt** — Chặn người bấm quá nhanh ở mức nào?

Trang đăng ký đang bị bấm liên tục để tạo tài khoản rác. Chặn được, nhưng chặn
chặt quá thì khách thật cũng bị chặn nhầm. Tôi không tự chọn mức này được.

**A.** Chặt tay — mỗi người 20 lần mỗi phút
   → Được: gần như hết tài khoản rác
   → Mất: khách thật bấm nhanh vào giờ cao điểm có thể bị chặn nhầm, phải đợi
     1 phút mới thử lại được

**B.** Lỏng tay — mỗi người 100 lần mỗi phút
   → Được: khách thật gần như không bao giờ bị chặn nhầm
   → Mất: vẫn lọt tài khoản rác, mỗi ngày khoảng vài chục cái phải dọn tay

➡️ **Tôi nghiêng về A** vì tài khoản rác dọn tay rất tốn công, còn khách bị chặn
nhầm chỉ cần đợi 1 phút và vẫn dùng được.

Không hiểu chỗ nào thì gõ `/mkit:ha`.
```

## Sau khi người dùng chốt

Phân loại bằng đúng một câu hỏi, tự trả lời, không hỏi người dùng:

> Tuần sau có việc khác chạm cùng chỗ này — câu trả lời này còn đúng không?

**Còn đúng** → tạo file mới trong `docs/decisions/` theo `core/templates/decision.md`.
Đánh số tiếp theo số lớn nhất đang có. Ghi cả hai mục: `Quyết định` bằng tiếng
người, `Ràng buộc kỹ thuật` chính xác để thi hành. Nếu quyết định này thay thế
một quyết định cũ, sửa trạng thái file cũ thành `Superseded bởi NNNN` — không
xoá, không sửa nội dung, không chuyển thư mục.

**Không** → ghi vào mục `Quyết định trong việc này` của `docs/active/<task>.md`.

Rồi báo lại đúng một dòng:

> Đã ghi thành luật chung — mọi việc sau sẽ theo.

hoặc

> Chỉ áp cho việc này thôi.

Dòng này là chỗ duy nhất người dùng có cơ hội nói "không, đừng áp mãi".

## Khi nào dừng

Dừng khi không còn mục nào trong cổng chốt bị chạm mà chưa có quyết định. Không
hỏi thêm cho đủ bộ. Mỗi câu hỏi thừa làm người dùng bớt đọc kỹ câu sau.

Nếu phải hỏi quá năm câu cho một việc, dừng lại và nói thẳng: việc này lớn hơn
vẻ ngoài của nó, nên tách nhỏ ra làm từng phần.

## Khi người dùng không có thẩm quyền

Có câu người dùng thật sự không được quyền quyết — giá bán, chính sách hoàn
tiền, điều khoản pháp lý. Đừng ép họ chọn. Nói rõ:

> Câu này cần người quyết định về <chuyện gì>. Bạn hỏi giúp rồi quay lại đây, hoặc
> tôi làm phần khác trước và để trống chỗ này.
