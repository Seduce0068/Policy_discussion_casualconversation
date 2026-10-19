---
type: Note
title: "リポジトリ整備メモ"
description: "政策メモ・雑談リポジトリの初期整備内容と運用上の決定事項の記録"
tags:
  - housekeeping
  - okf
  - markdownlint
generated:
  by: opencode
  at: "2026-10-19T12:17:46+09:00"
status: "approved"
---

## 背景

- 2026-10-19 に本リポジトリを作成し、政策メモと雑談記録の蓄積を開始した。
- 空の状態であったため、文書メタデータ（OKF）と lint 検証を含む執筆規約を
  先に固めた。

## 初期整備内容

- ディレクトリ: `notes/`、`chats/`、`templates/`
- 文書メタデータ: OKF フロントマター（`type`、`title`、`description`、`tags`、
  `generated`、`status`）
- 検証: markdownlint 0.49.1 と `.markdownlint.jsonc`
  （`MD013` は 120 文字、`MD024` は `siblings_only`）

## 運用上の決定事項

- 政策メモは `type: Note` で `notes/` へ、雑談記録は `type: Chat` で
  `chats/` へ置く。
- 本リポジトリで許容する `type` は `Note`、`Chat`、`Reference`。
- 日時は RFC 3339 の明示オフセット付きで記録する。

## 検証

```bash
markdownlint -c .markdownlint.jsonc --dot notes/ chats/ templates/ README.md AGENTS.md
```

- 全ファイルで警告 0 件を確認済み。

## 参照

- [AGENTS.md](../AGENTS.md)
- [README.md](../README.md)
- [Open Knowledge Format (OKF)](https://github.com/GoogleCloudPlatform/knowledge-catalog/tree/main/okf)
