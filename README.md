---
type: Reference
title: "Policy_discussion_casualconversation"
description: "政策検討メモと雑談記録を蓄積する個人ナレッジリポジトリの概要と運用手順"
tags:
  - policy
  - notes
  - okf
generated:
  by: opencode
  at: "2026-10-19T12:17:46+09:00"
status: "approved"
---

## 目的

- 政策・技術・社会課題などの検討メモを `notes/` に蓄積する。
- 雑談の中で得た気づきや話題を `chats/` に記録する。
- すべての文書を Open Knowledge Format (OKF) のフロントマターで管理し、
  検索と再利用に耐える形に保つ。

## 構成

| パス | 責務 |
| --- | --- |
| `notes/` | 政策検討メモ。テーマごとに 1 ファイル、命名は `YYYY-MM-DD-topic.md` |
| `chats/` | 雑談記録。命名は `YYYY-MM-DD-topic.md` |
| `templates/` | 新規文書の起点となるテンプレート |
| `AGENTS.md` | エージェントと人間が共通で従う執筆・検証ルール |

## 文書の作り方

1. `templates/note.md` または `templates/chat.md` を複製する。
2. フロントマターの `title`、`description`、`tags` を埋める。
3. `generated.at` は RFC 3339 の明示オフセット付き時刻に更新する。
4. 本文の最初の見出しは `##` から始める。
5. 検証コマンドを実行し、markdownlint の警告を 0 件にしてから push する。

## 検証

```bash
markdownlint -c .markdownlint.jsonc --dot notes/ chats/ templates/ README.md AGENTS.md
```

## 参照

- [Open Knowledge Format (OKF)](https://github.com/GoogleCloudPlatform/knowledge-catalog/tree/main/okf)
- [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339)
- [markdownlint Rules](https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md)
