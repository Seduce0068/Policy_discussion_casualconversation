---
type: Note
title: "レガシー Java のテスト方式とエビデンス運用"
description: "Web アプリと Java GUI の両方を想定し、テスト仕様書、テストコード、エビデンス、フレームワーク選定を整理する。"
tags:
  - testing
  - java
  - spring-boot
  - playwright
  - evidence
generated:
  by: opencode
  at: "2026-10-27T20:05:39+09:00"
status: "draft"
---

## 前提

- 現行のインターフェースは、Web（SAStruts / JSP）と Java GUI（Swing / JavaFX）の
  両方を想定する。どちらが主かは棚卸しで確認する。
- 移行先は Spring Boot + Thymeleaf の Web アプリを基本とする。
- GUI が残る場合は、Web とは別のテスト方式を使う。
- テストは「現行の振る舞いを固定する」ことから始める。

## テストレベル

| レベル | 目的 | Web | Java GUI | 主なツール |
| --- | --- | --- | --- | --- |
| 単体 | ロジック、変換、計算 | 共通 | 共通 | JUnit 5、AssertJ、Mockito |
| 結合 | DB、外部 IF、トランザクション | 共通 | 共通 | Spring Boot Test、Testcontainers |
| 画面 | 画面単位の操作と表示 | Playwright | AssertJ Swing / TestFX | 各フレームワーク |
| E2E | 業務シナリオ | Playwright | Robot Framework など | 最小限 |
| 受け入れ | 業務担当の確認 | 手動 + 自動 | 手動 + 自動 | シナリオテスト |
| 性能 | 応答時間、スループット | Gatling / k6 | 画面応答の計測 | JMeter も可 |
| セキュリティ | 脆弱性 | OWASP ZAP | OS 権限、画面ロック | Web 中心 |

## インターフェース別の違い

| 観点 | Web アプリ | Java GUI |
| --- | --- | --- |
| 操作対象 | DOM、HTTP、Cookie、セッション | ウィンドウ、コンポーネント、イベント |
| 自動化 | Playwright、Selenium | AssertJ Swing、TestFX |
| CI | ヘッドレスで実行可能 | Xvfb / VNC が必要 |
| 不安定要因 | 非同期通信、タイミング、ネットワーク | 描画、DPI、OS、フォーカス、画面ロック |
| エビデンス | スクリーンショット、動画、trace | スクリーンショット、画面キャプチャ |
| 並列実行 | 比較的容易 | 難しい。直列実行が基本 |
| 認証 | フォーム、OIDC、セッション | OS ユーザー、アプリ内認証 |

## フレームワーク選定

| 用途 | 推奨 | 代替 | 備考 |
| --- | --- | --- | --- |
| Web E2E | Playwright | Selenium | 既存資産が Selenium なら段階移行 |
| Web コンポーネント | Playwright | Cypress | JS チームが主体なら Cypress |
| Swing GUI | AssertJ Swing | Robot Framework | コンポーネント名で特定できると安定 |
| JavaFX GUI | TestFX | Robot Framework | ノード ID を付与する |
| API | REST Assured | Spring MockMvc | 内部 API は MockMvc で軽く |
| DB | Testcontainers | DbUnit | 本番と同じ DB を使う。H2 は避ける |
| バッチ | Spring Batch Test | JobLauncherTestUtils | 入出力を比較 |
| 帳票 | PDFBox | 画像比較 | バイト比較はフォントで差分が出る |
| 性能 | Gatling | k6 / JMeter | シナリオをコードで管理 |
| セキュリティ | OWASP ZAP | Burp Suite | CI に組み込む |
| コントラクト | Pact | Spring Cloud Contract | 新旧サービス間 |
| アーキテクチャ | ArchUnit | — | 依存方向の強制 |

## テスト仕様書

- Markdown + OKF で管理する。Excel のテスト仕様書は Markdown 化の対象。
- front matter にテストケース ID、要件 ID、設計 ID を記録する。
- シナリオは Gherkin（Given / When / Then）で書く。
- 仕様書を正とし、テストコードを実装する。ただし、クリティカルなシナリオに限定する。
- テスト結果は CI から自動収集し、Markdown レポートを生成する。
- 仕様書とテストコードの紐付けは、テストケース ID で行う。
- Excel のテスト仕様書は Markdown に変換し、レビューは Markdown で行う。
- Excel は配布用のビューに限定し、正としない。

## テストコード

- JUnit 5、AssertJ、Mockito を基本とする。
- Spring Boot Test と Testcontainers で結合テストを書く。
- Web は Playwright、GUI は AssertJ Swing / TestFX で画面テストを書く。
- テストデータは Fixture か Builder で生成する。本番データを使わない。
- 命名は `should_..._when_...` のように、振る舞いを表す。
- flaky テストは隔離し、原因を直す。放置しない。

## エビデンス

- CI のアーティファクトに、JUnit XML、HTML レポート、スクリーンショット、
  動画、Playwright の trace を保存する。
- 保存先は GitHub Actions Artifacts、S3、共有フォルダのいずれかにする。
- 保存期間は監査要件に合わせる。
- 各ファイルに SHA-256 とタイムスタンプを記録する。
- 失敗時はスクリーンショットとログを自動収集する。
- テスト結果のサマリを Markdown で生成し、PR にリンクする。
- エビデンスの改ざん防止と、誰がいつ実行したかを残す。

### レビュー対象物のテキスト化

- レビューの主対象は、テスト仕様書、テストコード、結果サマリ、メタデータにする。
- スクリーンショット、PDF、動画は「証跡」として保存するが、レビューの主対象にしない。
- バイナリは Git に入れず、CI のアーティファクトか S3 に置く。
- バイナリには SHA-256 を付け、結果サマリから参照する。
- 画像比較、PDF のテキスト差分、ハッシュ、件数、終了コードは自動検証にする。
- 人間は、自動検証が失敗した場合だけバイナリを開く。
- AI レビューにはテキストだけを渡す。バイナリは渡さない。

| 対象物 | 形式 | レビュー方法 | 保存先 |
| --- | --- | --- | --- |
| テスト仕様書 | Markdown + Gherkin | PR の差分 | Git |
| テストコード | Java / TypeScript | PR の差分 | Git |
| 結果サマリ | Markdown | PR コメント | Git または CI |
| スクリーンショット | PNG / WebP | 失敗時のみ目視 | CI アーティファクト |
| PDF | PDF | テキスト抽出と差分 | CI アーティファクト |
| 動画 | MP4 | 失敗時のみ | CI アーティファクト |
| GUI キャプチャ | PNG + コンポーネントツリー | ツリーの差分 | CI アーティファクト |

## 特性テストと移行前後の比較

- Web は Playwright で現行画面を操作し、スクリーンショットと DOM を保存する。
- GUI は AssertJ Swing で現行画面を操作し、スクリーンショットを保存する。
- 帳票は PDF を比較する。フォントと埋め込みで差分が出るため、許容差を設定する。
- バッチは入力と出力を比較する。
- DB は移行前後のスナップショットを比較する。
- シャドーリードで、新旧に同じリクエストを流し、差分を確認する。
- ゴールデンマスターとして、入力と出力の組を大量に用意する。

## テスト環境

- Testcontainers で DB、MQ、外部 IF を再現する。
- HTTP の外部 IF は WireMock でモックする。
- 本番と同じ DB 製品を使う。H2 は使わない。
- テストデータはマスキング済みのものを使う。
- テスト間の分離と並列実行を設計する。

## 出口条件

- 単体テストと結合テストが CI で通る。
- 特性テストが現行の振る舞いを再現する。
- E2E が主要シナリオを覆う。
- エビデンスが保存され、監査要件を満たす。
- flaky テストが放置されていない。
- 業務担当の受け入れテストが通る。

## 参照

- [JUnit 5](https://junit.org/junit5/)
- [AssertJ](https://assertj.github.io/doc/)
- [Mockito](https://site.mockito.org/)
- [Testcontainers](https://testcontainers.com/)
- [Playwright](https://playwright.dev/)
- [AssertJ Swing](https://assertj.github.io/doc/#assertj-swing)
- [TestFX](https://github.com/TestFX/TestFX)
- [REST Assured](https://rest-assured.io/)
- [Pact](https://pact.io/)
- [OWASP ZAP](https://www.zaproxy.org/)
- [Gatling](https://gatling.io/)
- [Apache PDFBox](https://pdfbox.apache.org/)
- [ISO/IEC/IEEE 29119](https://www.iso.org/standard/81291.html)
