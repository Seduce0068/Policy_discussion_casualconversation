---
type: Playbook
title: "Policy_discussion_casualconversation 作業手順"
description: "本リポジトリの文書メタデータ・命名・執筆・検証・コミットの運用手順"
tags:
  - playbook
  - okf
  - markdownlint
  - housekeeping
generated:
  by: opencode
  at: "2026-10-19T12:17:46+09:00"
status: "approved"
---

## 目的

- 人でもエージェントでも同じ品質で文書を追加できる手順を固定する。
- Open Knowledge Format (OKF) フロントマターと markdownlint 準拠を必須とする。

## ディレクトリ契約

| パス | 納める文書 |
| --- | --- |
| `notes/` | 政策・技術・社会課題の検討メモ（`type: Note`） |
| `chats/` | 雑談から得た気づきや話題の記録（`type: Chat`） |
| `adr/` | アーキテクチャ決定記録（`type: ADR`） |
| `reports/` | 技術報告書・実現可能性評価などの完成レポート（`type: Report`） |
| `templates/` | 新規文書の起点テンプレート（編集は規約変更時のみ） |

- 上記 5 ディレクトリ以外に文書を新規作成しない。
- `scripts/` は索引生成スクリプトの置き場であり、文書契約の対象外とする。
- リポジトリ直下の運用ファイル（`README.md`、`index.md`、`log.md`、
  本ファイル）はディレクトリ契約の対象外とする。
- `index.md` は各コンテンツディレクトリに置ける予約ファイルであり、
  概念文書として数えない。
- `log.md` はリポジトリ直下にのみ置く。ディレクトリ別の履歴は git に委ねる。
- 根拠資料が外部にある場合、`resource` キーか本文の `参照` セクションで示す。

## 文書メタデータ

- フロントマターの必須キーは `type`、`title`、`description`、`tags`、
  `generated`（`by` と `at` を含む）。
- 日時は RFC 3339 で明示オフセットを必ず含める。
  - 正: `2026-10-19T12:17:46+09:00`
  - 誤: `2026-10-19`（日付のみ）、`2026-10-19T03:17:46Z`（`Z` 接尾）
- `tags` は小文字の kebab-case で 1 つ以上 5 種以内に抑える。
- `type` は `Note`、`Chat`、`Reference`、`ADR`、`Report` を許容する。
  新たな型を追加する場合、先に本セクションで定義してから使う。
  - `ADR` は意思決定とその背景・代替案・結果を記録する。
  - `Report` は調査・評価の結果を構造化して報告する。
- `status` は `draft`、`review`、`approved`、`deprecated` のいずれか。
- タイムスタンプ等の検証は `okf` スキルのチェックリストと markdownlint で行う。

## 運用ファイル

- `index.md` と `log.md` は OKF の予約ファイルであり、
  概念文書のフロントマター規則を適用しない。
- 両ファイルともフロントマターを持たない。
  ルート `index.md` の `okf_version` も使用しない。
- 本文は単一の `#` 見出しから始め、以降を `##`、`###` の階段で構成する。
- `index.md` は配下の文書へのリンクと説明を列挙し、
  リンク先の `description` を転記する。表形式でもよい。
- `index.md` は各コンテンツディレクトリに置く。ルートはディレクトリ一覧、
  各階層は直下の文書一覧を持つ。`templates/` には置かない。
- 各コンテンツディレクトリの `index.md` は生成領域を持つ。
  `<!-- okf-index:begin -->` と `<!-- okf-index:end -->` の間は
  手動編集しない。
- pre-commit フックが `rust-script scripts/okf-index.rs` を実行し、
  生成領域の更新とステージを自動で行う。
- フックは初回のみ `git config core.hooksPath .githooks` で有効化する。
- ルート `index.md` は手動管理とする。
- `log.md` はルートのみに置く。ディレクトリ別の履歴は git に委ねる。
- `log.md` の日付見出しは `## YYYY-MM-DD` とし、新しい順に並べる。
  1 作業 1 行とし、時刻は 24 時間制で記す。
- 作業の詳細は各文書とコミットメッセージを参照し、ログは要約に留める。
- フロントマターを置いた場合、OKF の準拠条件（SPEC §11.3）を満たさない。

## 命名規約

- ファイル名は小文字 kebab-case で、記録時点の日付を先頭に付ける。
  例: `2026-10-19-repository-setup.md`
- `adr/` のファイルは ADR 標準慣例に従い、日付前置の代わりに
  `ADR-XXX-kebab-case.md` 形式を許容する。例: `adr/ADR-001-combini-e-receipt.md`
- 同一トピックの継続は既存ファイルへの追記が基本。新規ファイルに分離する
  場合は旧ファイルの `status` を更新し、相互参照を残す。

## 執筆ルール

- 概念文書の本文の最初の見出しは `##`。フロントマターの `title` が H1 を兼ねる。
- 見出しは `##` → `###` → `####` の階段で使用する。
- 予約ファイル（`index.md` / `log.md`）は例外とし、`#` から始める
  （「運用ファイル」を参照）。
- 見出し・リスト・表・コードブロックの前後に空行を置く。
- 行長は 120 文字以内（`.markdownlint.jsonc` で強制）。
- URL はリンク記法で書き、裸の URL を本文に置かない。
- 既存のメモを勝手に書き換えない。追記は新しい `###` セクションを追加するか、
  `status` を `deprecated` にして新規ファイルへ継承する。

## 検証手順

```bash
markdownlint -c .markdownlint.jsonc --dot notes/ chats/ adr/ reports/ \
  templates/ README.md AGENTS.md index.md log.md
rust-script scripts/okf-index.rs --check
```

- 索引が古い場合、`rust-script scripts/okf-index.rs` で更新してから再実行する。
- コミット前に必ず実行し、警告 0 件を確認する。
- markdownlint コマンドが無い環境では次の代替コマンドを使う。

```bash
npx markdownlint-cli@0.49.1 -c .markdownlint.jsonc --dot notes/ chats/ \
  adr/ reports/ templates/ README.md AGENTS.md index.md log.md
```

## コミット規約

- 1 コミット = 1 論理変更。規約変更と本文執筆を混在させない。
- メッセージは一行要約を先頭に置く。例: `docs: add policy note on X`
- push 後の履歴書き換えをしない。
- 秘密情報（トークン、API キー、住所などの個人情報）を commit しない。

## 参照

- [Open Knowledge Format (OKF)](https://github.com/GoogleCloudPlatform/knowledge-catalog/tree/main/okf)
- [RFC 3339](https://datatracker.ietf.org/doc/html/rfc3339)
- [markdownlint Rules](https://github.com/DavidAnson/markdownlint/blob/main/doc/Rules.md)
