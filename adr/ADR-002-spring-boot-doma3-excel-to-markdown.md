---
type: ADR
title: "レガシー Java を Spring Boot + Doma 3 へ移行し、Excel 設計書を Markdown 化する (ADR-002)"
description: "Seasar2/JBoss EAP 資産の移行先を Spring Boot 4.x + Doma 3 に決定し、Excel 設計書を Markdown を SSOT とする docs-as-code へ移行する。"
tags:
  - adr
  - java
  - spring-boot
  - doma
  - docs-as-code
generated:
  by: opencode
  at: "2026-10-27T18:50:12+09:00"
status: "approved"
---

## ADR-002: Spring Boot + Doma 3 への移行と Excel 設計書の Markdown 化

### 状態

承認済み (Approved)。2026-10-27 に採用を決定。

### 背景

- 既存システムは Seasar2（シーサー2）と JBoss EAP 7.x、Excel 設計書で構成されている。
- Seasar2 は 2016-09-26 に EOL。最終リリースは 2.4.48（2014-12）。
- JBoss EAP 7.x はフルサポートが 2023-12-31 に終了し、メンテナンスも
  2025-06-30 に終了している。ELS の上限は 2030-12-31。
- Java 25 LTS は 2025-09-16 にリリースされた最新の LTS。
- JBoss EAP 8 は Jakarta EE 10 で、`javax.*` から `jakarta.*` へ namespace が変わる。
  Seasar2 をそのまま載せることはできない。
- Excel 設計書はバイナリで差分が読めず、レビュー、検索、自動化、監査の阻害要因になる。
- 検討の全体像は
  [レガシー Java のモダナイズ方針](../notes/2026-10-27-legacy-java-seasar2-jboss-modernization.md)
  に記録した。

### 決定

1. 移行先のアプリケーション基盤は Spring Boot 4.x とする。
2. 実行環境は Java 25 LTS を基本とし、独立実行（埋め込み Tomcat）を前提とする。
   EAP 8 に残す部分は Java 21 とする。
3. O/R マッパーは Doma 3 とする。
4. Excel 設計書は Markdown を SSOT とし、Excel は生成物または移行期の入力に限定する。
5. 移行はストラングラー方式で行い、JBoss EAP は移行期間のみ維持する。
6. 新規機能は最初から Spring Boot + Doma 3 で実装する。
7. ドキュメントは docs-as-code とし、CI でリンク、構造、生成物の差分を検証する。
8. JDK ディストリビューションは Azul Zulu 25 を第一候補とし、
   Amazon Corretto 25 を代替とする。
9. 有償サポートは必須としない。本番で SLA が必要になった時点で検討する。
10. 移行の設計・実装は内製を主体とし、外部には JDK の専門知識と
    短期のレビューを求める。常駐型の移行支援は前提としない。

### 理由

#### Spring Boot を選ぶ理由

- Java 17 以上を前提とする標準技術で、Jakarta EE 11 に対応する。
  Spring Boot 4.1 は Java 27 まで対応し、Java 25 LTS を利用できる。
  Spring Boot 3.x を使う場合は Jakarta EE 10 の EAP 8 とも組み合わせられる。
- 埋め込み Tomcat で独立実行でき、EAP のサブスクリプションと運用から解放される。
- Spring MVC、Spring Batch、Spring Security、Actuator、Micrometer など、
  必要な機能が揃っている。
- 人材、書籍、クラウド対応、セキュリティ更新のエコシステムが広い。
- テストは JUnit 5、Testcontainers、Playwright で標準化できる。

#### Doma 3 を選ぶ理由

- S2Dao の DAO パターンと 2-way SQL を継承しており、SQL 資産を再利用しやすい。
- Seasar2 に依存せず、Spring Boot と組み合わせられる。
- Java 17 以上。コンパイル時に SQL ファイルの存在とコメント文法を検証する。
- 2-way SQL は SQL 単体で実行でき、DB 担当と開発担当の分業に向く。
- Spring Boot 連携は `doma-spring-boot-starter` 3.0.0 が提供されている。
  要件は Java 17 以上、Doma 3.x 以上、Spring Boot 4.x 以上。

#### Excel を Markdown 化する理由

- Git で差分、レビュー、履歴管理ができる。
- 全文検索、リンク、CI 検証ができる。
- LLM と opencode に渡しやすく、変換、要約、レビューの品質が安定する。
- GitHub、opencode、静的サイト生成との相性がよい。
- Excel は配布用ビューとして生成できるため、現場の閲覧手段は残せる。

#### JDK ディストリビューションを選ぶ理由

- 無償でセキュリティ更新が提供される期間を比較した。

| JDK | 無償のセキュリティ更新 | 期限 | 備考 |
| --- | --- | --- | --- |
| Azul Zulu 25 | あり | 2033年9月 | 無償版は商用サポートを含まない |
| Amazon Corretto 25 | あり | 2032年10月 | AWS の本番実績がある |
| Eclipse Temurin 25 | あり | 2031年9月 | コミュニティ中立 |
| Microsoft Build of OpenJDK 25 | あり | 2030年9月 | Azure 向け |
| Oracle JDK 25 | 制限あり | 2028年9月 | 無償利用は個人・開発向け |
| BellSoft Liberica 25 | 要確認 | 商用は 2034年3月 | 無償版の期間が不明確 |

- Azul Zulu 25 を第一候補とする。無償期間が最も長く、主要プラットフォームを
  サポートする。無償版はセキュリティ更新を含むが、商用サポートは含まない。
- Amazon Corretto 25 を代替とする。無償期間は Zulu より約 1 年短いが、
  AWS の本番環境での実績があり、ドロップイン置換が可能。
- Eclipse Temurin 25 はコミュニティ中立性が高いが、無償期間は 2031年9月まで。
- Oracle JDK 25 は無償更新が 2028年9月までで短く、採用しない。
- ベンダーロックを避けるため、標準 API に限定し、CI で Zulu と Corretto の
  マトリクスを回す。
- 有償サポートは、セキュリティパッチの優先提供、JDK の障害調査、
  JDK の移行支援、SLA を提供する。アプリケーションの移行設計や
  業務仕様の理解は含まれない。
- JDK ベンダーはアプリケーションの内部構造を知らないため、移行の主体は
  内製とする。ベンダーには JDK のピンポイントの疑問と障害対応を求める。
- 個人・小規模の開発では無償の Zulu / Corretto で足りる。本番で SLA や
  ベンダーの保証が必要になった時点で契約する。

### 代替案と却下理由

| 代替案 | 却下理由 |
| --- | --- |
| EAP 8 + Jakarta EE 標準（CDI / JAX-RS / JSF） | EAP のライセンスと運用が重い。Spring エコシステムの方が人材・ライブラリ・クラウド対応が広い |
| MyBatis | 2-way SQL の資産再利用とコンパイル時検証で Doma 3 が優位。動的 SQL が多用される箇所は個別に判断する |
| Excel を正のまま残す | 差分、レビュー、検索、自動化、監査の阻害が解消しない |
| AsciiDoc | 表の表現力は高いが、ツールチェーンと GitHub / opencode との相性で Markdown を優先する |
| 手作業で Markdown 化 | 量が多く再現性がない。自動変換と人間レビューを組み合わせる |

### 結果

#### 良い面

- 標準技術に寄り、人材確保とクラウド移行が容易になる。
- EAP のサブスクリプションと ELS のコストを削減できる。
- CI/CD、テスト、可観測性を標準化できる。
- 設計書の差分レビューと検索ができる。
- opencode による変換、レビュー、テスト生成がしやすい。

#### 悪い面

- EAP の運用資産（CLI、監視、パッチ）を捨てるコストが発生する。
- Spring Boot のバージョンアップ追従が必要。
- Doma 3 の学習コストが発生する。
- Excel 変換の初期コストが大きい。
- 移行期間は EAP と Spring Boot の二重運用になる。

#### リスクと対策

| リスク | 影響 | 対策 |
| --- | --- | --- |
| Seasar2 の暗黙仕様 | 移行漏れ | 特性テスト、dicon / SQL / JSP の静的解析、業務ヒアリング |
| Excel の暗黙仕様 | 仕様漏れ | 棚卸し、突合、業務担当レビュー |
| 移行の長期化 | ELS 期限超過 | ストラングラー、機能フラグ、カナリア、2030 年からの逆算 |
| Doma 3 の習熟 | 品質低下 | パイロットで 1 画面・1 バッチを実装し、型と規約を固める |
| Spring Boot の追従 | 脆弱性・陳腐化 | Renovate、LTS 基準、四半期の更新 |
| JDK ベンダーの方針変更 | セキュリティ更新の停止 | 標準 API に限定し、Zulu と Corretto を切り替え可能にする |
| Excel 変換の誤り | 仕様漏れ | 行数・キー・サンプル値の突合、人間レビュー、差分ゼロ検証 |

### 実装方針

#### Spring Boot

- Java 25 LTS。Spring Boot 4.1 系を基本とする。
- JDK は Azul Zulu 25 を既定とし、CI で Amazon Corretto 25 も検証する。
- Maven / Gradle の Toolchains で JDK を切り替え可能にする。
- EAP 8 に残す部分は Java 21 とする。EAP 8 のサポート構成は JDK 17/21。
- Spring MVC + Thymeleaf。画面は段階移行し、JSP は温存しない。
- Spring Batch。バッチはジョブ定義を外部化する。
- Spring Security。OIDC と LDAP を標準化する。
- Actuator、Micrometer、OpenTelemetry。ヘルスチェックとメトリクスを最初から入れる。
- テストは JUnit 5、Testcontainers、Playwright、Pact。
- EAP 8 への WAR デプロイは不可とする。Spring Boot 4.1 は Servlet 6.1 以上を
  要求するが、EAP 8 は Servlet 6.0（Jakarta EE 10）である。
  EAP 8 に載せる必要がある場合は、Spring Boot 3.x と Jakarta EE 10 の
  組み合わせを別途検証する。

#### Doma 3

- `doma-spring-boot-starter` 3.0.0 と Doma 3.x の最新安定版を使う。
- `@Dao` インタフェースと 2-way SQL で実装する。
- `doma-processor` でコンパイル時にコード生成と検証を行う。
- SQL ファイルは次の規約に置く。

```text
src/main/resources/META-INF/<DaoのFQCN>/<メソッド名>.sql
```

- トランザクションは Spring の `@Transactional` を使う。
- テストは Testcontainers で実 DB に近い環境を使う。
- 動的 SQL が複雑で 2-way SQL に馴染まない箇所は、Doma の DSL か
  MyBatis の併用を個別に判断する。既定は Doma 3 とする。

#### Excel → Markdown

- 変換ツールは Rust の `calamine` で実装する。仮称 `xlsx2md`。
- 入力は `docs/design/*.xlsx`、出力は `docs/design/*.md` とする。
- シートごとに 1 ファイル、または 1 シート 1 セクションに分割する。
- セル結合は結合範囲を取得し、左上の値を範囲に展開して分割する。
- 数式は計算結果の値を使う。計算ロジックはコードへ移す。
- 画像は抽出しない。必要な場合は元 Excel を参照する。
- 出力の front matter に元ファイル、シート名、変換日時、ツールのバージョンを記録する。
- CI で `xlsx2md --check` を実行し、生成物の差分ゼロを検証する。
- 変換後は Excel を読み取り専用にし、Markdown を正とする。
- 移行期に Excel 編集が必要な領域は、`xlsx2csv` で差分を見えるようにしてから移す。

### 移行計画

| Phase | 内容 | 出口条件 |
| --- | --- | --- |
| 0 | 現状可視化、ビルド再現、依存関係の棚卸し | CI でビルドが通る |
| 1 | パイロット（1 画面、1 バッチ、1 帳票）、Excel → Markdown の変換 | 特性テスト、PDF 一致、`xlsx2md --check` が通る |
| 2 | 新規機能を Spring Boot で実装、EAP 7.4 ELS で延命 | 新規機能が本番で動く |
| 3 | 変更頻度の高い機能をストラングラーで移行 | 上位機能が移行済み |
| 4 | EAP 退役、運用移管 | 旧系停止、Runbook 整備 |

- パイロットの成功条件は、振る舞いの同一性、性能、セキュリティ、カットオーバー判定。
- 各 Phase の終了時に ADR を見直し、決定を更新する。
- ELS の上限 2030-12-31 から逆算し、Phase 3 の遅延を許容しない。

### 成功条件

- パイロットの画面が、特性テストで既存と同じ振る舞いを示す。
- パイロットの帳票が、PDF のスナップショットで既存と一致する。
- Excel から変換した Markdown が、元 Excel の行数、キー、サンプル値と一致する。
- CI ゲート（compile、test、静的解析、セキュリティ）が通る。
- 性能が移行前のベースライン内に収まる。

### 未決事項

- EAP 8 を移行期間中に併用するか、EAP 7.4 ELS のみで Spring Boot に全面移行するか。
- オンプレミスかクラウドか。
- 内製かベンダーか。
- 画面を Thymeleaf で統一するか、React を併用するか。
- 帳票を JasperReports にするか、Excel テンプレートを維持するか。
- 有償サポートの契約要否と、契約する場合のベンダー。
- 外部の移行支援をスポットで入れるか、内製だけで進めるか。

### 参照

- [レガシー Java のモダナイズ方針](../notes/2026-10-27-legacy-java-seasar2-jboss-modernization.md)
- [Seasar Project](https://www.seasar.org/)
- [Doma 概要](https://doma.seasar.org/overview.html)
- [Doma GitHub](https://github.com/domaframework/doma)
- [doma-spring-boot](https://github.com/domaframework/doma-spring-boot)
- [Spring Boot](https://spring.io/projects/spring-boot)
- [Spring Boot System Requirements](https://docs.spring.io/spring-boot/system-requirements.html)
- [Spring Framework 6 baseline](https://spring.io/blog/2021/09/02/a-java-17-and-jakarta-ee-9-baseline-for-spring-framework-6)
- [JBoss EAP Product Life Cycles](https://access.redhat.com/product-life-cycles?product=Red+Hat+JBoss+Enterprise+Application+Platform)
- [Azul Java Support Roadmap](https://www.azul.com/products/azul-support-roadmap/)
- [Amazon Corretto FAQs](https://aws.amazon.com/corretto/faqs/)
- [Adoptium Temurin Support](https://adoptium.net/support/)
- [Microsoft Build of OpenJDK Support](https://learn.microsoft.com/en-us/java/openjdk/support)
- [calamine](https://github.com/tafia/calamine)
