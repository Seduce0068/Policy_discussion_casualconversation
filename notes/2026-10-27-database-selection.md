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

- 新規サービスは PostgreSQL を第一候補とする。
- 既存 DB は維持を基本とし、移行する場合も PostgreSQL を選ぶ。
- 2026年10月時点の最新安定版は PostgreSQL 18。
- PostgreSQL 19 は 2026年11月19日に GA 予定。GA 後に検証して採用する。

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

- 新規サービスは PostgreSQL 18 を使う。
- PostgreSQL 19 は GA 後に検証し、問題がなければ採用する。
- 既存 DB は維持を基本とする。DB 移行はリスクが大きいため、
  新規サービスと変更の多い機能から PostgreSQL に寄せる。
- マネージドサービスを選ぶ。AWS Aurora PostgreSQL、Google Cloud SQL、
  Google AlloyDB、Azure Database for PostgreSQL Flexible Server が候補。
- ベクター検索が必要なら pgvector を使う。
- 分析用途は DuckDB か ClickHouse を別途検討する。

## 移行の注意

- PL/SQL などのストアドプロシージャは、Java か SQL に書き換える。
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

- [Stack Overflow Developer Survey 2026: Databases](https://survey.stackoverflow.co/2026/technology/data/databases)
- [DB-Engines Ranking](https://db-engines.com/en/ranking)
- [PostgreSQL 19 RC 1 Released](https://www.postgresql.org/about/news/postgresql-19-rc-1-released-3409)
- [pgvector](https://github.com/pgvector/pgvector)
- [Flyway](https://flywaydb.org/)
- [Doma](https://github.com/domaframework/doma)
