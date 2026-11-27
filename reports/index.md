# レポート一覧

## 文書一覧

<!-- okf-index:begin -->
| 文書 | 状態 | 概要 |
| --- | --- | --- |
| [DSH / OpenCode プラグイン導入調査](2026-11-27-dsh-opencode-plugin-survey.md) | draft | 2026-11-27 時点のローカル環境実測と一次情報をもとに、DSH と OpenCode へ導入すべきプラグインを絞り込んだ調査報告 |
| [AIエージェント時代の秘密・機微情報管理のベストプラクティス調査](2026-11-24-ai-agent-secrets-management-best-practices.md) | draft | 2026年11月時点の一次情報をもとに、エージェント認証・動的シークレット・実行時制御・ローカル保管・規制要件を横断調査した |
| [ローカル LLM におけるスパースアテンション活用の実現可能性評価](2026-11-23-local-sparse-attention-llm-feasibility.md) | draft | GTX 1660 SUPER 6GB / Turing sm_75 環境で、スパース・線形アテンション系モデルの速度、エージェント適性、コミュニティ派生のリスクを実測と一次情報で評価した。 |
| [個人リポジトリの依存・ライブラリ・言語バージョン最新性チェック](2026-11-23-personal-repos-dependency-freshness.md) | draft | `<current-account>` 配下 31 リポジトリ（オリジナル 27 + fork 4）のマニフェストを一次レジストリと照合し、更新候補と優先度を整理した |
| [シェル言語別 AI エージェント誤り率の実測](2026-11-23-shell-agent-mistake-rate.md) | approved | bash / zsh / nushell / dash の4シェルに15問ずつ出題し、隔離環境で誤り率を計測した |
| [AI利用制限契約の適正化に関する政策提言 レビューレポート](2026-11-09-ai-usage-restriction-policy-review.md) | draft | 政策提言案の事実記述を一次情報で検証し、論理・法制度・制度設計上の課題と修正案を優先度付きで整理したレビュー。 |
| [AI利用制限契約の適正化に関する政策提言 v2（レビュー反映改訂版）](2026-11-09-ai-usage-restriction-policy-v2.md) | draft | 原案とレビューを踏まえ、立法事実の明示、禁止理由の類型化、無効化対象の限定、既存制度との接続、ソフトロー先行の実装順序を組み込み、独立レビューの指摘も反映した政策提言の改訂版 |
| [AI利用制限契約の適正化に関する政策提言](2026-11-09-ai-usage-restriction-policy.md) | deprecated | 認定エンタープライズAIの標準利用を原則自由とし、合理的根拠のない利用禁止契約の無効化と、取引上の優越的地位を利用した制限の防止を提案する。 |
| [放送ニュース透明性制度 現段階フル版](2026-11-09-broadcast-news-transparency-governance.md) | draft | 放送報道の情報種別透明化制度案を検証可能なPoCへ移行するためのGovernance・Manifest・Trust・Cross-review設計の統合ドラフト |
| [AIデータセンターの水道直結型ワンススルー冷却・静音化構想](2026-11-09-data-center-once-through-cooling.md) | draft | 水道水を独立した熱輸送媒体として熱交換器に通し、下水道へ排出する方式について、熱収支・水量・配管・水道事業コスト・排水・騒音・制度設計を札幌市の公開データで検討する。 |
| [オープン教材・Markdown・GitHub・AI家庭教師による教育DX構想](2026-11-09-education-dx-open-textbook-ai-tutor.md) | draft | 既存標準とクラウドサービスを活用し、構造化教材を制作・配布・再利用してAI家庭教師に接続する構想の調査・設計レポート |
| [公共・非営利団体会計情報公開標準 v3（Requirements Draft WD 0.3）](2026-11-09-public-nonprofit-accounting-standard-v3-wd03.md) | draft | v2レビューからWD 0.3メタ仕様確定までの設計判断を統合した公共・非営利団体会計情報公開標準のフルドラフト |
| [会計情報公開標準 v2→v3（WD 0.3）差分評価](2026-11-09-public-nonprofit-accounting-v2-v3-diff.md) | draft | 公共・非営利団体会計情報公開標準の旧案 v2 から v3 WD 0.3 への変更を、章対応・新たな規範化・判断変更・後退点の観点で評価した差分評価レポート |
| [AgentRouter 無料枠の実態と日本語入力の回避策](2026-11-07-agentrouter-model-access.md) | review | AgentRouter のモデルアクセス・クォータ・日本語 content-blocked を実測し、コードポイント変換プラグインで恒久対策した調査報告 |
| [自宅 Linux の IPv6 有効化と外部到達性の評価](2026-11-06-ipv6-enablement-and-firewall.md) | review | `<router>` の RA によるグローバル IPv6 取得、ufw の IPv6 ルール整備、外部到達性テストの結果と運用方針 |
| [自宅常駐サーバのセキュリティ監査（2026-11-04）](2026-11-04-home-server-security-audit.md) | draft | Linux 常駐サーバの外部公開ポート・認証・権限・更新状況を実測し、優先度付きの対策を整理した監査報告 |
| [Apache HTTP Server 2.4.44 から 2.4.69 への破壊的変更調査報告](2026-10-30-apache-httpd-2-4-44-to-2-4-69-breaking-changes.md) | review | Apache HTTP Server 2.4.44 から 2.4.69 までの公式 CHANGES_2.4 を精査し、設定・プロトコル・ビルド要件の後方互換性を損なう変更を重要度別に分類して移行対応を整理した調査報告。 |
| [opencodego モデルルーティング調査報告: サブエージェント別割り当て](2026-10-25-opencodego-model-routing.md) | review | OpenCode Go 加入環境の全モデル候補をコスト・速度・精度・役割適正の4軸で評価し、サブエージェント5種とグローバルデフォルトのモデル割り当てを決定・適用した調査報告。 |
| [コンビニ電子レシート導入の実現可能性評価レポート](2026-10-19-e-receipt-feasibility-report.md) | draft | 大手コンビニ4チェーンのアプリ資産・データ骨格・法務要件を検証し、 紙レシート廃止に至る実装ロードマップを評価する技術報告書 |
<!-- okf-index:end -->
