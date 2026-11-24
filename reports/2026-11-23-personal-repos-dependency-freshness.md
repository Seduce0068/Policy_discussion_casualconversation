---
type: Report
title: "個人リポジトリの依存・ライブラリ・言語バージョン最新性チェック"
description: "ACCOUNT 配下 31 リポジトリ（オリジナル 27 + fork 4）のマニフェストを一次レジストリと照合し、更新候補と優先度を整理した"
tags:
  - dependency
  - audit
  - rust
  - npm
  - ci
generated:
  by: lead
  at: "2026-11-23T21:38:41+09:00"
status: "draft"
---

## 概要

`github.com/ACCOUNT/*` の 31 リポジトリ（オリジナル 27 + fork 4）を対象に、
依存ライブラリ・言語/ツールチェーン・CI アクションのピン留めが最新かを静的調査した。
初回確認は `2026-11-23T21:24:13+09:00` 以降、fork の追加調査は
`2026-11-23T23:18:47+09:00` 時点である。

結論は次のとおり。

- Rust 安定版の最新は 1.100.0 で、複数リポの MSRV/CI ピンは 1.85〜1.95 と古い。
- 破壊的変更を伴う更新候補が 10 件以上あり、特に ACP 0.11.1→5.5.1、rmcp 0.3→3.11、
  serde_yaml の保守終了は計画的な移行が必要。
- 低リスクな範囲内更新（tokio、clap、walkdir、playwright など）は比較的多い。
- CI は可変タグ参照とピン不整合が混在し、共通ワークフローの参照先も統一が必要。

## 調査範囲と判定基準

### 対象

- ルートおよび入れ子の `Cargo.toml` / `Cargo.lock` / `package.json` / `package-lock.json`
- `rust-toolchain.toml`、`Cargo.toml` の `rust-version` / `edition`
- `.github/workflows/*` の `uses:` ピンと `node-version` / `rust_toolchain`
- `.pre-commit-config.yaml` の `rev` ピン

### 判定基準

- 宣言バージョンではなく、ロックファイル上の解決バージョンと最新を比較した。
- `^` / `~` / `>=` の範囲指定は「範囲内なら更新可能」として扱い、固定指定は手動更新が必要と分類した。
- メジャー差がある場合は破壊的変更の可能性を明記し、即時更新を推奨しない。
- 最新値は crates.io API、npm registry、Node 配布 JSON、GitHub Releases/Tags から取得した。
- 子エージェントの主張はすべて親が一次情報で再取得し、２件を修正した（「R4 での修正」を参照）。

### 除外・未確認

- 旧ユーザー名 `OLD_ACCOUNT` は 2026-11-22 に `ACCOUNT` へリネーム済みで、
  GitHub API 上は 404（ユーザー自体が存在しない）。旧名配下に独立したリポは存在しない。
- 認証ユーザー視点で見つかった `opencode` / `OpenCodeRust` / `UserScripts` / `ZenzaWatch` の
  4 リポは fork だが、ユーザー指示により個人リポとして追加調査した（後述）。
- 実際の `cargo update` / `npm update` / ビルド / テストは実施していない。
  本レポートは静的調査であり、更新後の互換性は未検証である。
- 推移依存を全件網羅していない。直接依存と明示ピンを優先した。

## 基準となった最新バージョン

| 項目 | 最新 | 取得元 |
| --- | --- | --- |
| Rust 安定版 | 1.100.0（2026-11-12） | [GitHub Releases](https://api.github.com/repos/rust-lang/rust/releases/latest) |
| Node.js | v26.13.1（2026-11-10、LTS Lithium）npm 11.20.0 | [nodejs.org](https://nodejs.org/dist/index.json) |
| TypeScript | 7.0.2 | [npm](https://registry.npmjs.org/typescript/latest) |
| playwright / playwright-core | 1.65.1 | [npm](https://registry.npmjs.org/playwright/latest) |
| WebdriverIO 系 | 10.2.0 | [npm](https://registry.npmjs.org/@wdio%2Fcli/latest) |
| @types/node | 26.6.6 | [npm](https://registry.npmjs.org/@types%2Fnode/latest) |
| markdownlint-cli2 | 0.24.2 | [npm](https://registry.npmjs.org/markdownlint-cli2/latest) |
| @deepseek-ai/dsh | 0.2.2-rc.5 | [npm](https://registry.npmjs.org/@deepseek-ai%2Fdsh/latest) |
| actions/checkout | v7.0.1 | [GitHub Releases](https://api.github.com/repos/actions/checkout/releases/latest) |
| actions/setup-node | v7.1.0 | [GitHub Releases](https://api.github.com/repos/actions/setup-node/releases/latest) |
| taiki-e/install-action | v2.90.8（SHA `fe3acb53…`） | [GitHub Tag](https://api.github.com/repos/taiki-e/install-action/git/ref/tags/v2.90.8) |
| gitleaks | v8.30.1 | [GitHub Releases](https://api.github.com/repos/gitleaks/gitleaks/releases/latest) |
| cargo-deny | 0.22.0 | [GitHub Releases](https://api.github.com/repos/EmbarkStudios/cargo-deny/releases/latest) |
| Betterleaks | v1.10.1（v2 は RC 開発中） | [GitHub Releases](https://api.github.com/repos/betterleaks/betterleaks/releases/latest) |

主要クレートの最新版は `cargo search` / `cargo info` でも確認した。
例: `notify` 9.0.0、`rmcp` 3.11.0、`sqlx` 0.9.1、`axum` 0.8.9、
`agent-client-protocol` 5.5.1、`gpui-kit` 0.10.0、`tokio` 1.54.1、`comrak` 1.2.0、
`socket2` 0.6.5、`clap` 4.6.9、`walkdir` 2.5.1、`async-openai` 0.43.0、
`secret-service` 6.0.0、`gpui` 0.2.2、`chrono` 0.4.45、`fs2` 0.4.3。

## 更新候補（優先度別）

### P1: 移行計画が必要な更新

| リポジトリ | 依存 | 現状 | 最新 | 備考 |
| --- | --- | --- | --- | --- |
| dsh-gpui-client-poc | agent-client-protocol | `=0.11.1` | 5.5.1 | 5 メジャー差。ACP v5 の移行ガイド確認が必要 |
| jarvis/rust | rmcp | 0.3 / lock 0.3.2 | 3.11.0 | 0.x SDK。server/client と transport feature の差分を段階確認 |
| gpui-task-manager, gpui-file-explorer, bitwarden-gpui-poc, dsh-gpui-client-poc | gpui-kit | `=0.9.1` | 0.10.0 | UI API の差分レビュー後に 4 箇所を同時更新 |
| howdy-notifier | notify | 6.1 / lock 6.1.1 | 9.0.0 | 9.0.0 は rustc 1.91 以上を要求 |
| openrustbooks | comrak | 0.59 / lock 0.59.0 | 1.2.0 | 1.0 の移行差分を先に確認 |
| openrustbooks | WebdriverIO 系 | 9.31.x | 10.2.0 | `@wdio/tauri-service` の peer が `^9.0.0` でブロッカー |
| openrustbooks | typescript | 5.9.3 | 7.0.2 | 5→6→7 の段階更新と `tsc -p` の検証が必要 |
| grobalOpencodeSettings/okf_common | serde_yaml | 0.9 | 0.9.34+deprecated | 保守終了。後継クライアントへの乗り換えを検討 |
| gpui-task-manager/app | secret-service | `=5.2.0` | 6.0.0 | crypto feature 名の変更を確認 |
| system-diagnostics | socket2 | 0.5 / lock 0.5.10 | 0.6.5 | 0.6.4 は推移依存として既に lock に同居 |
| jarvis/rust | axum | 0.7 / lock 0.7.9 | 0.8.9 | Router/State/serve に破壊的変更。現状維持判断のコメントあり |
| openrustbooks | @types/node | 24.10.1 | 26.6.6 | 型定義のメジャー差。実行 Node との整合を確認 |

### P2: 同一メジャー内の更新（比較的低リスク）

| リポジトリ | 依存 | 現状 | 最新 |
| --- | --- | --- | --- |
| cordis-rs, dsh-gpui-client-poc | tokio | `=1.48.0` | 1.54.1 |
| dsh-gpui-client-poc | tokio-util | `=0.7.16` | 0.7.20 |
| system-diagnostics | clap | lock 4.6.1 | 4.6.9 |
| KubuntuStrageCleanRustscript | walkdir | 2.5 / lock 2.5.0 | 2.5.1 |
| jarvis/rust | cedar-policy | 4.13 / lock 4.13.0 | 4.15.0 |
| jarvis/rust | sqlx | 0.8 / lock 0.8.6 | 0.9.1 |
| openrustbooks | async-openai | 0.42 / lock 0.42.2 | 0.43.0 |
| userscript | playwright | 1.61.1（lock も同値） | 1.65.1 |
| dsh-gitbash-win | playwright-core（CI 一時導入） | 1.56.1 | 1.65.1 |
| system_all_update | chrono / fs2（rust-script 内） | 0.4 / 0.4 | 0.4.45 / 0.4.3 |

`gpui-task-manager` の `serde = "1"` / `serde_json = "1"` は範囲指定であり、
最新 1.0.229 を包含するため更新不要。`gpui = "=0.2.2"` も crates.io 最新と一致した。

### P3: CI・ツールチェーン・横断課題

- **Rust**: 最新 1.100.0 に対し、gpui-task-manager 1.95（sysinfo 0.40.3 要件で意図的固定）、
  gpui-file-explorer 1.92、cordis-rs 1.85、dsh-gpui-client-poc 1.88、
  openrustbooks CI 1.95.0、bitwarden-gpui-poc CI 1.95 と古い。
  jarvis の `rust-version = "1.100"` は最新と一致している。
- **共通ゲート**: `ACCOUNT/workflows` の `rust-gates.yml` は `rust_toolchain` 既定が可変の `stable` で、
  各リポの MSRV を固定検証していない。
- **Node**: userscript CI は 22、dsh-gitbash-win CI は 24、openIllustration は `engines.node >=22`。
  最新安定版は 26.13.1（LTS Lithium）。22 のサポート終了時期は今回未確認。
- **可変タグ**: userscript は `actions/checkout@v7` / `setup-node@v7`、
  knowledge は `checkout@v4` / `cache@v4` / `dtolnay/rust-toolchain@stable` を使用。
  SHA 固定と実バージョンのコメント併記を推奨する。
- **共通ワークフロー参照**: `v1` は移動タグで、gpui-task-manager は SHA 固定、
  jarvis などは `@v1` 参照。タグ付け替え時に全呼び出し側が同時に変わる。
- **ピン不整合**: `taiki-e/install-action` は v2.90.3 / v2.90.6 に分かれ、最新は v2.90.8。
  `cargo-deny` は 0.20.2 / 0.21.0 に分かれ、最新は 0.22.0。
- **参照先アカウント**: system-diagnostics は `OLD_ACCOUNT/workflows@...v1` を参照。
  旧アカウントは到達可能で v1 の SHA も同一だが、共通指示に従い
  `ACCOUNT/workflows` 参照へ統一するのが安全。
- **シークレットスキャン**: cordis-rs と dsh-gitbash-win は gitleaks、
  他は Betterleaks。共通指示は Betterleaks 主軸のため、方針適用か例外文書化が必要。
- **npm**: 4 リポ（`userscript`、`dsh-gitbash-win`、`openrustbooks`、`openIllustration`）に
  `packageManager` 宣言がなく、npm 版が CI 環境依存で変わり得る。
- **DSH 互換**: dsh-gitbash-win は rc.1〜rc.3 のみ検証済みで、
  peer 範囲は最新 rc.5 を含むが互換性は未検証。

## リポ別サマリー

### 更新候補あり（17 リポ）

| リポジトリ | 代表的な更新候補 |
| --- | --- |
| gpui-task-manager | gpui-kit 0.10.0、secret-service 6.0.0、Rust CI 1.95 |
| gpui-file-explorer | gpui-kit 0.10.0（app/poc）、MSRV 1.92、install-action ピン |
| cordis-rs | tokio 1.54.1、gitleaks v8.30.1、MSRV 1.85 |
| dsh-gpui-client-poc | ACP 5.5.1、tokio 1.54.1、gpui-kit 0.10.0、toolchain 1.88 |
| openrustbooks | comrak 1.2.0、WDIO 10.2.0、TS 7.0.2、Rust CI 1.95.0 |
| openIllustration | cargo-deny 0.22.0（npm 依存は最新） |
| bitwarden-gpui-poc | gpui-kit 0.10.0、Rust CI 1.95 |
| system-diagnostics | socket2 0.6.5、clap 4.6.9、共通ワークフロー参照先 |
| howdy-notifier | notify 9.0.0 |
| system_all_update | スクリプト内 chrono 0.4.45、fs2 0.4.3 |
| KubuntuStrageCleanRustscript | walkdir 2.5.1 |
| jarvis | rmcp 3.11.0、sqlx 0.9.1、axum 0.8.9、cedar-policy 4.15.0 |
| grobalOpencodeSettings | okf_common の serde_yaml（保守終了） |
| userscript | playwright 1.65.1、Node 22、可変アクションタグ |
| dsh-gitbash-win | playwright-core 1.65.1、DSH rc.5 未検証、gitleaks |
| knowledge | checkout/cache v4、dtolnay@stable |
| workflows | install-action ピン v2.90.3（共通ゲート側） |

### 顕著な更新候補なし／静的コードなし（10 リポ）

`-rustscript`、`dsh-config`、`Policy_discussion_casualconversation`、
`Pixiv_Ultimate_Filter`、`-----onetab---`、`agents`、`kaibunshoHokanko`、
`openclaw-workspace`、`pixivAndroidtool`（空リポ）、`X-Twitter-Search-and-Timeline-Mute`。

`agents` は設定ファイル主体で、今回の走査範囲では更新候補は検出されなかった。
ユーザースクリプト系 3 リポは npm/Rust のマニフェストを持たない。

### 追加調査: fork 4 リポ

fork は上流の追従関係があるため、「最新版に上げる」ことが常に正しいとは限らない。
以下の判定はローカル fork の現状に対するもので、上流との差分は未調査である。

| リポ | マニフェスト | 主な更新候補 | 判定 |
| --- | --- | --- | --- |
| opencode | `package.json` / `bun.lock`（bun@1.4.4、workspace catalog） | bun 1.4.5、`@playwright/test` 1.60.0→1.65.1、oxlint 1.60.0→1.94.0、turbo 2.10.2→2.11.13、`@types/node` 24.12.2→26.6.6、vite は catalog 7.3.7 と patch 8.2.4 が不整合（最新 8.3.8）、solid-js 1.9.17→2.0.3（メジャー） | catalog と overrides で管理。上流の更新方針に従う |
| OpenCodeRust | `Cargo.toml` / `Cargo.lock` / `renovate.json` | `oxc_*` `=0.154.0`→0.160.0（exact pin）、notify lock 8.2.2→9.0.0、walkdir 2.5.0→2.5.1、toml lock 1.2.1→1.2.2。sqlx 0.9.1 / axum 0.8.9 / tokio 1.54.1 / clap 4.6.9 は最新 | MSRV 1.97（1.100.0 で検証済みのコメントあり）。renovate 設定済み |
| UserScripts | なし（ユーザースクリプトのみ） | 依存なし | 更新対象なし |
| ZenzaWatch | `package.json`（devDependencies のみ） | gulp 3.9→5.0.1、webpack 4.27→5.113.0、eslint 4.19→10.15.0、mocha 3.5→12.0.3、jquery 3.3→4.1.0、jsdom 11→31.0.1、`@babel/core` 7.4→8.1.3 | 2018 年世代のスタック。個別更新ではなく全面移行が必要 |

opencode の catalog は TypeScript を `@typescript/typescript6@6.0.2` エイリアスと
`typescript@7.1.0-beta` に分けており、安定版 7.0.2 をそのまま採用していない。
意図的な選択かどうかは上流を確認するまで未確認である。

## R4 での修正

- 子エージェントは `OLD_ACCOUNT/workflows` の到達性を「未確認」としていたが、
  親が GitHub MCP で確認したところリポは実在し、v1 の SHA は
  `b7675efd8a9a84ebc5b560da628d1a82f3fbf98f` で `ACCOUNT/workflows` と同一だった。
  フォローアップで `GET /users/OLD_ACCOUNT` と `GET /users/OLD_ACCOUNT/repos` が
  いずれも 404 であること、`GET /repos/OLD_ACCOUNT/workflows` が
  `ACCOUNT/workflows` へリダイレクトされることを確認し、リネームが確定した。
  `gh auth status` の表示は旧名のままだが、`gh api user` の `login` は `ACCOUNT` である。
- 子エージェントは cargo-deny の統一先を 0.21.0 としていたが、
  最新リリースは 2026-11-19 公開の 0.22.0 だった。本レポートでは 0.22.0 を採用した。

## 推奨される次の一手

1. P2 の低リスク更新をリポごとに実施し、各リポの標準検証ループ
   （fmt → check/clippy → test）を通す。
2. P1 は移行ブランチを切り、破壊的変更の差分レビューとテストを先に用意する。
3. `ACCOUNT/workflows` 側で install-action を v2.90.8 の SHA へ、
   cargo-deny を 0.22.0 へ統一し、self-test 後に v1 タグを更新する。
4. 可変タグ参照を SHA 固定へ寄せ、共通ワークフロー呼び出し側の書式を統一する。
5. gitleaks 系 2 リポを Betterleaks v1.10.1 主軸へ移行するか、例外を文書化する。
6. dsh-gitbash-win の CI matrix に DSH rc.4 / rc.5 を追加し、互換性を実測してから
   `dshReleases` と peer 範囲を更新する。
7. fork については先に上流との差分を確認する。OpenCodeRust は renovate 設定が
   あるため自動更新に委譲し、ZenzaWatch は全面移行が必要なら専任タスクとして分離する。

## 未確認事項と限界

- 更新後のビルド・テストは未実施。互換性は推奨事項であって確認済みではない。
- Node 22 のサポート終了時期、および `@wdio/tauri-service` の WDIO 10 対応版の有無は未確認。
- `src/` 配下のコードを読んでいないため、宣言バージョンが実際に使う API と整合するかは未検証。
- 旧名 `OLD_ACCOUNT` 配下のリポはリネームにより存在しないことを確認済み。
- fork 4 リポの依存は調査済みだが、上流リポジトリとの差分は未調査。
  「最新版へ更新」が上流追従方針と冲突する可能性がある。
- opencode の TypeScript エイリアス（6.0.2 / 7.1.0-beta）が意図的かどうかは未確認。

## 出典

- [Rust Releases](https://api.github.com/repos/rust-lang/rust/releases/latest)
- [crates.io API](https://crates.io/api/v1/crates/notify)
- [npm registry](https://registry.npmjs.org/typescript/latest)
- [Node.js dist index](https://nodejs.org/dist/index.json)
- [ACCOUNT/workflows tags](https://api.github.com/repos/ACCOUNT/workflows/git/refs/tags/v1)
- [旧ユーザー名 OLD_ACCOUNT（404）](https://api.github.com/users/OLD_ACCOUNT)
- [認証ユーザー ACCOUNT](https://api.github.com/user)
- [opencode package.json](https://github.com/ACCOUNT/opencode/blob/dev/package.json)
- [OpenCodeRust Cargo.toml](https://github.com/ACCOUNT/OpenCodeRust/blob/master/Cargo.toml)
- [ZenzaWatch package.json](https://github.com/ACCOUNT/ZenzaWatch/blob/playlist-deploy/package.json)
- [各リポジトリのマニフェストとワークフロー](https://github.com/ACCOUNT)
