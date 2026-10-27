---
type: Note
title: "Seasar2・JBoss・Excel 設計書で構成されたレガシー Java のモダナイズ方針"
description: "Seasar2 と JBoss EAP、Excel 設計書のレガシー Java 資産を、延命・段階移行・docs-as-code・CI 自動化で刷新する運用と開発手法を整理する。"
tags:
  - legacy-modernization
  - java
  - seasar2
  - jboss
  - docs-as-code
generated:
  by: opencode
  at: "2026-10-27T18:20:21+09:00"
status: "approved"
---

## 結論

- 採用は「延命 + ストラングラー + Spring Boot 移行」の併用。一括書き換えはしない。
- Seasar2（シーサー2）は 2016-09-26 に EOL。JBoss EAP 8 は Jakarta EE 10 であり、
  Seasar2 をそのまま載せることはできない。EAP 8 に移るなら Seasar2 の撤去が前提。
- 移行先は Spring Boot 4.x + Doma 3（Java 25 LTS）、Excel 設計書は Markdown を SSOT とする。
  詳細は [ADR-002](../adr/ADR-002-spring-boot-doma3-excel-to-markdown.md) に記録した。
- 短期は JBoss EAP 7.4 の ELS で延命し、新規・変更の多い機能から
  Spring Boot + Doma 3 の新サービスへ移す。
- Excel 設計書は「正」の座を降ろす。テキストの単一情報源（SSOT）から
  Excel を生成する方向へ寄せる。
- 自動化はテストとドキュメントの再生成から始める。CI ゲートを先に作り、
  移行対象を増やす。
- opencode は調査・変換の下書き・レビューに使う。業務仕様の判断と本番操作は
  人間が保持する。OpenCode Go のモデル別枠内で、調査は flash、判断は pro に
  割り当てる。

## 前提と現状認識

### 技術スタックの寿命

| 要素 | 現状 | 期限・根拠 |
| --- | --- | --- |
| Seasar2 | 最終リリース 2.4.48（2014-12） | 2016-09-26 に EOL。公式サポート終了 |
| JBoss EAP 7.x | javax 世代 | フルサポート 2023-12-31 終了、メンテナンス 2025-06-30 終了、ELS 2030-12-31 まで |
| JBoss EAP 8.x | Jakarta EE 10、JDK 17/21 | GA 2024-02-05、フルサポート 2028-02-05、メンテナンス 2031-02-05 |
| Java | 8 前提の可能性が高い | 最新 LTS は Java 25（2025-09-16）。EAP 8 のサポート構成は 17/21 |
| Excel 設計書 | バイナリ、差分不能 | レビュー・検索・自動化・監査の阻害要因 |

- Seasar2 は Servlet 2.4 世代の設計で、Java 8 以降の公式サポートはない。
- EAP 7.4 は JDK 11/17 をサポートするが、Seasar2 は Java 8 以前の世代であり、
  JDK 17 での動作は保証されない。
- EAP 8 は `javax.*` から `jakarta.*` へ namespace が変わる。Servlet は 6.0。
- Seasar2 は `javax.servlet` と Javassist に依存する。EAP 8 上で動かすには
  Jakarta 化と DI 移行が不可避であり、そのまま載せることはできない。

### Excel 設計書が生む負債

- Git の差分が読めず、レビューが「開いて目視」になる。
- セル結合、数式、マクロ、外部リンク、埋め込み画像が仕様の実体になる。
- Excel と実装の二重管理が常態化し、どちらが正か分からなくなる。
- 全文検索できず、用語揺れと重複定義が増える。
- 属人化し、AI にも渡しにくい。構造化されていないため変換品質が安定しない。
- 変更履歴と承認の証跡が残らない。

## 移行戦略の選択肢

### 方式の比較

| 方式 | 内容 | 長所 | 短所 | 適用条件 |
| --- | --- | --- | --- | --- |
| リホスト | EAP 8 へ載せ替え | インフラ更新が中心 | Seasar2 が動かない。実質フレームワーク移行 | 既に Jakarta 対応済みのアプリのみ |
| フレームワーク移行 | Seasar2 を Spring Boot / Jakarta EE 標準へ | 既存資産と業務知識を活かす | 全画面・全バッチの改修が必要 | 中規模以下、テストがある |
| ストラングラー | 機能単位で新サービスへ切り出す | リスク分散、段階リリース、並行稼働 | ルーティング、DB 共有、二重運用の設計が必要 | 大規模、変更頻度が偏る |
| 再構築 | 業務プロセスから見直す | 最大の効果 | コスト・期間・リスクが最大 | 業務自体を変える場合 |

### 採用: 延命 + ストラングラー + Spring Boot 移行

- 延命: EAP 7.4 ELS を契約し、JDK 11 を維持する。Seasar2 は凍結し、
  新規機能を載せない。ELS の上限は 2030-12-31。JDK 11 自体のサポート期限は
  ベンダーごとに異なるため、契約前に確認する。
- 新規・変更の多い機能: Spring Boot 4.x + Doma 3（Java 25 LTS）の新サービス。
  Apache httpd / ALB / API Gateway でルーティングする。
- 残存機能: 変更のタイミングで Spring Boot へ移す。EAP 8 に残す部分も、
  Seasar2 を捨てて Jakarta EE 標準に寄せ、最終的に Spring Boot へ統合する。
  EAP 8 残存時の Java は 21 とする。
- 最終的にランタイムを 1 つに寄せる。EAP と Spring Boot の二重運用は
  移行期間だけに限定する。
- 優先順位は「変更頻度 × ビジネス価値 × 技術的リスク」で決める。
  変更が多く価値が高い機能から移す。変更の少ない基幹機能は後回しにする。

### EAP 8 にそのまま載せられない理由

- EAP 8 は Jakarta EE 10。Servlet 6.0、CDI 4.0、JPA 3.1、`jakarta.*`。
- Seasar2 は Servlet 2.4 と `javax.*` 前提。S2Container の DI/AOP は
  Java 8 以前の世代のリフレクションと Javassist に依存する。
- EAP 8 のサポート構成は JDK 17/21。Seasar2 は Java 8 以前の世代で、
  公式サポートは 2016-09-26 に終了している。
- したがって EAP 8 移行は「Seasar2 撤去 + Jakarta 化 + DI 移行」と同義。
- EAP 7.4 ELS は `javax` のまま延命できるが、2030 年が上限。
  それまでに移行を終える計画が必要。

## ターゲット構成

### スタック対応表

| 層 | 現状 | 移行先 | 備考 |
| --- | --- | --- | --- |
| 実行環境 | Java 8 | Java 25 LTS（新規）/ Java 21（EAP 8 残存時） | JDK は Zulu 25 / Corretto 25 |
| AP サーバ | JBoss EAP 7.x | Spring Boot 4.x 埋め込み（EAP 8 は移行期間のみ） | EAP 8 は Jakarta EE 10 |
| DI/AOP | Seasar2 | Spring DI（CDI は EAP 8 残存時） | dicon を Java Config へ |
| Web | SAStruts / Struts | Spring MVC | JSP は Thymeleaf へ段階移行 |
| O/R | S2Dao / S2JDBC | Doma 3 | 2-way SQL を再利用 |
| 画面 | JSP | Thymeleaf / React | 画面単位で移行 |
| バッチ | 独自バッチ（S2Container） | Spring Batch（Jakarta Batch は EAP 8 残存時） | ジョブ定義を外部化 |
| 帳票 | Excel / POI | JasperReports / テンプレート | PDF のスナップショットテスト |
| ビルド | Ant / Maven | Maven / Gradle Wrapper | 再現性を最優先 |
| テスト | 手動 | JUnit 5 / Testcontainers / Playwright | 特性テストから始める |
| ドキュメント | Excel | Markdown / OpenAPI / DDL | Markdown を SSOT とする |
| 可観測性 | ログファイル | OpenTelemetry / Micrometer | 相関 ID と構造化ログ |

### Seasar2 からの移行マッピング

| Seasar2 | 移行先 | 変換の要点 |
| --- | --- | --- |
| S2Container（dicon） | Spring DI / CDI | Bean 定義、スコープ、AOP。自動変換はテストで検証 |
| S2AOP | Spring AOP / AspectJ | ポイントカットとアドバイスの対応 |
| S2Transaction | `@Transactional` | 伝播属性とロールバック条件 |
| S2Dao | Doma 3 | DAO インタフェースと 2-way SQL |
| S2JDBC | Doma 3 | 流れるインタフェースは書き換え |
| SAStruts | Spring MVC | Action / Form を Controller / Form へ |
| S2Struts | Spring MVC | Struts 設定の移植 |
| Teeda | JSF / Spring MVC | 画面遷移とコンポーネント |
| S2Unit | JUnit 5 + Mockito | テストの置き換え |
| Dolteng | Spring Initializr | プロジェクト雛形 |
| JSP | Thymeleaf | Jakarta 化と式言語の移行 |

- dicon の自動変換は Bean 定義の抽出までとする。AOP のポイントカット、
  スコープ、トランザクション伝播は手作業で検証する。
- Doma 3 は S2Dao の DAO パターンと 2-way SQL を継承し、Seasar2 に依存しない。
  Java 17 以上が前提。
- SQL 資産の再利用性が高いため Doma 3 を採用する。動的 SQL が複雑な箇所は
  Doma の DSL か MyBatis の併用を個別に判断する。既定は Doma 3 とする。
- S2Dao の SQL ファイル名は `Dao名_メソッド名.sql`。Doma は `メソッド名.sql`。
  リネームは機械的にできるが、SQL コメントの文法は異なるため、変換はテストで検証する。

### JBoss EAP 8 への移行

- Jakarta 化: `javax.*` を `jakarta.*` へ置換する。OpenRewrite の Jakarta レシピで
  機械変換し、`javax.transaction.xa` など例外を個別に確認する。
- 静的解析: Red Hat の Migration Toolkit for Applications（MTA）で
  移行困難箇所を洗い出す。Konveyor AI も選択肢になる。
- 構成管理: `provisioning.xml` に feature pack、layer、channel を宣言する。
  `jboss-eap-installation-manager` と Maven プラグインで再現可能にする。
- セキュリティ: Elytron と OIDC を使う。legacy security subsystem は削除済み。
- 運用: CLI、パッチ、CP を自動化する。OpenShift なら EAP Operator を使う。
- 注意: EAP 8 は Jakarta EE 10。Jakarta EE 11 は 2025-06-26 にリリース済みだが、
  EAP 8 の対応は 10 のまま。EAP 9 のリリースは本メモ執筆時点で確認していない。

## Excel 設計書の刷新

### 正となる情報源の再定義

| 設計書 | SSOT | 生成物 |
| --- | --- | --- |
| 要件 | Markdown（要件 ID 付き） | Excel、PDF |
| 画面遷移 | Mermaid | 図 |
| 画面項目 | YAML / CSV | Excel、HTML |
| API | OpenAPI | HTML、クライアント、スタブ |
| DB | DDL（Flyway） | ER 図、テーブル定義書 |
| 帳票 | テンプレート + サンプル | PDF、Excel |
| バッチ | ジョブ定義（YAML） | 設計書 |
| テスト | テストコード / Gherkin | テスト仕様書 |

- Markdown を SSOT とし、Excel は「配布用ビュー」に限定する。正を Excel に戻さない。
- どうしても Excel 編集が必要な領域は、正を Excel にしたまま `xlsx2csv` で
  テキスト化し、Git で差分を見えるようにしてから段階的に移す。
- 変換の優先順位は、参照頻度が高い、変更頻度が高い、移行対象に近い、の順。

### 変換パイプライン

1. 棚卸し: シート、外部リンク、マクロ、数式、画像、セル結合を一覧化する。
2. テキスト化: xlsx を CSV / Markdown へ変換する。セル結合は分割し、数式は値化する。
3. 正規化: 項目 ID、型、桁、必須、コード値、更新日時を揃える。
4. 突合: 元 Excel と変換結果の行数、キー、サンプル値を比較する。
5. 自動生成: DDL から tbls / SchemaSpy、OpenAPI から HTML、Mermaid から図を生成する。
6. 凍結: 変換済み Excel は読み取り専用にする。CI で再生成し、差分ゼロを検証する。

- 変換は LLM で下書きし、人間がレビューする。LLM の出力は「もっともらしいが違う」
  ため、突合テストを必須にする。
- セル結合、数式、マクロは自動変換の最大の敵。先に潰す。
- セル結合は Markdown のテーブルで表現できない。項目を分割して繰り返すか、
  AsciiDoc の grid table を使う。
- 実装は Rust の calamine などを使うと CI に組み込みやすい。
- 用語集を作り、表記揺れを textlint / Vale で検出する。
- 図は PlantUML か Mermaid のテキストにする。画像のままにしない。

### トレーサビリティ

- 要件 ID、設計 ID、テスト ID を front matter かコメントで紐付ける。
- CI でリンク切れ、孤児要件、未テスト要件を検査する。
- 変更影響を「要件 → 設計 → 実装 → テスト」のグラフで追えるようにする。
- Excel の承認履歴は、Git のコミットと PR レビューを一次証跡にする。

## 運用・保守の設計

### 可観測性

- 構造化ログ（JSON）、相関 ID、個人情報のマスキング。
- メトリクス: Micrometer などのメトリクスライブラリから Prometheus へ。
- トレース: OpenTelemetry。新サービスから導入する。
- ヘルスチェック: liveness と readiness を分ける。
- SLO: 可用性、レイテンシ、エラー率。アラートは症状ベースにする。
- ダッシュボード: Grafana。各アラートから Runbook へリンクする。

### 構成管理とリリース

- EAP 8 の `provisioning.xml` を Git 管理する。
- 設定は環境変数、ConfigMap / Secret、Vault に外部化する。
- IaC: Terraform / Ansible。サーバー構成を手作業にしない。
- 不変イメージ、Trivy スキャン、SBOM（CycloneDX）。
- リリース: ローリング / ブルーグリーン、機能フラグ、カナリア。
- DB: Flyway / Liquibase。Expand-Contract で旧新スキーマを併存させる。
- ロールバック手順を自動化し、定期的に訓練する。

### セキュリティ

- Elytron、OIDC、LDAP を標準化する。
- 依存脆弱性: OWASP Dependency-Check と Renovate で継続的に更新する。
- 秘密情報は Vault と環境変数に置く。リポジトリに置かない。
- Excel マクロ（VBA）は廃止する。業務ロジックはコードへ移す。
- AI に渡すデータはマスキングする。閉域ならローカル LLM を検討する。
- 監査ログを構造化し、保持期間とアクセス制御を決める。

### コスト

- EAP サブスクリプションと ELS の費用を、移行期間のコストとして計上する。
- JDK のライセンスを確認する。Oracle JDK の無償範囲、Temurin / Corretto の選択。
- クラウドとオンプレの比較は、移行後の運用体制まで含めて行う。
- 移行人月は、画面・バッチ・帳票の棚卸し結果から見積もる。
- OpenCode Go は $10/月で、モデル別の月間枠がある。調査・変換は flash、
  設計判断とレビューは pro に割り当てる。既存のモデルルーティング報告に従う。

## 開発プロセスと自動化

### リポジトリと CI

- 移行対象ごとに境界を切る。モノレポにするなら、ディレクトリ単位で
  ビルドとテストを独立させる。
- `AGENTS.md` に規約、検証コマンド、アーキテクチャ境界、禁止事項を書く。
- CI ゲート: fmt → compile → unit → integration → static analysis →
  security → AI review。AI review は補助で、決定論的検証を代替しない。
- 再利用可能ワークフローでゲートを共通化し、リポジトリごとに複製しない。
- ArchUnit で依存方向を強制する。レイヤ違反をテストで落とす。
- SonarQube、SpotBugs、PMD、Checkstyle、Error Prone を組み合わせる。
- OWASP Dependency-Check、Trivy、SBOM を CI に置く。
- カバレッジは JaCoCo。数値目標より「変更がテストで検知されること」を重視する。

### テスト戦略

- テストピラミッドを基本にし、E2E は最小限にする。
- 特性テスト: 現行の振る舞いを固定する。画面は Playwright、帳票は PDF 比較、
  DB はスナップショット比較。
- ゴールデンマスター: 入力と出力の組を大量に用意し、移行前後で比較する。
- コントラクトテスト: 新旧サービス間は Pact で契約を固定する。
- シャドーリード: 新旧に同じリクエストを流し、差分を比較する。
- 性能テスト: 移行前にベースラインを取得し、移行後に比較する。
- flaky テストは放置しない。隔離するか、原因を直す。

### レビュー

- 小さな PR、1 PR 1 論理変更。
- レビュー観点: 振る舞いの同一性、Jakarta 化、SQL、トランザクション、
  セキュリティ、運用。
- 移行専用のチェックリストを用意し、PR テンプレートに埋め込む。
- AI レビューは補助。最終ゲートはコンパイル、テスト、静的解析。

## opencode の活用

### 向く作業

- 影響範囲調査: LSP / Serena でシンボル参照、grep で設定・SQL・JSP を追う。
  外部の移行支援に頼る前に、コードベースの理解を opencode で加速する。
- 変換の下書き: dicon → Java Config、S2Dao SQL → Doma SQL、Excel → Markdown。
- テスト生成: 特性テスト、JUnit 5 移行、Testcontainers の雛形。
- レビュー: arch-reviewer、security-reviewer、ops-reviewer の並列実行。
- ドキュメント: ADR、Runbook、移行計画、チェックリスト。
- 差分説明: 大量のログ・差分の要約。

### 向かない作業

- 業務仕様の最終判断。
- 本番環境の操作。
- ライセンス・契約の判断。
- テストなしの一括変換。
- 業務上の移行優先順位の最終決定。決定権は人間に残す。

### 設定

- `AGENTS.md`: ビルド、テスト、lint、禁止事項、アーキテクチャ境界。
- スキル: `tech-selection`、`test-spec`、`doc-create`、`review-flow`、`cross-review`。
- サブエージェント: `arch-reviewer`、`security-reviewer`、`ops-reviewer`、
  `test-author`。
- モデル: 探索・変換は flash、設計判断・レビューは pro。
- MCP: context7（Spring / EAP など）、deepwiki（OSS）、Serena（LSP）。

### ガードレール

- 生成物は必ずコンパイル、テスト、差分で検証する。AI の出力を信じない。
- 秘密情報と個人情報を送らない。Excel はマスキングしてから渡す。
- 外部サービスの契約とデータポリシーを確認する。閉域要件がある場合は
  ローカル LLM を優先する。
- 外部由来の設計書やログを信頼しない。プロンプトインジェクションを想定する。
- 変更は小さく、PR 単位で行う。一括変換はレビュー不能になる。
- AI の指摘は仮説として扱い、現行コードとテストで事実確認する。
- コストはキャッシュ、モデル別枠、サブエージェントの並列数で制御する。

## ロードマップ

| Phase | 期間 | 目的 | 成果物 | 出口条件 |
| --- | --- | --- | --- | --- |
| 0 | 2-4 週 | 現状可視化 | 依存関係、画面・バッチ・帳票一覧、ビルド再現、リスク表 | CI でビルドが通る |
| 1 | 1-2 ヶ月 | 足場固め | CI、テスト基盤、ドキュメント基盤、AGENTS.md | 主要画面の特性テストがある |
| 2 | 1-2 ヶ月 | 延命と隔離 | EAP 7.4 ELS、Seasar2 凍結、新スタックの雛形 | 新規機能が新スタックで出る |
| 3 | 6-18 ヶ月 | 段階移行 | ストラングラー、DB 移行、画面移行 | 変更頻度上位の機能が移行済み |
| 4 | 3-6 ヶ月 | 退役と移管 | EAP 退役、運用移管、Runbook | 旧系の停止、運用が回る |

- 最初の対象は、変更が多く、テストで振る舞いを固定しやすい画面・バッチ・帳票。
- クリティカルでも変更が少ない機能は後回しにし、延命期間でカバーする。
- 各 Phase のゲートは、テスト、性能、セキュリティ、カットオーバー判定。
- ELS の上限 2030-12-31 から逆算し、Phase 3 の遅延を許容しない。

## 決定事項（2026-10-27）

- アプリケーション基盤は Spring Boot 4.x（Java 25 LTS、独立実行）とする。
- O/R マッパーは Doma 3 とする。
- Excel 設計書は Markdown を SSOT とし、Excel は生成物または移行期の入力に限定する。
- 移行はストラングラー方式とし、JBoss EAP は移行期間のみ維持する。
- JDK は Azul Zulu 25（無償版、2033年9月まで）を採用する。
  Amazon Corretto 25 は CI の互換性検証に使う。
- Java SE 標準 API と標準 JVM オプションに限定し、JDK 固有 API は使わない。
- 有償サポートは必須とせず、本番で SLA が必要になった時点で検討する。
- 移行の設計・実装は内製を主体とし、外部には JDK の専門知識と
  短期のレビューを求める。常駐型の移行支援は前提としない。
- 詳細と代替案の却下理由は
  [ADR-002](../adr/ADR-002-spring-boot-doma3-excel-to-markdown.md) に記録した。

### 未決事項

- EAP 8 を移行期間中に併用するか、EAP 7.4 ELS のみで進めるか。
- 画面を Thymeleaf で統一するか、React を併用するか。
- 帳票を JasperReports にするか、Excel テンプレートを維持するか。
- 有償サポートの契約要否と、契約する場合のベンダー。
- 外部の移行支援をスポットで入れるか、内製だけで進めるか。
- オンプレミスかクラウドか。
- 内製かベンダーか。

## リスク

| リスク | 影響 | 対策 |
| --- | --- | --- |
| Seasar2 の暗黙仕様 | 移行漏れ | dicon / SQL / JSP の静的解析、特性テスト、業務ヒアリング |
| テスト不在 | 回帰 | ゴールデンマスター、シャドーリード、段階リリース |
| Excel の暗黙仕様 | 仕様漏れ | 棚卸し、突合、業務担当レビュー |
| 一括移行 | 失敗 | ストラングラー、機能フラグ、カナリア |
| ELS 期限 | サポート切れ | 2030 年から逆算したロードマップ |
| AI 変換の誤り | 障害 | コンパイル、テスト、差分、人間レビュー |
| 人材不足 | 停滞 | ペア作業、ドキュメント化、外部支援 |
| JDK ベンダーの方針変更 | セキュリティ更新の停止 | 標準 API に限定し、Zulu と Corretto を切り替え可能にする |
| JDK 固有機能の不足 | 機能制約 | JavaFX / Native Image / CRaC / FIPS は必要時に別途検討する |
| ライセンス | コスト | JDK / EAP の契約確認 |

## 参照

- [Seasar Project](https://www.seasar.org/)
- [Doma 概要](https://doma.seasar.org/overview.html)
- [Doma GitHub](https://github.com/domaframework/doma)
- [JBoss EAP Product Life Cycles](https://access.redhat.com/product-life-cycles?product=Red+Hat+JBoss+Enterprise+Application+Platform)
- [JBoss EAP 8 Supported Configurations](https://access.redhat.com/articles/6961381)
- [JBoss EAP Supported Standards](https://access.redhat.com/articles/113373)
- [What's new in JBoss EAP 8.0](https://developers.redhat.com/articles/2024/02/05/whats-new-jboss-enterprise-application-platform-8)
- [EAP 8 Installation Methods](https://docs.redhat.com/en/documentation/red_hat_jboss_enterprise_application_platform/8.0/html/red_hat_jboss_enterprise_application_platform_installation_methods/assembly_adding-feature-packs-in-your-jboss-eap-installation-using-the-jboss-eap-installation-manager_default)
- [Oracle Java SE Support Roadmap](https://www.oracle.com/java/technologies/java-se-support-roadmap.html)
- [Spring Boot System Requirements](https://docs.spring.io/spring-boot/system-requirements.html)
- [Azul Java Support Roadmap](https://www.azul.com/products/azul-support-roadmap/)
- [Amazon Corretto FAQs](https://aws.amazon.com/corretto/faqs/)
- [Adoptium Temurin Support](https://adoptium.net/support/)
- [Spring Framework 6 baseline](https://spring.io/blog/2021/09/02/a-java-17-and-jakarta-ee-9-baseline-for-spring-framework-6)
- [Jakarta EE 11](https://jakarta.ee/release/11)
- [OpenRewrite Jakarta recipes](https://docs.openrewrite.org/recipes/java/migrate/jakarta/javaxresourcetojakartaresource)
- [Strangler Fig Application](https://martinfowler.com/bliki/StranglerFigApplication.html)
- [Parallel Change](https://martinfowler.com/bliki/ParallelChange.html)
- [Testcontainers](https://testcontainers.com/)
- [OpenTelemetry](https://opentelemetry.io/)
- [opencodego モデルルーティング調査報告](../reports/2026-10-25-opencodego-model-routing.md)
- [ADR-002: Spring Boot + Doma 3 への移行と Excel 設計書の Markdown 化](../adr/ADR-002-spring-boot-doma3-excel-to-markdown.md)
