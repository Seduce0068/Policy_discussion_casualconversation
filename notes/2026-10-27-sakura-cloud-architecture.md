---
type: Note
title: "さくらのクラウドを前提とした移行アーキテクチャ"
description: "さくらのクラウドの VM、DB アプライアンス、AppRun、オブジェクトストレージを使った移行構成と制約を整理する。"
tags:
  - sakura-cloud
  - architecture
  - postgresql
  - spring-boot
  - operations
generated:
  by: opencode
  at: "2026-10-27T23:00:39+09:00"
status: "draft"
---

## 前提

- クラウドはさくらのクラウドを使う。
- アプリケーションは Spring Boot、DB は PostgreSQL を基本とする。
- リージョンは東京、大阪、石狩のいずれかとする。
- 既存のオンプレ資産からの移行を想定する。

## 構成案

### 小規模: VM + DB アプライアンス

- アプリは VM 上で Docker か Podman を動かす。
- DB はさくらのクラウドのデータベースアプライアンスを使う。
- ロードバランサーで TLS を終端する。
- エビデンスとバックアップはオブジェクトストレージに置く。
- 監視は Prometheus と Grafana を VM に載せる。

### 中規模: 自前 Kubernetes + CloudNativePG

- Kubernetes は RKE2 などを VM 上に構築する。
- PostgreSQL は CloudNativePG で運用する。
- オブジェクトストレージにバックアップを置く。
- 監視は Prometheus Operator と Grafana を使う。
- 運用負荷は高いが、PostgreSQL 18 など新しいバージョンを使える。

### サーバーレス: AppRun

- コンテナイメージをさくらのクラウドのコンテナレジストリに登録する。
- AppRun にデプロイする。
- DB はデータベースアプライアンスを使う。
- 小規模な API やバッチに向く。

## 重要な制約

- データベースアプライアンスの PostgreSQL は 15.18（2026年7月23日更新）。
  PostgreSQL 16、17、18 は提供されていない。
- 2027年11月は PostgreSQL コミュニティの PostgreSQL 15 の EOL。
  さくらのクラウドのサポート期限は別途確認が必要。
- バージョン自動更新機能は MariaDB のみ。PostgreSQL は自動更新の対象外。
- PostgreSQL のメジャー・マイナーバージョンはアプライアンス作成時に固定され、
  利用中に変更できない。メジャーバージョンの更新は、新規アプライアンスを作成し、
  データを移行する必要がある。
- さくらのクラウドは 2026年7月に PostgreSQL 14 から 15 に上げたばかりで、
  バージョンアップのペースは遅い。次のメジャーバージョンの提供時期は未定。
- PostgreSQL 18 を使う場合は、VM に自前で構築するか、Kubernetes 上で
  CloudNativePG を運用する。
- AppRun は Kubernetes と Knative をベースにしたコンテナ実行基盤。
  Operator を自由に導入する用途には向かない。
- オブジェクトストレージは Amazon S3 互換 API を提供する。
  ただし、バケットの作成と削除は S3 互換 API ではできない。
- Terraform は `sacloud/sakuracloud` プロバイダーを使う。
- 監視とログは自前で構築する前提とする。

## 推奨

- まずは小規模構成で始める。VM、DB アプライアンス、ロードバランサー、
  オブジェクトストレージの組み合わせ。
- アプリは Docker か Podman で動かし、GitHub Actions からデプロイする。
- DB はデータベースアプライアンスの PostgreSQL 15 を使うか、VM に PostgreSQL 18 を
  自前で構築するかを、運用体制とサポート期限で決める。
- PostgreSQL 18 を必須とする場合は、VM か Kubernetes + CloudNativePG を選ぶ。
- DB アプライアンスを使う場合、2027年11月のコミュニティ EOL までに
  次のバージョンへ移行する計画を立てる。PostgreSQL は自動更新されないため、
  新規アプライアンスの作成とデータ移行が必要。
- エビデンス、バックアップ、ログはオブジェクトストレージに置く。
- IaC は Terraform で管理する。
- 監視は Prometheus と Grafana、ログは Loki を自前で構築する。

## ネットワーク

- VPC とサブネットを分け、アプリ、DB、運用で分離する。
- ファイアウォールで DB への接続元をアプリ層に限定する。
- ロードバランサーで TLS を終端し、HTTP/2 と WebSocket を確認する。
- 管理用の接続は VPN か踏み台経由にする。
- 閉域網が必要な場合は、専用回線や VPC 間接続を検討する。

## 運用

- DB の自動バックアップとポイントインタイムリカバリを設定する。
- オブジェクトストレージのバージョニングとライフサイクルを設定する。
- 監視は外形監視、リソース監視、ログ監視の 3 層にする。
- パッチ適用は四半期ごとに計画する。
- リージョン間のバックアップと復旧手順を決める。
- Runbook を Markdown で管理し、アラートからリンクする。

## QA に出す条件

- リージョンは東京、大阪、石狩のどれか。
- SLA とサポート体制はどうなっているか。
- さくらのクラウドの PostgreSQL のサポート期限はいつまでか。
- PostgreSQL 18 が必須か、15 で足りるか。
- PostgreSQL のパッチ適用は、さくらのクラウドのメンテナンスで行われるか。
- メジャーバージョンアップの手順とダウンタイムは。
- DB のバージョンアップ方針はどうなっているか。
- バックアップの頻度と保持期間はどうするか。
- ネットワークの閉域要件はあるか。
- 月額コストの上限はいくらか。
- 既存のオンプレ資産とどう接続するか。

## 参照

- [さくらのクラウド](https://cloud.sakura.ad.jp/)
- [データベースアプライアンス PostgreSQL](https://manual.sakura.ad.jp/cloud/appliance/database/postgresql.html)
- [データベースアプライアンス PostgreSQL 15 アップデート](https://cloud.sakura.ad.jp/news/2026/07/23/db-appliance-postgresql-15)
- [オブジェクトストレージ](https://cloud.sakura.ad.jp/products/object-storage)
- [オブジェクトストレージの API](https://manual.sakura.ad.jp/cloud/objectstorage/api.html)
- [AppRun](https://cloud.sakura.ad.jp/products/apprun/)
- [Terraform Provider for Sakura Cloud](https://registry.terraform.io/providers/sacloud/sakuracloud/latest)
- [CloudNativePG](https://cloudnative-pg.io/)
