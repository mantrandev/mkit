# mkit

Bộ kit harness workflow cho AI agent, dành cho người không đọc được code.

Mỗi dòng dưới đây tự khai trạng thái. Chỉ dòng `✅ chạy` là thứ đã kiểm thật —
có bằng chứng chạy được, không phải viết xong là tính.

## Luật cho agent

- [x] Cổng chốt 6 mục — dừng khi chạm con số/tiền/dữ liệu cá nhân/xoá/bên thứ ba/phân quyền — ✅ chạy · 2026-08-14
- [x] 5 dấu hiệu dừng giữa chừng — ✅ chạy · 2026-08-14
- [x] Vòng lặp làm rõ yêu cầu, thoát khi viết được kịch bản nghiệm thu — ✅ chạy · 2026-08-14
- [x] Chuẩn hoàn thành bằng thao tác người dùng tự bấm, cấm báo số test — ✅ chạy · 2026-08-14
- [x] Luật tiết chế — viết ít nhất, chỉ đụng thứ buộc phải đụng — ✅ chạy · 2026-08-14
- [x] Luật lưu mốc — tự commit ở 3 thời điểm, không bao giờ push — ✅ chạy · 2026-08-14
- [x] Hai tầng ngôn ngữ — hướng dẫn tiếng Anh, nói với người dùng tiếng Việt — ✅ chạy · 2026-08-14

## Lệnh

- [x] `plan` — bàn, không sửa code — ✅ chạy · 2026-08-14
- [x] `implement` — làm tới khi có thứ bấm thử được — ✅ chạy · 2026-08-14
- [x] `fix` — chưa tái hiện được thì không sửa — ✅ chạy · 2026-08-14
- [x] `continue` — hôm trước làm tới đâu — ✅ chạy · 2026-08-14
- [x] `grill-me` — hỏi 1 câu/lần, mỗi lựa chọn nêu được gì mất gì — ✅ chạy · 2026-08-14
- [x] `ha` — nói lại kiểu khác — ✅ chạy · 2026-08-14
- [x] `init` — cài vào repo đích — ✅ chạy · 2026-08-14

## Tài liệu sinh ra trong repo đích

- [x] `spec.md` — bảng tổng, trạng thái từng dòng — ✅ chạy · 2026-08-14
- [x] `docs/decisions/` — luật liên-task, có `Superseded` — ✅ chạy · 2026-08-14
- [x] `docs/active/` · `docs/done/` — ✅ chạy · 2026-08-14

## Cài đặt

- [x] `install.sh` — chèn khối có mốc, chạy lại không đẻ khối trùng — ✅ chạy · 2026-08-14
- [ ] Cài qua `/plugin marketplace add` — ⏳ chưa kiểm được, repo còn private
- [ ] Chạy thật trên Codex — ⬜ chưa thử
- [ ] Chạy thật trên Pi — ⬜ chưa thử
- [ ] Chạy thật với một người dùng non-tech — ⬜ chưa thử

## Chưa làm

- [ ] Hook chặn lệnh git nguy hiểm (từ `git-guardrails-claude-code`) — ⬜ chưa làm
- [ ] `handoff` — nén hội thoại thành `docs/active/` — ⬜ chưa làm
- [ ] `to-questionnaire` — câu vượt thẩm quyền thì gửi người khác trả lời — ⬜ chưa làm
- [ ] Vòng lặp preview/screenshot để agent tự nhìn được kết quả — ⬜ chưa làm
