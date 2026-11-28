# レポート更新履歴

## 2026-11-28

- 19:13 - [PC と Bluetooth イヤホンの接続不安定・ユーザビリティ改善調査](2026-11-28-bluetooth-headphones-stability.md)
  を追加。途切れは TX 完了報告欠落と無効な btusb 設定、取り合いはマルチポイント設定が主因と特定。
  R4 で Sony の Linux 対応外・BlueZ 5.85 の plugin 主張を格下げ。R5 で 0411:0374 の
  RTL8761BU 推定と汎用バインドを追記。設定変更は未実施。
- 15:30 - [DSH プラグイン導入時のホットリロード調査](2026-11-28-dsh-plugin-hot-reload.md)
  を追加。標準 `dsh-hmr` が有効で追加プラグイン不要だが、preset 変更・`restart-required`・
  Web release 変更などは同一セッション反映の例外と整理。
- 02:45 - `dsh-mnemon` / `dsh-remote` / session-search / terminal-bundle /
  `opencode-mem` を導入。delegation は移植済みのため除外。公式 experimental 2件は
  npm 版 0.2.1-alpha.2 で、再起動後の実動作は未確認。
- 02:34 - `@klarkxy/dsh-current-title` 0.3.11 を web プロファイルへ導入。
  bundles 登録と `--dump-config` のランタイム行を確認。他は用途不明で保留。
- 02:26 - `dsh-delegation` の executor ガイダンスを swarm 系スキル4件へ移植し、
  `@klarkxy/dsh-git-commit` 0.3.11 を web プロファイルへ導入。ランタイム行を確認。
- 02:16 - [DSH / OpenCode プラグイン導入調査](2026-11-27-dsh-opencode-plugin-survey.md)
  に klarkxy 3プラグインと swarm 系スキルの違い、DSH のシームレス性（5軸）を追記。
  git-commit は独自価値、current-title は部分的代替可へ修正、delegation は機能代替可・
  UI 独自と判定。未導入6件の現状も確定。

## 2026-11-27

- 21:56 - `dshmarket` 1.84.1 と `@klarkxy/dsh-web-search-manager` 0.4.1 を web プロファイルへ
  導入。bundles 登録と `--dump-config` のランタイム行を確認。再起動後の実動作は未確認。
- 21:52 - [DSH / OpenCode プラグイン導入調査](2026-11-27-dsh-opencode-plugin-survey.md)
  にコミュニティプラグインの機能比較表を追記。dshmarket のテーマ切替・バックアップ・
  診断、web-search-manager の web_search 一元代替は既存機能で代替不可と判明し、
  前回の「不要」判断を修正。
- 21:38 - [DSH / OpenCode プラグイン導入調査](2026-11-27-dsh-opencode-plugin-survey.md)
  のプラグイン有効状態を確定。`--dump-config` で GUI 表示とバンドル登録は別レイヤーで
  矛盾しないことを確認（ralph / terminal / session-search / liveshare はランタイムに
  不在、worktree は forge-bundle 経由で存在）。
- 21:36 - [DSH / OpenCode プラグイン導入調査](2026-11-27-dsh-opencode-plugin-survey.md)
  に DSH プラグイン有効状態の食い違いを記録。ユーザーは GUI で組み込み4・実験的12が
  すべてオンと報告するが、`plugin_manager` とプロファイル設定では公式 experimental 5件が
  無効。追加導入判断は食い違い解消まで保留。
- 21:15 - [DSH / OpenCode プラグイン導入調査](2026-11-27-dsh-opencode-plugin-survey.md)
  に既存スキル・既存機能との比較を追記。native の session-query / bash-jobs が
  代替可能なため session-search と terminal-bundle、opencode-mem を条件付きへ格下げ。
- 21:05 - [DSH / OpenCode プラグイン導入調査](2026-11-27-dsh-opencode-plugin-survey.md)
  を追加。swarm-research の R1〜R5 を並列調査し、DSH は session-search /
  terminal-bundle、OpenCode は opencode-mem を推奨。子1体の途中終了分は親が代替検証。

## 2026-11-24

- 20:00 - [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md)
  にスマホ（一般 Android 端末）への展開手順と実機確認結果を追記。
  Termux + llama-server を推奨。Q4_K_M の転送完了を記録。

- 01:20 - シークレットスキャンを `<current-account>/workflows` の
  `secret-gates.yml` へ集約し、当リポジトリは `uses:` で参照する形に変更。
- 00:26 - [AIエージェント時代の秘密・機微情報管理のベストプラクティス調査](2026-11-24-ai-agent-secrets-management-best-practices.md)
  を追加。16体のサブエージェントで一次調査・敵対検証を行い、出典付きで整理。
  R4/R5 で日付・規格名・製品機能の範囲を修正。

## 2026-11-23

- 23:23 - [個人リポジトリの依存・ライブラリ・言語バージョン最新性チェック](2026-11-23-personal-repos-dependency-freshness.md)
  を更新。fork 4 リポ（opencode / OpenCodeRust / UserScripts / ZenzaWatch）を
  対象に追加し、依存最新版との比較結果を記載。対象は 27→31 リポ。
- 23:07 - [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md)
  を更新。Q4_K_M 環境をユーザー領域に構築し、起動スクリプトと検証結果を記載。
- 22:55 - [個人リポジトリの依存・ライブラリ・言語バージョン最新性チェック](2026-11-23-personal-repos-dependency-freshness.md)
  を更新。旧ユーザー名 `<old-account>` が 404 で新名へリネーム済みであることを確定し、
  未確認事項を修正。漏れていた 4 リポはすべて fork のため対象外と明記。
- 22:28 - [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md)
  に unsloth 量子化 4 種の実測を追記。Q4_K_M が最速で推奨と判定。
- 21:38 - [個人リポジトリの依存・ライブラリ・言語バージョン最新性チェック](2026-11-23-personal-repos-dependency-freshness.md)
  を追加。`<current-account>` 配下 27 リポのマニフェストを一次レジストリと照合し、
  更新候補を優先度別に整理。R4 で 2 件の誤りを修正。
- 21:24 - [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md)
  に Jackrong 蒸留 2 モデルの実機検証結果を追記。2B は 125 tok/s、
  4B は 65 tok/s。両者とも工具呼び出し・JSON・追従は正常。
- 20:06 - [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md)
  の速度比較表に日本語特化 Gemma の実測値を追記。
- 20:01 - [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md)
  の分類表を訂正。DeepSeek 蒸留版と Heretic 適用版を分離し、
  Jackrong 2B 蒸留の固有リスクは未確認と明記。
- 19:38 - [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md)
  を更新。コミュニティ派生の分類・リスク、日本語特化版の実測
  （97〜99 tok/s）を追記。
- 19:02 - [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md)
  を更新。Gemma 4 E2B の実測 95.8 tok/s、エージェント目的の実機比較、
  Qwen 3.5 以降の系譜、VRAM＋メインメモリ併用の評価を追記。
- 17:38 - [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md)
  を追加。sm_75 / 6GB で Qwen3.5-4B を実測し 63〜65 tok/s を確認。
- 13:44 - [シェル言語別 AI エージェント誤り率の実測](2026-11-23-shell-agent-mistake-rate.md)
  を追加。bash/zsh/dash/nu の4シェルに15問ずつ出題し隔離環境で採点。
  R4 レビューの指摘（安全性の限界・統計的有意差なし）を反映。

## 2026-11-09

- 22:45 - [AI利用制限契約の適正化に関する政策提言](<2026-11-09-ai-usage-restriction-policy.md>)、
  [同レビュー](<2026-11-09-ai-usage-restriction-policy-review.md>)、
  [改訂版v2](<2026-11-09-ai-usage-restriction-policy-v2.md>) を追加。
  原案は deprecated とし、レビューと独立レビューを反映した v2 を本版とした。
- 22:18 - [会計情報公開標準 v2→v3（WD 0.3）差分評価](2026-11-09-public-nonprofit-accounting-v2-v3-diff.md) を追加。
- 20:22 - [データセンターのワンススルー冷却](<2026-11-09-data-center-once-through-cooling.md>) を取り込み、検証上の留保を追記。
- 20:16 - 政策レポート3本の横断整合性レビューを作成（3本は別件との指摘を受け22:18に削除）。
- 19:36 - [オープン教材・Markdown・GitHub・AI家庭教師による教育DX構想](2026-11-09-education-dx-open-textbook-ai-tutor.md) を追加。
- 19:36 - [放送ニュース透明性制度 現段階フル版](2026-11-09-broadcast-news-transparency-governance.md) を追加。
- 19:36 - [公共・非営利団体会計情報公開標準 v3（Requirements Draft WD 0.3）](2026-11-09-public-nonprofit-accounting-standard-v3-wd03.md)
  を追加。

## 2026-11-07

- 00:15 - [AgentRouter 無料枠の実態と日本語入力の回避策](2026-11-07-agentrouter-model-access.md) を追加。

## 2026-11-06

- 00:40 - [自宅 Linux の IPv6 有効化と外部到達性の評価](2026-11-06-ipv6-enablement-and-firewall.md) を追加。

## 2026-11-05

- 23:15 - セキュリティ監査レポートの H3/M2/Ollama の記述を実行結果に基づき訂正。コード例を Bash に統一。

## 2026-11-04

- 20:27 - [自宅常駐サーバのセキュリティ監査（2026-11-04）](2026-11-04-home-server-security-audit.md) を追加。

## 2026-10-30

- 13:05 - [Apache httpd 2.4.44→2.4.69 破壊的変更調査報告](2026-10-30-apache-httpd-2-4-44-to-2-4-69-breaking-changes.md) を追加。

## 2026-10-25

- 23:20 - [opencodego モデルルーティング調査報告](2026-10-25-opencodego-model-routing.md) を追加。
- 23:50 - 学習利用許容方針を受け、general / arch / ops-reviewer を `muse-spark-1.3-contributor` へ切替。コンソールのプライバシー設定有効化を要実施事項として追記。
- 23:26 - 直近 24h の実測（pro 57 req / $0.53 への急増、muse-spark 0 req）を反映し、実施状況セクションを未確認として更新。
- 23:40 - テスト系サブエージェント（`test-author` / `test-reviewer`）を追加し、割り当て表と設計判断を追記。テスト実行は親の責務として明確化。
- 23:45 - `opencode run` で muse-spark に実リクエストを投げ、正常応答と $0.0022 の課金記録を確認。プライバシー設定の実施状況を確認済みに更新。

## 2026-10-26

- 00:05 - 外部レビュー（Claude Sonnet 5.5）の指摘 8 件を事実確認し、
  妥当な 7 件（コスト非拘束性・評価軸の強度訂正・explore の期間限定・
  mimo の優位性・文書不整合 5 点）を反映。`#variant` 構文の指摘は公式例
  で確認済みとして却下、test-author の mimo 即時切替は A/B 計画に
  組み込み却下。

## 2026-10-21

- 18:08 - ディレクトリ別の更新履歴を開始。

## 2026-10-19

- 22:00 - [電子レシート実現可能性評価](2026-10-19-e-receipt-feasibility-report.md) を追加。
