---
name: init
description: Cài mkit vào repo hiện tại — tạo thư mục tài liệu, chèn khối hướng dẫn vào AGENTS.md và CLAUDE.md. Dùng một lần cho mỗi dự án.
disable-model-invocation: true
---

Cài mkit vào repo đang mở. Chạy được nhiều lần, lần sau chỉ cập nhật khối
hướng dẫn, không đụng gì khác.

## 1. Kiểm tra

Đang ở trong một repo git chứ. Không phải thì hỏi người dùng có muốn `git init`
không trước khi làm gì thêm — không có git thì mọi checkpoint đều vô nghĩa.

## 2. Tạo chỗ chứa

```
docs/decisions/
docs/active/
docs/done/
```

Đã có thì để nguyên.

## 3. Chèn khối hướng dẫn

Vào `AGENTS.md`: nội dung `core/AGENTS.block.md`.
Vào `CLAUDE.md`: nội dung `core/CLAUDE.block.md`.

Quy tắc chèn, áp cho cả hai file:

- file đã có `<!-- MKIT:BEGIN -->` → thay toàn bộ đoạn giữa `BEGIN` và `END`,
  không đụng chữ nào bên ngoài
- file có sẵn nhưng chưa có mốc → nối khối vào **cuối file**
- file chưa có → tạo mới với tiêu đề `# Project Rules` rồi mới tới khối

Không bao giờ ghi đè cả file. Luật riêng của người dùng nằm ngoài khối và phải
còn nguyên.

## 4. Tạo `spec.md`

Chưa có thì tạo theo `core/templates/spec.md`. Hỏi người dùng một câu để điền
dòng đầu:

> Sản phẩm này làm gì, cho ai? Một câu thôi.

Đã có `spec.md` thì để nguyên, không đụng.

## 5. Báo

Nói ngắn, bằng tiếng người: đã cài xong, từ giờ gõ được bốn lệnh nào, và lệnh
nào nên gõ trước.

Nói thêm một câu quan trọng:

> Lúc nào tôi nói thứ gì bạn không hiểu, gõ `/mkit:ha` — tôi sẽ nói lại kiểu khác.
