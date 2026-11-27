---
type: Report
title: "DSH / OpenCode プラグイン導入調査"
description: "2026-11-27 時点のローカル環境実測と一次情報をもとに、DSH と OpenCode へ導入すべきプラグインを絞り込んだ調査報告"
tags:
  - dsh
  - opencode
  - plugin
  - tech-selection
generated:
  by: lead
  at: "2026-11-27T21:05:00+09:00"
status: "draft"
---

## 概要

DeepSeek Harness（以下 DSH）と OpenCode（v2.4.2）の両ハーネスについて、導入済みプラグインの実測と
外部一次情報の並列調査を行い、追加導入候補を評価した。結論は次のとおりである。

- DSH はまず native の `dsh-tool-session-query` と既存 `bash` / `job_*` を試し、
  不足する場合のみ **session-search** と **terminal-bundle** を導入する（条件付き）。
  どちらもローカル完結・ロールバック容易だが、既存機能と一部重複する。
- メモリ層の **dsh-mnemon** は条件付き候補。まず session-search で不足が解消されるか確認してから判断する。
- OpenCode は **opencode-mem** を条件付きで推奨する。既存 MCP memory で要件が足りる場合は不要。
- 通知系・検索インデックス・Ralph ループ・LiveShare・worktree ツールは、既存環境で重複または過剰であるため追加しない。
- OpenCode の V1 SDK 依存プラグイン（opencode-dcp、opencode-openai-codex-auth）は v2.4.2 での
  実動作を確認できず、今回は推奨しない。

調査は swarm-research の R1〜R5 を完全無制限プロファイルで実施し、R6 で親が出典・妥当性を集約した。
子エージェント8体を起動し、そのうち1体（OpenCode 候補検証）と1体（リスク評価）は途中終了したため、
該当項目は親が代替検証した。

## 調査範囲と方法

- 対象: DSH（実行中のプロファイル `web`）と OpenCode（実行バイナリ v2.4.2）。
- 情報源: ローカル設定・プロファイル・キャッシュの直接読み取り、GitHub REST API、npm registry /
  downloads API、OpenCode 公式ドキュメント、各プロジェクトの README。
- 並列調査: R1 論点分解、R2 一次調査（5体）、R3/R4 深掘りと敵対的検証（2体）、R5 リスク評価（1体）。
- 除外: 実機へのプラグイン導入・有効化、課金を伴う操作、外部への書き込み。本報告は調査のみで、
  設定変更は行っていない。

## 確認済みの現状

### DSH 側

- プロファイル `web` には22バンドルが登録されている（標準2・公式カタログ由来10・
  公式サブエージェントプロバイダ2・サードパーティ4・自作4）。
  出典: `~/.dsh/profiles/web/package.json`（ローカル実測、2026-11-27 21:26 時点）。
- サードパーティは `dsh-web-ui-notify`（npm ^0.1.7）、`dsh-ntfy-notify`（ローカル link、`private: true`）、
  `dsh-mobile-maxxing`（ローカル tarball、v1.0.9 ベース）、`dsh-resilient-retry`（ローカル link、v1.3.0）
  の4件。コミュニティの公開インデックスは dshopen.org と awesome-dsh-plugin.com の2件を登録済み。
- `@deepseek-ai/dsh-experimental-plugin-market-bundle` は有効で、
  `~/.dsh/cordis.patch.yml:606-611` に2つのインデックスが登録済み。
  - `https://dshopen.org/index.json`
  - `https://awesome-dsh-plugin.com/index.json`
- `plugin_manager list_bundles` と `~/.dsh/profiles/web/package.json` の状態では、
  公式 experimental バンドルのうち ralph、terminal、session-search、liveshare、tool-worktree
  の5件が未登録・無効（`enabled: false` / `installed: false`）。
  ユーザーは「組み込みプラグイン4個・実験的プラグイン12個がすべてオンで、
  それ以外は存在しない」と報告している。`dsh --profile web --dump-config` では
  ralph / terminal / session-search / liveshare はランタイムに存在せず、
  `worktree` と `tool-worktree` の行のみが有効な forge-bundle 経由で存在した。
  つまり GUI 表示（ユーザー報告）とバンドル登録（`plugin_manager`）は別レイヤーの記述であり、
  矛盾しない。tool-worktree は forge-bundle と機能重複する。
- macOS 専用の Computer Use バンドルは Linux 環境では対象外（`unsupported-platform`）。

### OpenCode 側

- 実行バイナリは `opencode v2.4.2`（`opencode --version` 実測）。V2 SDK `@opencode/plugin` も 2.4.2 で一致。
- `opencode plugin list` は `No plugins found`。npm パッケージプラグインは未導入。
- 自作 TypeScript プラグイン2本のみが `~/.config/opencode/plugins/` にあり、全プロジェクトで自動有効。
  - `agentrouter-escape`: AgentRouter 宛非 ASCII メッセージをコードポイント列へ変換。
  - `termux-notify`: サーバイベントを購読し `notify.rs` 経由で通知。
- MCP サーバ13件（有効9件、無効4件）。正本は `~/.agents/mcp.json` で、
  `~/.agents/bin/mcp-sync.rs --apply` が `opencode.jsonc` を生成する。
- `~/.local/share/opencode/opencode.db` は約170MB。過去セッションが相当量蓄積されている。
- サブエージェント5件、コマンド14件を自前定義済み。レビュー・テスト系は充足している。

## 既存スキル・既存機能との比較

ユーザーから「既存スキルよりも優れているなら比較の上検討すべき」との指摘を受け、
4候補について既存スキル（`~/.agents/skills` 67個）とハーネス既存機能・MCP の重複を再検討した。
判定は「代替可 / 部分的代替可 / 代替不可（独自価値あり）」の3段階とする。

### 既存スキルとの比較

| スキル | 比較対象 | 判定 | 根拠 |
| --- | --- | --- | --- |
| opencode-environment-report | session-search | 代替不可 | 設定の静的棚卸しであり、セッションDBは明示的に読まない |
| shell / loop | terminal-bundle | 代替不可 | 1回限りの実行・反復手順であり、ターミナル状態を保持しない |
| swarm-* 群 | dsh-mnemon / opencode-mem | 代替不可 | todo.md による手動受け渡しで、自動抽出・永続ベクトル状態はない |
| create-hook / ai-selection | opencode-mem | 代替不可 | Codex hooks の手順・選定一覧であり、ハーネスも異なる |
| tech-selection | 比較の枠組み | 該当なし | スコアカード・ADR として本比較に利用可能 |

結論: スキルは手順・テンプレート・評価基準を提供する文的仕組みであり、
実行時のツールや永続状態を提供しない。4候補の代替にはならない。
根拠: [opencode-environment-report](../../../.agents/skills/opencode-environment-report/SKILL.md):73
（「`.env`、認証ストア、セッションDB、ログ、service設定を読み出さない。」）、
[swarm-coding](../../../.agents/skills/swarm-coding/SKILL.md):319（「各ラウンド終了時に親が成果を
担当単位で要約し、次ラウンドへ渡す。」）。

### DSH 既存機能との比較

- **session-search**: `dsh-session-query-sqlite` は SQLite FTS5 バックエンドとしてマウントされるが
  `openAt: never` のため全文検索は無効。Web サイドバー検索はタイトル・workspace 名のみ。
  ただし native の `dsh-tool-session-query` が同一の読み取り専用5ツールを提供しており、
  experimental バンドルは有効化ラッパーに近い。判定は**部分的代替可**。
- **terminal-bundle**: `tool-bash` / `tool-jobs` はホスト層で disabled だが standard プリセットが
  再マウントし、本セッションでも `bash(run_in_background)` と `job_*` は呼べる。
  `dsh-terminal-panel` と `dsh-tool-terminal` も本体同梱。long-lived 実行と出力回収は既存で足り、
  バンドルの固有価値は全プリセットへの一括 globals 化と isolated registry。
  判定は**部分的代替可**（サイドバーで人間が監視・引き継ぎする PTY が必要な場合のみ独自価値）。
- **dsh-mnemon**: `~/.agents/mcp.json` の memory 定義は DSH 側が `supported: false`。
  セッション検索と永続化はあっても自動抽出・自動注入は無く、代替不可。
  根拠: [packages.md](../../../.npm-global/lib/node_modules/@deepseek-ai/dsh/node_modules/@deepseek-ai/dsh-agent-preset/skills/dsh-doc/references/packages.md):406,423-424,234-237。

### OpenCode 既存機能との比較

- 既存の MCP memory（`@modelcontextprotocol/server-memory` 2.0.0）は有効で、
  entities / relations / observations の knowledge graph を明示ツールで操作する。
  保存先は `~/.local/share/opencode/memory.jsonl` で、現時点でファイルは存在しない。
- `opencode-mem` は Turso/libSQL ベクトル DB で、アイドル時に記憶を抽出し次セッションへ
  自動注入する。検索は埋め込みベースの類似検索であり、MCP memory の文字列検索とは異なる。
- 判定: 明示的な事実・関係の手動保存だけを要するなら**部分的代替可**。
  あいまいな意味類似での recall や自動注入を求めるなら**代替不可（独自価値あり）**。
  既存グラフが空のため移行作業は不要。
  根拠: `opencode.jsonc:382-392`、`server-memory/README.md:3,115,327,339,350`（ローカル実測）。

### 結論の修正

- session-search / terminal-bundle: native の `dsh-tool-session-query` と `bash` / `job_*` で
  要件が足りる場合は experimental バンドルは不要。**条件付き**へ格下げる。
  isolated index や全プリセットへの一括 globals 化が必要な場合のみ experimental を選ぶ。
- dsh-mnemon: DSH 側にメモリ基盤が無く、既存スキル・既存機能での代替不可を確認。**推奨を維持**
  するが、まず session-search の有効化で不足が解消されるか確認する前提は変わらない。
- opencode-mem: 既存 MCP memory で要件が足りる場合は導入不要。**条件付き**へ格下げし、
  自動抽出・自動注入を明確に求める場合のみ導入する。採用前に実機で両機能を確認する。

## 推奨: DSH

### 条件付き: session-search

- パッケージ: `@deepseek-ai/dsh-experimental-session-search`（公式・experimental）。
- 機能: 5つの読み取り専用グローバルツールで過去セッションを検索・読み取り。
  コンテンツインデックスは初回検索時にオープンし、全エージェントで共有される。
- 採用理由: `opencode.db` 170MB 相当の蓄積は DSH 側にも当てはまる。ただし native の
  `dsh-tool-session-query` が同一5ツールを提供するため、まずそちらの有効化を検討する。
  isolated index でホスト設定を変えたくない場合のみ experimental を選ぶ。
  読み取り専用で外部送信がなく、HMR で即時に有効・無効を切り替えられる。
- リスク: 低。pre-stable だが、ロールバックはプラグインの無効化のみ。
- 出典: チェックアウト内の
  `~/.npm-global/lib/node_modules/@deepseek-ai/dsh/node_modules/@deepseek-ai/dsh-experimental-session-search/README.md`
  （ローカル、2026-11-27取得）。

### 条件付き: terminal-bundle

- パッケージ: `@deepseek-ai/dsh-experimental-terminal-bundle`（公式・experimental）。
- 機能: 6つの永続ターミナルツールと、ユーザー自身のターミナルを読む2つの読み取り専用ツール。
  dev server や REPL を起動し、後のツール呼び出しで新しい出力を読める。
  ターミナルはサイドバータブで監視・引き継ぎ可能。
- 採用理由: Kubuntu をメイン開発環境とするユーザーの開発ワークフローに直結する。
  ただし standard プリセットの `bash(run_in_background)` と `job_*` で long-lived 実行と
  出力回収は既に可能。サイドバーで人間が監視・入力引き継ぎする PTY が必要な場合のみ導入する。
- リスク: 低〜中。有効化にはプロファイル再起動が必要。行単位の override で無効化可能。
- 出典: 同チェックアウトの `dsh-experimental-terminal-bundle/README.md`（ローカル、2026-11-27取得）。

### 条件付き: dsh-mnemon

- パッケージ: `dsh-mnemon`（MIT、SLSA provenance 付き、直近30日 164,107 DL、
  GitHub 1,146 star）。
- 機能: layered memory（Runtime memory / Project Documents / Memory Spaces）と
  self-evolving skills を提供するメモリ・コンテキスト層。
- 採用条件: session-search を導入しても「過去の判断や経緯が参照できない」不満が残る場合に限る。
  メモリ層を追加すると既存のセッション永続化・SQLite 検索と役割が近づき、運用コストが増える。
- リスク: 中。依存23パッケージと多く、メモリの自動注入が既存の `AGENTS.md` 指示と競合する可能性がある。
- 出典: [dsh-mnemon README](https://raw.githubusercontent.com/omdsh-dev/dsh-mnemon/main/README.md)、
  [npm registry](https://registry.npmjs.org/dsh-mnemon/latest)、
  [npm downloads API](https://api.npmjs.org/downloads/point/last-month/dsh-mnemon)
  （いずれも 2026-11-27取得）。

### コミュニティプラグインの機能比較

前回の回答では「既存機能との重複で絞り込んだ」と述べたが、実際に機能内容と既存機能を
突き合わせた対応表は作っていなかった。各候補の README / index.json を再取得し、
既存機能と比較した結果を次に示す。

| 候補 | 主な機能 | 既存の相当物 | 判定 |
| --- | --- | --- | --- |
| dshmarket | 12,000+件の閲覧・1クリック導入・一括更新・テーマのライブ切替・バックアップ/復元・競合診断 | Discover + 2インデックス（閲覧・導入・更新のみ）、client-ui-theme（light/dark/system のみ） | **一部独自価値**。テーマ切替・バックアップ・診断は既存なし |
| dsh-TUI | ターミナルネイティブ UI、TPS ゲージ、コンテキスト進捗、double-Esc rewind、通知4種 | Web GUI、通知3系統（web-ui / ntfy / mobile-maxxing） | 通知は重複。UI 自体は別物で Web GUI メインなら不要 |
| @klarkxy/dsh-git-commit | 会話から git commit、モデルがグルーピングとメッセージ生成 | エージェントの bash と AGENTS.md §4 の運用 | 重複。不要 |
| @klarkxy/dsh-delegation | セッション単位の委託モード、主代理が計画・委託・検収 | subagent / agent-team と swarm スキル群 | 重複。不要 |
| @klarkxy/dsh-current-title | セッションタイトルの自動更新（手動名は保持） | session-titles-bundle（有効） | ほぼ重複。不要 |
| @klarkxy/dsh-web-search-manager | web_search / web_fetch の複数プロバイダを1設定画面で管理・順序付け（DuckDuckGo, Exa, Tavily, Brave, Serper, Jina 等） | exa / tavily / context7 の MCP（個別接続）。`web_search` は DeepSeek API バックエンドのため AGENTS.md §16 で使用禁止 | **一元管理と web_search の置き換えは独自価値あり**。AGENTS.md §16 の懸念に直結するため導入検討候補 |

出典: [dsh-market README](https://raw.githubusercontent.com/dsh-market/dsh-market/main/README.md)、
[dsh-TUI README](https://raw.githubusercontent.com/ccch1mneyyy/dsh-TUI/main/README.md)、
[klarkxy index.json](https://raw.githubusercontent.com/klarkxy/dsh-plugins/main/index.json)
（いずれも 2026-11-27 取得）。

前回の「dshmarket は不要」「web-search-manager は不要」という判断は、機能内容の比較が
不足していたため修正する。dshmarket は条件付き候補、web-search-manager は
AGENTS.md §16 の「DeepSeek API を Web 検索バックエンドとして使わない」方針を
恒久化する手段として検討する価値がある。

### 追加しないもの

| 候補 | 判断 | 根拠 |
| --- | --- | --- |
| 通知系プラグイン | 不要 | ブラウザ通知・ntfy push・PWA 通知の3系統が導入済み |
| 検索インデックスの追加 | 不要 | dshopen.org と awesome-dsh-plugin.com の2件が登録済み |
| ralph-bundle | 不要 | swarm スキル群で同種の反復実行を運用済み |
| liveshare | 不要 | 別途 Liveshare サーバのデプロイが必要で単独開発に過剰 |
| tool-worktree | 不要 | 有効な forge-bundle が同一機能を提供 |
| dshmarket | 条件付き | 閲覧・導入・更新は Discover で代替。テーマのライブ切替・バックアップ/復元・競合診断が必要な場合のみ |
| dsh-tui | 保留 | 8,351 star、週次15,602 DL と活発だが Web GUI を主利用するため優先度は低い |

## 推奨: OpenCode

### 推奨: opencode-mem

- パッケージ: `opencode-mem` 2.44.0（MIT、GitHub 1,895 star、週次13,006 DL）。
- 機能: 組み込み Turso/libSQL によるローカルベクトル DB。アイドル時に記憶を抽出し、
  後のセッションへ自動注入する。Web UI 付き。
- 採用理由: npm の依存に `@opencode/plugin ^2.3.2` を含み、V2 API 対応が確認できる。
  外部サービス不要で、設定は `opencode.jsonc` の `plugins` 配列に
  `"opencode-mem@latest"` を追加するだけ。
- リスク: 低。署名付きパッケージだが SLSA attestation は確認できなかった。
- 出典: [GitHub](https://github.com/tickernelz/opencode-mem)、
  [npm registry](https://registry.npmjs.org/opencode-mem/latest)、
  [downloads API](https://api.npmjs.org/downloads/point/last-week/opencode-mem)
  （いずれも 2026-11-27取得）。

### 条件付き・非推奨

| 候補 | 判断 | 根拠 |
| --- | --- | --- |
| opencode-plugin-litellm | 条件付き | V1/V2 両対応、週次15,217 DL。LiteLLM プロキシを運用する場合のみ |
| opencode-dcp | 非推奨 | V1 SDK のみ依存、AGPL-3.0、README が開発鈍化と Sleep 移行を明記 |
| opencode-supermemory | 非推奨 | ホスト型で外部送信あり。ローカル完結の opencode-mem を優先 |
| opencode-openai-codex-auth | 非推奨 | V1 SDK のみ依存で v2.4.2 での動作未確認 |
| opencode-claude-auth | 非推奨 | README 自身が ToS 上の回避策と明記 |
| Figma プラグイン | 不要 | 対象ワークフローに Figma 連携の要求なし |

## リスク評価

- **サプライチェーン**: npm プラグインは任意コード実行を伴う。主要候補の `scripts` に
  postinstall / prepare 等の自動実行は見当たらず、build・test 中心であった（親が registry で確認）。
  `dsh-mnemon` と `dshmarket` は npm attestation と署名の両方を持つ。
  `opencode-mem` は署名のみで attestation は未確認。
- **ライセンス**: 主要候補は MIT。`opencode-dcp` のみ AGPL-3.0-or-later で、
  個人利用では問題になりにくいが、配布時の影響を考慮する必要がある。
- **API 世代**: OpenCode は V1（`@opencode-ai/plugin`、設定キー `plugin`）と
  V2（`@opencode/plugin`、設定キー `plugins`）が混在する。v2.4.2 は V2 であり、
  V1 SDK 依存のみのプラグインは起動しないか、将来の更新で壊れる可能性がある。
- **重複**: 通知系は DSH 側で3系統、レビュー・テスト系 agent/command は OpenCode 側で自前充足。
  新規プラグインはこれらと重複しない領域に限定すべき。

## klarkxy 3プラグインと swarm 系スキルの違い（2026-11-28 再評価）

ユーザーから「既存よりクオリティ・使い方がうまいなら検討すべき」「swarm 系スキルとの違いは？」
「DSH がシームレスに使いやすかったりするのではないか？」との指摘を受け、
klarkxy 製3プラグインを swarm-research（完全無制限プロファイル、子4体）で再評価した。

### スキルとプラグインの境界

- スキルはモデルが `skill` ツールで明示的に読み込むプロンプト手順であり、
  ツールや UI を登録しない。プラグインは起動時に Cordis へ effect として登録され、
  waterfall listener（`agent/pre-step` / `tools/pre-execute` 等）としてループ途中で自動適用される。
- ただし「スキル＝永続状態なし」は不正確。swarm 系スキルは `/goal` で R1〜R6 を
  ランタイムの永続ゴールとして扱い、review-flow は SQLite に結果を蓄積する。
  正しくは「スキル＝手順の明示的読込＋ゴール/DB による永続状態」と区別する。
- `swarm-limited` / `swarm-unlimited` はリソース予算の上書き層であり、
  機能そのものは対象スキルに委譲する。delegation プラグインとは役割が異なる。

出典: `~/.agents/AGENTS.md:81`、`~/.agents/skills/swarm-coding/SKILL.md:129-130`、
[skill-tool row](../../../.npm-global/lib/node_modules/@deepseek-ai/dsh/node_modules/@deepseek-ai/dsh-agent-preset/skills/dsh-doc/references/packages.md):435、
[practices.md](../../../.npm-global/lib/node_modules/@deepseek-ai/dsh/node_modules/@deepseek-ai/dsh-agent-preset/skills/dsh-doc/references/practices.md):8-16。

### 3プラグインの再評価

| プラグイン | 最新版 | 既存との関係 | 判定 |
| --- | --- | --- | --- |
| `@klarkxy/dsh-current-title` | 0.3.11 | 公式 session-titles-bundle と中核（全メッセージ追跡・手動名保持・失敗時旧タイトル維持）はほぼ同一。klarkxy は Web UI・更新頻度選択・カスタムプロンプト・再生成ボタン・競合 provider の自動無効化と復元で差別化 | **部分的代替可**（前回の「完全重複」を修正） |
| `@klarkxy/dsh-delegation` | 0.2.8 | 既存 `subagent` / `subagent_fork` + swarm-coding で機能は代替可。ただしセッション作成ページのトグル、選択の記憶、fork 継承、子への executor ガイダンス自動付与は独自。ツール・権限は不変で委譲は誘導のみ | **機能は代替可、UI/状態は独自** |
| `@klarkxy/dsh-git-commit` | 0.3.11 | AGENTS.md §4 はエージェント判断の commit を許すが、変更のグルーピング・モデル計画・失敗時復旧・設定保持を担う既存なし。会話右上の Commit ボタン＋確認ダイアログで、既定 `allowFallback=false` は安全側 | **代替不可（独自価値あり）** |

いずれも Web 界面を持ち、スラッシュコマンドではなく UI 操作として統合される。
DSH >=0.2.1-rc.1 を要求し、直近4日以内に npm publish、前日にリポジトリ更新と活発である。
出典: [klarkxy index.json](https://raw.githubusercontent.com/klarkxy/dsh-plugins/main/index.json)、
[current-title](https://klarkxy.github.io/dsh-plugins/en/plugins/current-title/)、
[delegation](https://klarkxy.github.io/dsh-plugins/en/plugins/delegation/)、
[git-commit](https://klarkxy.github.io/dsh-plugins/en/plugins/git-commit/)
（いずれも 2026-11-28 取得）。

### DSH のシームレス性

5軸で定義できる: ①コマンド不要 ②モデルの自動選択 ③GUI 設定完結 ④HMR/再起動の自動
⑤セッション横断の状態持続。

- `dshmarket`: ③④⑤を満たす。toggle は `cordis.patch.yml` に書き出され HMR で約1秒再構成、
  テーマは即時適用かつ再起動後も選択が残る。
- `web-search-manager`: ②③を満たす。Settings 1画面でプロバイダ順序・モード・承認を管理し、
  `ctx.web` を引き継いで tool-web が無い構成でもモデルへ `web_search` / `web_fetch` を自動提供。
- `current-title`: ②③⑤を満たす。インストール後は自動更新、手動名は保護。
- `delegation`: ③⑤を満たすが、プロンプト誘導のみで②の確実性は限定的。
- `git-commit`: ①は満たすがボタン操作が必須で②は満たさない。過大評価を避ける。

結論: ユーザーの指摘のとおり、DSH プラグインはスキルより「シームレス」な軸（GUI 完結・
自動適用・状態永続）を持つ。ただし git-commit と delegation はモデルが直接呼ぶツールではなく
UI 操作である点を明示し、swarm スキルは「一回限りのオーケストレーション手順」として
棲み分けるのが妥当である。

## 導入結果

ユーザーの明示的な指示により、2026-11-27 21:56 に次の2件を web プロファイルへ導入した。

- `dshmarket` 1.84.1: `dsh plugin --profile web add dshmarket`
- `@klarkxy/dsh-web-search-manager` 0.4.1:
  `dsh plugin --profile web add @klarkxy/dsh-web-search-manager`

検証結果:

- 両件とも `~/.dsh/profiles/web/package.json` の `dsh.profile.bundles` と `dependencies` に登録された。
- `dsh --profile web --dump-config` に `dsh-market`（name: `dshmarket`）と `web-search-manager`
  （name: `@klarkxy/dsh-web-search-manager`）のランタイム行を確認した。
- `pnpm peers check` は missing peer を報告したが、対象は既存の subagent 関連と
  `@deepseek-ai/dsh-web-search-exa` が中心で、今回の2件のロードは成功している。
- Web Search のプロバイダ設定・順序付けは DSH 再起動後の Settings → Web Search で行う。
  再起動後の実動作は未確認。

2026-11-28 の実測で、次はすべて**未導入**と確定した（bundles / dependencies / `dump-config` /
`opencode plugin list` のいずれにも不在）: `dsh-mnemon`、`dsh-remote`、
`@deepseek-ai/dsh-experimental-session-search`、`@deepseek-ai/dsh-experimental-terminal-bundle`、
`opencode-mem`、`opencode-supermemory`。導入済みは `dshmarket` と
`@klarkxy/dsh-web-search-manager` の2件のみで、前回の記録と一致する。
session-query-sqlite は base 由来で存在するが `openAt: never` のため全文検索は無効、
native の `terminal-controller` / `ui-sidebar-terminal` / `pty` は存在するが
永続 PTY ツール（`dsh-tool-terminal`）は未ロード。

2026-11-28 02:26 に、ユーザーの承認（「任せた」）のもとで次を実施した。

- `dsh-delegation` の非重複部分のうち、スキルへ移植可能な executor ガイダンスを
  `swarm-coding` / `swarm-research` / `swarm-todo-discovery` / `swarm-project-review`
  の子エージェント指示テンプレートへ追加した。「親は coordinator として計画・割当て・
  検収を担い、子は executor として割当て範囲のみを実行する。判断が必要なら親へ返す。
  `subagent_fork` で生えた子も同じ方針を継承する」という一文。
  UI トグル・選択の記憶・ターン後のロックはスキル単体では移植不可のため未実施。
- `@klarkxy/dsh-git-commit` 0.3.11 を web プロファイルへ導入。
  `dsh --profile web --dump-config` に `git-commit`（`disabled: false`）のランタイム行を確認。
- 2026-11-28 02:34 に、ユーザーの追加指示により `@klarkxy/dsh-current-title` 0.3.11 も
  web プロファイルへ導入。`bundles` / `dependencies` に登録され、
  `--dump-config` に `current-title`（`disabled: false`）のランタイム行を確認。
  既存の `optional-session-title-all-prompts` 行は静的設定に残るが、プラグインが
  起動時に競合 provider を自動で無効化し、アンインストール時に復元する設計のため
  追加の手動設定は不要。
- 2026-11-28 02:45 に、ユーザーの「入れただけで負荷がかかるわけでもないなら
  とりあえず入れておけばいいのでは」との判断により、残る未導入候補も導入した。
  - `dsh-mnemon` 0.5.55 / `dsh-remote` 1.5.0
  - `@deepseek-ai/dsh-experimental-session-search` 0.2.1-alpha.2
  - `@deepseek-ai/dsh-experimental-terminal-bundle` 0.2.1-alpha.2
  - `opencode-mem` 2.44.0（`opencode.jsonc` の `plugins` に登録、`opencode plugin list` で確認）
  - `dsh-delegation` のみ除外（executor ガイダンスをスキルへ移植済みで二重指示になるため）
  - 公式 experimental の2件は npm 公開版が `0.2.1-alpha.2` で、
    チェックアウトの `0.2.2-rc.5` と異なる。`--dump-config` は成功しているが、
    実動作は DSH 再起動後に確認が必要。
- 負荷に関する補足: `dsh-current-title` は既定 `cadence='all-prompts'` で
  毎メッセージにタイトル生成 LLM を呼ぶ。`dsh-mnemon` / `opencode-mem` は
  アイドル時の自動抽出ベクトル処理が走る。「入れただけ」で完全に無負荷ではない。

## 未確認事項

- DSH のプラグイン有効状態は、GUI 表示（ユーザー報告: 組み込み4・実験的12がすべてオン）と
  バンドル登録（`plugin_manager`: 公式 experimental 5件が無効）でレイヤーが異なる。
  `--dump-config` で両者は矛盾しないことを確認済み。GUI 自体は Pocket ID 認証のため
  エージェントから目視できていない。
- `opencode-dcp` と `opencode-openai-codex-auth` が opencode v2.4.2 で実際に起動するか
  （V1 SDK 依存のため、今回は導入せず未検証）。
- `opencode-mem` と `opencode-supermemory` を同時導入した場合のメモリ注入の競合。
- `dsh-web-ui-notify` の SLSA provenance の有無（コミュニティ情報ではありと報告されたが、
  親の registry 確認では取得できず未確認）。
- 導入済み `dsh-mobile-maxxing` の `v1.0.9-local2` と公開版 v1.0.9 の差分（ローカルビルドと推測）。
- `opencode-mem` のアイドル時自動抽出・次セッション自動注入は実機未検証。
  既存 MCP memory との比較は README とローカル設定に基づく。
- R5（リスク評価）の子エージェントは途中終了したため、独立的なリスク評価は親が代替した。
  上記リスク評価は親の直接確認に基づく。

## 推奨される次の一手

1. DSH 側の追加導入判断は、GUI 表示とバンドル登録が別レイヤーであることを踏まえ、
   `--dump-config` で実際に不足する機能があるかを基準に行う（両者の矛盾は確認済み）。
2. DSH ではまず native の `dsh-tool-session-query` を有効化し、`openAt` の上書きと
   過去セッション検索を試す。ホスト設定を変えられない場合のみ experimental バンドルを検討する。
3. ターミナルはまず既存の `bash(run_in_background)` と `job_*` で dev server を起動し、
   出力回収を確認する。サイドバーでの監視・入力引き継ぎが必要な場合のみバンドルを有効化する。
4. OpenCode ではまず既存 MCP memory で事実・関係の保存が足りるか確認し、
   あいまいな意味類似での recall や自動注入が必要なら `opencode plugin add opencode-mem@latest` を実行する。
5. klarkxy 3プラグインのうち `git-commit` は独自価値があるため第一候補として導入を検討する。
   `current-title` は公式バンドルとの UI 差（更新頻度・再生成ボタン）を重視する場合のみ、
   `delegation` は常時トグルによる軽量ガイダンスが欲しい場合のみ導入し、
   swarm スキルとの二重指示を避ける。
6. 1週間程度で効果と競合を振り返り、dsh-mnemon / dsh-remote の要否を最後に判断する。

## 参照

- [OpenCode 公式プラグインドキュメント](https://opencode.ai/docs/plugins/)
- [OpenCode プラグイン構築ガイド](https://opencode.ai/docs/build/plugins)
- [OpenCode Server プラグイン API](https://opencode.ai/docs/build/plugins/server)
- [OpenCode Ecosystem（V1 コミュニティリスト）](https://opencode.ai/v1/docs/ecosystem/)
- [awesome-opencode](https://github.com/awesome-opencode/awesome-opencode)
- [awesome-dsh-plugin](https://github.com/awesome-dsh-plugin/awesome-dsh-plugin)
- [dsh-market/dsh-market](https://github.com/dsh-market/dsh-market)
- [klarkxy/dsh-plugins](https://github.com/klarkxy/dsh-plugins)
- [omdsh-dev/dsh-mnemon](https://github.com/omdsh-dev/dsh-mnemon)
- [kuraneko1/dsh-mobile-maxxing](https://github.com/kuraneko1/dsh-mobile-maxxing)
