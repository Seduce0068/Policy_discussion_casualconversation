# ノート更新履歴

## 2026-10-27

- 23:21 - [さくらのクラウドを前提とした移行アーキテクチャ](2026-10-27-sakura-cloud-architecture.md) を更新。
  PostgreSQL 15 の EOL はコミュニティの期限であり、さくらのクラウドの期限ではない旨を明確化。
- 23:01 - [さくらのクラウドを前提とした移行アーキテクチャ](2026-10-27-sakura-cloud-architecture.md) を追加。
  VM、DB アプライアンス、AppRun、オブジェクトストレージの構成と制約を整理。
- 22:55 - [移行先データベースの選定と最新トレンド](2026-10-27-database-selection.md) を追加。
  PostgreSQL を第一候補とし、最新トレンド、比較、移行の注意を整理。
- 22:00 - [Excel 生成と PDF 出力の帳票移行方針](2026-10-27-report-migration.md) を更新。
  Excel の必要性を QA に追加し、直接 PDF 生成の第一候補を Typst に変更。
- 21:28 - [Excel 生成と PDF 出力の帳票移行方針](2026-10-27-report-migration.md) を追加。
  現行フロー、移行方式、技術選定、テスト、リスク、QA 条件を整理。
- 21:24 - [設計書とソースの乖離分析と QA 管理](2026-10-27-gap-analysis-qa.md) を追加。
  乖離の分類、深刻度、QA の項目と運用、テンプレートを整理。
- 20:52 - [レガシー Java のテスト方式とエビデンス運用](2026-10-27-test-strategy.md) を更新。
  JPEG XL を採用せず、WebP を既定とする方針に変更。
- 20:43 - [レガシー Java のテスト方式とエビデンス運用](2026-10-27-test-strategy.md) を更新。
  Edge 155 の JPEG XL 対応が公式に未確認である旨を追記。
- 20:40 - [レガシー Java のテスト方式とエビデンス運用](2026-10-27-test-strategy.md) を更新。
  画面キャプチャの画像形式を JPEG XL に変更し、変換と表示の条件を追記。
- 20:37 - [レガシー Java のテスト方式とエビデンス運用](2026-10-27-test-strategy.md) を更新。
  画面状態のスナップショット、活性・非活性、文字制限、エビデンスの軽量化を追記。
- 20:28 - [レガシー Java のテスト方式とエビデンス運用](2026-10-27-test-strategy.md) を更新。
  レビュー対象物のテキスト化と、バイナリを証跡として扱う方針を追記。
- 20:06 - [レガシー Java のテスト方式とエビデンス運用](2026-10-27-test-strategy.md) を追加。
  Web と Java GUI のテスト方式、テスト仕様書、テストコード、エビデンス、フレームワーク選定を整理。
- 20:02 - [レガシー Java モダナイズの決定事項チェックリスト](2026-10-27-modernization-decision-checklist.md) を更新。
  人間が担う役割と、判断をボトルネックにしない進め方を追記。
- 19:54 - [レガシー Java モダナイズの決定事項チェックリスト](2026-10-27-modernization-decision-checklist.md) を更新。
  初期セットアップの順序、出口条件、容量と解析の前提、現行機能の保証を追記。
- 19:48 - [レガシー Java モダナイズの決定事項チェックリスト](2026-10-27-modernization-decision-checklist.md) を追加。
  設計の前提、データ、運用の残りの決定事項を優先度別に整理。
- 19:43 - [Seasar2・JBoss・Excel 設計書で構成されたレガシー Java のモダナイズ方針](2026-10-27-legacy-java-seasar2-jboss-modernization.md) を更新。
  JDK を Azul Zulu 25（無償版）に確定し、標準 API に限定する方針を追記。
- 19:38 - [Seasar2・JBoss・Excel 設計書で構成されたレガシー Java のモダナイズ方針](2026-10-27-legacy-java-seasar2-jboss-modernization.md) を更新。
  移行は内製主体とし、外部には短期のレビューを求める方針を追記。
- 19:33 - [Seasar2・JBoss・Excel 設計書で構成されたレガシー Java のモダナイズ方針](2026-10-27-legacy-java-seasar2-jboss-modernization.md) を更新。
  有償サポートを必須としない方針を追記。
- 19:26 - [Seasar2・JBoss・Excel 設計書で構成されたレガシー Java のモダナイズ方針](2026-10-27-legacy-java-seasar2-jboss-modernization.md) を更新。
  JDK ディストリビューションを Azul Zulu 25（第一候補）と Amazon Corretto 25（代替）に決定。
- 19:20 - [Seasar2・JBoss・Excel 設計書で構成されたレガシー Java のモダナイズ方針](2026-10-27-legacy-java-seasar2-jboss-modernization.md) を更新。
  Java 25 LTS の採用に合わせて ADR-002 と整合させた。
- 18:52 - [Seasar2・JBoss・Excel 設計書で構成されたレガシー Java のモダナイズ方針](2026-10-27-legacy-java-seasar2-jboss-modernization.md) を更新。
  Spring Boot 4.x + Doma 3、Excel の Markdown 化の決定を反映し、status を approved に変更。
- 18:27 - [Seasar2・JBoss・Excel 設計書で構成されたレガシー Java のモダナイズ方針](2026-10-27-legacy-java-seasar2-jboss-modernization.md) を追加。
  Seasar2/JBoss EAP の寿命、移行方式、Excel 設計書の SSOT 化、CI 自動化、opencode 活用を整理。

## 2026-10-22

- 18:50 - [実家のネットワークカメラと家庭内ネットワークのセキュリティ対策](2026-10-22-home-network-camera-security.md) に
  セキュアバイデフォルトの難しさと成立条件を追記。
- 08:44 - [実家のネットワークカメラと家庭内ネットワークのセキュリティ対策](2026-10-22-home-network-camera-security.md) を追加。
  脅威モデル、優先順位、ルータ・カメラ・アカウントの設定、購入基準を整理。

## 2026-10-21

- 18:08 - ディレクトリ別の更新履歴を開始。
- 18:04 - [韓国語の漢字併用と教育政策](2026-10-21-korean-hanja-education-policy.md) を追加。
- 00:08 - [オープンスタック AI 家庭教師](2026-10-20-open-stack-ai-tutor.md) に
  無料枠、料金、長時間稼働、データ学習、実害の論点を追記。

## 2026-10-20

- 23:52 - [オープンスタック AI 家庭教師](2026-10-20-open-stack-ai-tutor.md) を追加。

## 2026-10-19

- 21:48 - [MiniMax/DeepSeek 比較](2026-10-19-minimax-m31-flash-vs-deepseek-v41-flash.md) を追加。
- 12:17 - [リポジトリ整備メモ](2026-10-19-repository-setup.md) を追加。
