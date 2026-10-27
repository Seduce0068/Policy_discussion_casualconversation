---
type: Note
title: "レガシー Java モダナイズの決定事項チェックリスト"
description: "Spring Boot + Doma 3 + Markdown 化を前提に、移行プロジェクトで決めるべき技術・データ・運用の項目を優先度別に整理する。"
tags:
  - modernization
  - checklist
  - java
  - spring-boot
  - operations
generated:
  by: opencode
  at: "2026-10-27T19:47:36+09:00"
status: "draft"
---

## 目的

- Spring Boot + Doma 3 + Markdown 化の方針は決まった。
- ここでは、移行の設計と運用に影響する残りの決定事項を、優先度別に整理する。
- すべてを同時に決めない。設計の前提になるものから順に決める。

## 決定済み

| 項目 | 決定 |
| --- | --- |
| アプリケーション基盤 | Spring Boot 4.x |
| Java | Java 25 LTS |
| JDK | Azul Zulu 25（無償版） |
| O/R マッパー | Doma 3 |
| 設計書 | Markdown を SSOT |
| 移行方式 | ストラングラー |
| 移行主体 | 内製。外部は短期レビュー |
| 有償サポート | 必須としない。本番で SLA が必要になった時点で検討 |

## 今すぐ決める（設計の前提）

| 項目 | 選択肢 | 推奨 | 理由 |
| --- | --- | --- | --- |
| DB | 既存維持 / PostgreSQL | 既存維持を基本 | DB 移行はリスクが大きい。新規サービスだけ PostgreSQL も可 |
| ビルドツール | Maven / Gradle | Maven | 既存資産と Spring Boot の標準。Wrapper で再現性を確保 |
| DB マイグレーション | Flyway / Liquibase | Flyway | SQL ベースで差分が読みやすく、レビューしやすい |
| 認証・認可 | 既存 LDAP/AD / OIDC | 既存基盤を Spring Security で接続 | 認証基盤の変更は全画面に影響する |
| セッション | Redis / JDBC / インメモリ | 要件で判断 | 水平分散なら Redis、単一ノードなら JDBC |
| 画面 | Thymeleaf / React | 画面単位で判断 | 変更頻度と複雑さで決める。一括 SPA 化はしない |
| 帳票 | JasperReports / Excel テンプレート | 帳票の複雑さで判断 | 既存 Excel 資産を活かすならテンプレートを維持 |
| バッチ | Spring Batch / 既存基盤 | Spring Batch | ジョブ定義の外部化、再実行、進捗管理 |
| アーキテクチャ | レイヤード / モジュラモノリス | モジュラモノリス | 段階移行の境界を明確にし、後で分離しやすくする |
| CI/CD | GitHub Actions / 既存 | GitHub Actions | 再利用可能ワークフローでゲートを共通化 |
| コンテナ | Docker / Podman | 環境に合わせる | オンプレは Podman、クラウドは Docker も可 |

## 移行設計で決める（データと業務）

| 項目 | 決めること | 推奨 |
| --- | --- | --- |
| 文字コード | UTF-8 に統一するか、既存の Windows-31J を残すか | UTF-8。既存データは変換時に件数と文字化けを検証 |
| タイムゾーン | UTC 保存か、Asia/Tokyo 保存か | UTC 保存、表示時に Asia/Tokyo へ変換 |
| 排他制御 | 楽観ロック / 悲観ロック | 楽観ロックを基本。競合が多い画面だけ悲観ロック |
| 監査ログ | 誰が、いつ、何を、どこから | 構造化ログと DB 監査テーブルを併用 |
| 外部 IF | SOAP / REST / ファイル | 既存を維持し、新規は REST。SOAP は Spring Web Services |
| データ移行 | NULL、日付、暗号化、桁数、丸め | 移行リハーサルで件数とサンプルを突合 |
| 個人情報 | マスキング、保持期間、削除 | ログと画面でマスキング。保持期間を先に決める |
| 性能 | ベースライン、目標値、負荷試験 | 移行前に現行のベースラインを測る |

## 運用前に決める（運用と体制）

| 項目 | 決めること | 推奨 |
| --- | --- | --- |
| 監視 | メトリクス、ログ、トレース | Prometheus + Grafana + Loki + Tempo |
| SLO | 可用性、レイテンシ、エラー率 | 業務影響から逆算する |
| バックアップ | 頻度、保持期間、リストア訓練 | 日次バックアップと月次リストア訓練 |
| DR | RTO / RPO | 業務要件から決める。数字を先に合意する |
| リリース | ローリング / ブルーグリーン / カナリア | ローリングを基本。DB 変更は Expand-Contract |
| オンコール | 体制、エスカレーション、Runbook | 当番制と Runbook の整備 |
| 予算 | クラウド / オンプレ、ライセンス | 3 年 TCO で比較。EAP の ELS も含める |
| 体制 | 内製 / ベンダー、教育 | 内製 + スポットレビュー。属人化を避ける |

## 決め方

- 決定には期限を付ける。期限のない保留は決めないのと同じ。
- 決定は ADR に記録する。選択肢、理由、却下理由、影響を残す。
- 業務判断は業務担当、技術判断は技術担当が持つ。
- opencode は選択肢と根拠を出すが、決定は人間が行う。
- 決められない項目は「保留」と「再検討の日付」をセットにする。

## 参照

- [レガシー Java のモダナイズ方針](2026-10-27-legacy-java-seasar2-jboss-modernization.md)
- [ADR-002: Spring Boot + Doma 3 への移行と Excel 設計書の Markdown 化](../adr/ADR-002-spring-boot-doma3-excel-to-markdown.md)
- [Spring Boot System Requirements](https://docs.spring.io/spring-boot/system-requirements.html)
- [Flyway](https://flywaydb.org/)
- [Spring Modulith](https://spring.io/projects/spring-modulith)
