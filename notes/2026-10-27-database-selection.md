---
type: Note
title: "移行先データベースの選定と最新トレンド"
description: "オープンソース DB の最新トレンドを整理し、レガシー Java の移行先として PostgreSQL を第一候補にする根拠を示す。"
tags:
  - database
  - postgresql
  - open-source
  - java
  - modernization
generated:
  by: opencode
  at: "2026-10-27T22:55:19+09:00"
status: "draft"
---

## 結論

- 新規サービスは Oracle Database 26AI Free を第一候補とする。
- 既存の Oracle 18c を維持し、Oracle 26AI Free に移行する。
- Oracle Free の制約（2 CPU、2GB RAM、12GB ユーザーデータ、サポートなし）を
  許容できる場合に限る。
- 現行の Oracle 18c Free で十分に処理できているため、26AI Free の
  2 CPU、2GB RAM でも性能上の問題は想定しない。
- パフォーマンスは優先度の高い要件ではない。移行前後のベースライン比較だけ行う。
- 制約を超える場合は、Oracle 26AI Enterprise Edition か PostgreSQL を検討する。
- PostgreSQL は、Oracle のライセンスや制約が合わない場合の代替とする。

## 最新トレンド

- Stack Overflow Developer Survey 2026 では、PostgreSQL が 57.9% で 1 位。
  SQLite 42.1%、MySQL 35.1%、Redis 28.3%、SQL Server 27.1%、
  MariaDB 20.3%、MongoDB 19.0%、Oracle 9.4% が続く。
- DB-Engines 2026年10月では、Oracle、MySQL、SQL Server、PostgreSQL の順。
  ただし PostgreSQL は前月比 +5.50、前年比 +45.56 で成長率が最も高い。
- PostgreSQL 19 は 2026年10月15日に RC1、2026年11月19日に GA 予定。
- ベクター検索は pgvector が PostgreSQL に統合されている。
  専用 DB は Milvus、Qdrant、Weaviate、Chroma がある。
- 分析用途は DuckDB と ClickHouse が伸びている。
- 分散 SQL は CockroachDB、TiDB、YugabyteDB が候補。
- サーバーレス PostgreSQL は Neon、Supabase がある。

## 比較

| DB | ライセンス | 強み | 弱み | 用途 |
| --- | --- | --- | --- | --- |
| Oracle 26AI Free | Oracle Free Use Terms | PL/SQL、AI Vector Search、JSON、Graph | 2 CPU、2GB RAM、12GB、サポートなし | 現行資産の継続 |
| PostgreSQL | PostgreSQL License | 機能、拡張性、人気、クラウド対応 | 運用ノウハウが必要 | 汎用、新規 |
| MySQL | GPLv2 | Web アプリの実績、シンプル | 拡張性は PostgreSQL に劣る | Web アプリ |
| MariaDB | GPLv2 | MySQL 互換、オープン | エコシステムは MySQL に劣る | MySQL 代替 |
| CockroachDB | BSL | 分散 SQL、高可用性 | ライセンスと運用コスト | 大規模分散 |
| TiDB | Apache 2.0 | HTAP、分散 SQL | 運用が複雑 | 大規模、分析 |
| YugabyteDB | Apache 2.0 | 分散 SQL、PostgreSQL 互換 | 成熟度 | 分散 |
| SQLite | Public Domain | 組み込み、軽量 | サーバー DB ではない | 組み込み、テスト |

## レガシー Java との相性

- Doma 3、MyBatis、Spring Data JDBC は PostgreSQL に対応している。
- JDBC ドライバは pgjdbc を使う。
- SQL 方言は標準 SQL に寄せ、PostgreSQL 固有機能は局所化する。
- Oracle からの移行では、PL/SQL、シーケンス、日付型、NULL の扱い、
  ページング、照合順序の違いを確認する。
- Flyway でスキーマを管理し、Testcontainers で PostgreSQL を再現する。

## 推奨

- 新規サービスは Oracle Database 26AI Free を使う。
- 既存の Oracle 18c の PL/SQL 資産を活かす。
- 12GB のデータ制限を監視し、超える前に Oracle 26AI EE か PostgreSQL へ移す。
- サポートが必要なら Oracle 26AI EE か Oracle Cloud を選ぶ。
- PostgreSQL は、Oracle のライセンスや制約が合わない場合の代替とする。
- 分析用途は DuckDB か ClickHouse を別途検討する。

## 移行の注意

- Oracle 18c から 26AI へのアップグレードは、AutoUpgrade か Data Pump を使う。
- PL/SQL はまず現行のまま移し、特性テストで固定してから Java へ移す。
- Oracle 固有の SQL（ROWNUM、CONNECT BY、DECODE、NVL など）は、
  26AI でもそのまま使える。標準 SQL への書き換えは急がない。
- 非推奨機能と初期化パラメータを確認する。
- シーケンスと ID 生成の方式を確認する。
- 日付型、タイムゾーン、NULL の扱いを確認する。
- ページングの SQL を書き換える。
- 文字コードと照合順序を UTF-8 に統一する。
- データ型の桁数と丸めを確認する。
- 性能は移行前にベースラインを測る。

## QA に出す条件

- 既存 DB の種類とバージョンは何か。
- ライセンス契約はどうなっているか。
- データ量と成長率はどれくらいか。
- ストアドプロシージャや独自拡張を使っているか。
- クラウドかオンプレか。
- 運用体制とバックアップ方式はどうなっているか。

## 参照

- [Oracle AI Database Free](https://www.oracle.com/database/free/)
- [Oracle AI Database Free Licensing Restrictions](https://docs.oracle.com/en/database/oracle/oracle-database/26/xeinl/licensing-restrictions.html)
- [Stack Overflow Developer Survey 2026: Databases](https://survey.stackoverflow.co/2026/technology/data/databases)
- [DB-Engines Ranking](https://db-engines.com/en/ranking)
- [PostgreSQL 19 RC 1 Released](https://www.postgresql.org/about/news/postgresql-19-rc-1-released-3409)
- [pgvector](https://github.com/pgvector/pgvector)
- [Flyway](https://flywaydb.org/)
- [Doma](https://github.com/domaframework/doma)
