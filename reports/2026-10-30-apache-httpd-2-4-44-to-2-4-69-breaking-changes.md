---
type: Report
title: "Apache HTTP Server 2.4.44 から 2.4.69 への破壊的変更調査報告"
description: "Apache HTTP Server 2.4.44 から 2.4.69 までの公式 CHANGES_2.4 を精査し、設定・プロトコル・ビルド要件の後方互換性を損なう変更を重要度別に分類して移行対応を整理した調査報告。"
tags:
  - apache-httpd
  - breaking-changes
  - upgrade
  - migration
generated:
  by: opencode
  at: "2026-10-30T13:01:40+09:00"
status: review
---

## 目的と範囲

Apache HTTP Server（httpd）2.4.44 から 2.4.69 までのリリース間で、既存の
設定・ビルド・運用に後方互換性を損なう変更（破壊的変更）を特定し、
アップグレード時の移行対応を整理する。

- 対象範囲: 2.4.44 より後の 2.4.45 〜 2.4.69 の全変更（2.4.69 は 2026-10-01
  リリースの現行 GA）。
- 破壊的変更の定義: 既存の設定ファイル・ビルド手順・公開動作が新しい
  バージョンで期待どおり機能しなくなる変更。セキュリティ修正に付随する
  意図的な動作変更を含む。脆弱性そのものの説明は範囲外。
- 除外: リリース日・モジュール内部実装のリファクタリングで外部動作が
  変わらないもの、mod_http2 / mod_md のバグ修正単体。

## 調査方法とデータソース

| データ | ソース | 種別 |
| --- | --- | --- |
| 2.4.44 〜 2.4.69 の全変更履歴 | [CHANGES_2.4](https://downloads.apache.org/httpd/CHANGES_2.4) | 一次情報 |
| 2.4.69 の依存要件 | [Announcement2.4](https://www.apache.org/dist/httpd/Announcement2.4.html) | 一次情報 |
| APR ハード要件（ビルド時検証） | `httpd-2.4.46` / `httpd-2.4.69` ソース tarball の `configure` | 一次情報 |
| 脆弱性と修正バージョンの対応 | [vulnerabilities_24](https://httpd.apache.org/security/vulnerabilities_24.html) | 一次情報 |

注記: `httpd-2.4.44` の tarball は archive.apache.org から削除済みのため、
直後の 2.4.46（セキュリティ修正のみでビルド要件の変更記載なし）で
2.4.44 当時の APR 要件を代理確認した。

## 要約: 重大度別変更一覧

### 高: 移行作業が必須な変更

| 変更 | 導入 | 影響と必要な対応 |
| --- | --- | --- |
| `AddType` によるハンドラー指定の無効化 | 2.4.60（61 で部分修正、62 で確定） | content-type 経由のハンドラー割当（例: `AddType application/x-httpd-php .php`）が無視されソース露出・誤配信になる。`AddHandler` へ書き換え必須（CVE-2024-38476 / 39884 / 40725） |
| mod_rewrite 置換の安全性強制 | 2.4.60 | 置換先頭セグメントがファイルシステムに触れる書き方、backref 内のエンコード済み `?` が拒否される。該当ルールは `UnsafePrefixStat` / `UnsafeAllow3F` フラグで明示的に opt-in するか書き直す（CVE-2024-38475 / 38474） |
| Windows での UNC アクセス制限 | 2.4.60 | リクエスト処理中の UNC パスアクセスが新ディレクティブ `UNCList` で許可されたホストのみに制限される。UNC を使う既存構成は `UNCList` 設定が必要（CVE-2024-38472） |
| CGI 類似スクリプトの TE/CL ヘッダー禁止 | 2.4.59 | CGI/CGI 類似のハンドラーが `Transfer-Encoding` と `Content-Length` を設定できなくなる。`Content-Length` が必要ならリクエスト単位で環境変数 `ap_trust_cgilike_cl` を設定（CVE-2024-24795） |
| TLS アップグレード (`SSLEngine optional`) の削除 | 2.4.64 | `SSLEngine optional` 構成が利用不可になる。該当構成は通常の `SSLEngine on` へ移行（CVE-2025-49812） |
| `LimitRequestBody` 既定値 1GB 化 | 2.4.54 | 既定が 0（無制限）から 1GB に変更。1GB 超のアップロードを扱うサイトは明示的な値の設定が必要 |
| RFC 2069 Digest 認証の削除 | 2.4.69 | mod_auth_digest が RFC 2069（`AuthDigestAlgorithm MD5` の旧モード）をサポートしない。RFC 2617 以降（`MD5-sess` 含む現行実装）のみ動作 |
| `mod_tls` の削除 | 2.4.63 | rustls ベースの実験的モジュールが本体から削除。必要ならスタンドアロン版を別途導入 |
| mod_proxy 未ロードでの `[P]` 利用の起動エラー化 | 2.4.63 | ディレクトリコンテキストの `RewriteRule ... [P]` で mod_proxy が未ロードだと設定エラーになる。mod_proxy のロードまたはルール修正が必要 |

### 中: 設定確認・受入試験を推奨する変更

| 変更 | 導入 | 影響 |
| --- | --- | --- |
| `MDServerStatus` 既定 off 化 | 2.4.69 | mod_md のステータス出力が既定で無効。監視で参照している場合は明示的に on を設定 |
| `H2SerializeHeaders` の no-op 化 | 2.4.55 | ディレクティブは残るが効果がなくなる（mod_http2 2.0.11 の再設計） |
| ACMEv1 のサポート削除 | 2.4.48 | mod_md が ACMEv1 CA を使用不可。ACMEv2 対応 CA のみ |
| tailscale 統合の削除 | 2.4.66 | mod_md の tailscale 証明書管理が削除（API 期限切れのため） |
| `.js` の MIME 型変更 | 2.4.58 | `application/javascript` から `text/javascript` へ変更。Content-Type で挙動分岐するバックエンド・キャッシュ・ブラウザ側検証が必要 |
| CGI 環境変数上書きの禁止 | 2.4.66 | Apache 設定由来の環境変数がサーバー計算の CGI 変数を上書きできなくなる（CVE-2025-65082）。意図的に上書きしていた構成は CGI 側での変数名変更が必要 |
| SSI `#exec cmd` へのクエリ文字列付加廃止 | 2.4.66 | mod_cgid 構成で `#exec cmd="..."` に shell-escape 済みクエリ文字列が渡らなくなる（CVE-2025-58098）。クエリを期待するスクリプトは引数渡しに変更 |
| フォワードプロキシの http(s) スキーム強制 | 2.4.52 | フォワードプロキシ対象の完全修飾 URI は http(s) スキーム・ホスト名が必須に。その他スキームは拒否される |
| `UNIQUE_ID` の base64url 化 | 2.4.47 | 変数値の形式が変更（`@` 等の除去）。`UNIQUE_ID` をキーにするログ解析・アプリは形式変更を吸収する必要 |
| 304 レスポンスのヘッダー削除方式変更 | 2.4.47 | 明示サブセット渡しから RFC 7234 準拠の一括削除へ。残存ヘッダー集合が変わり得る |
| `HeartbeatMaxServers` 既定値 10 化 | 2.4.54 | ドキュメント既定の 10 が実際の既定になる（従来 0）。ファイルベース監視を使う構成は 0 を明示 |
| クライアント開始再ネゴシエーションの拒否方式変更 | 2.4.60 | 接続切断から TLS alert 拒否へ変更。レガシークライアントの挙動が変わる |
| `SSLCACertificatePath` の CA DN 並び替え | 2.4.59 | CA 名リストが OpenSSL 標準関数で組み立てられ常にソートされる。DN 順序に依存する古いクライアントで選択される証明書が変わり得る |
| 中間レスポンス reason phrase の制御文字拒否 | 2.4.68 | ステータスコードの区切りは空白のみ受容。ヘッダー値に制御文字を含むバックエンドは 502 になり得る |
| `server-status` 出力の変更 | 2.4.58 | `BusyWorkers` / `IdleWorkers` の重複キーが削除され `GracefulWorkers` が追加。機械的パースを行う監視は修正が必要 |
| `mod_rewrite` の proxy 必須検査 | 2.4.63 | proxy を使う RewriteRule に mod_proxy が必須（高欄の起動エラー化と同根。コンテキストで挙動が異なる） |

### 低: 特定環境での周辺影響

| 変更 | 導入 | 影響 |
| --- | --- | --- |
| `Request-Range` ヘッダー対応の削除 | 2.4.63 | Navigator 2-3 / MSIE 3 由来のレガシーヘッダーを処理しない |
| HTTP/2 ヘッダー値の前後空白除去 | 2.4.55 | mod_http2 が HTTP/1.1 パーサーと同等に前後の space/tab を除去 |
| `mod_proxy_uwsgi` の `PATH_INFO` 正規化 | 2.4.52 | 先頭の重複スラッシュが削除される。uwsgi アプリ側の受入確認 |
| WebSocket の mod_proxy_http フォールバック | 2.4.48 | wstunnel が Upgrade 未対応時に mod_proxy_http へフォールバック（既定 on、`ProxyWebsocketFallbackToProxyHttp off` で opt-out） |
| ENGINE API 既定互換モード | 2.4.59 | OpenSSL 3 で `SSLCryptoDevice` ENGINE 系が `OPENSSL_API_COMPAT` 既定 1.1.1 互換で動作。ENGINE 非搭載ビルドでは `SSLCryptoDevice builtin` のみ |

## ビルド・依存要件の変化

| 項目 | 2.4.44 時点 | 2.4.69 時点 | 備考 |
| --- | --- | --- | --- |
| APR / APR-Util（ハード要件） | 1.4 以降 | 1.4 以降 | tarball `configure` の検証メッセージで両版とも「APR 1.4.0 or later」を確認。ハード要件は不変 |
| APR / APR-Util（推奨） | 1.4 台で全機能 | 1.5.x 最低、一部機能は 1.6.x | 2.4.69 の Announcement で明記。IPv6 リンクローカルの zone/scope 指定は APR 1.7.x が必要（2.4.60） |
| 正規表現ライブラリ | PCRE 8.x | PCRE2 10.x 対応（8.x は EOL） | 2.4.53 で pcre2 サポート追加。依存パッケージの更新が必要になり得る |
| OpenSSL | 1.0.2 / 1.1.x 系 | 3.x 系対応（2.4.52〜）、4.0 対応（2.4.68〜） | OpenSSL 3 では ENGINE API が非推奨。SSLFIPS は 3.0 対応（2.4.54）。OpenSSL 1.0.2 以前の組み合わせは 2.4.68/69 のビルド保証外 |
| Lua | 5.3 まで | 5.4 対応 | 2.4.47 で追加 |
| systemd 連携 | リンクされる構成あり | `mod_systemd` が DSO で `-lsystemd` 非リンク（2.4.44 で解消済み） | 2.4.44 時点で既に変更済みのため本調査の差分対象外 |

注記: TLS 1.0 / 1.1 の既定無効化は httpd の CHANGES に記載がなく、
OpenSSL 側・ディストリビューション側の既定変更に由来する。本レポートの
対象から除外する。

## アップグレードパス上のリグレッション

一括飛び越し（2.4.44 → 2.4.69）ではなく中間バージョンを経由する場合、
または段階検証する場合の注意点。

| 期間 | 問題 | 状態 |
| --- | --- | --- |
| 2.4.60 〜 2.4.62 | `AddType` ハンドラー無効化の修正が段階的（60 で導入、61 は部分修正、62 で完全） | 2.4.62 以降で確定。60/61 は使用しない |
| 2.4.64 のみ | 「`RewriteCond expr` が常に true と評価される」バグ（CVE-2025-54090）。セキュリティ制御の `RewriteCond expr` が全通過になる重大リグレッション | 2.4.65 で修正。2.4.64 は本番使用を回避する |
| 2.4.60 〜 2.4.68 | CGI 有効ディレクトリへの内部リダイレクト先が CGI として実行され得る（CVE-2026-42356） | 2.4.69 で修正 |
| 2.4.66 のみ | mod_http2 の early reset で double free・潜在的 RCE（CVE-2026-23918） | 2.4.67 で修正。2.4.66 は回避する |
| 2.4.65 のみ | 単一 CVE 修正のみのパッチリリース | 実質 2.4.64 の修正版 |

結論: 2.4.44 からの移行先として 2.4.69 を直接選ぶのが最も安全である。
経由する場合、2.4.60 / 2.4.61 / 2.4.64 / 2.4.66 の 4 版は避けて
2.4.63 → 2.4.65 → 2.4.67 → 2.4.69 の順が安全な経路となる。

## アップグレードチェックリスト

設定移行の観点で、2.4.44 から 2.4.69 へ上げる際の最小確認項目。

1. `AddType` でハンドラーを割り当てている箇所を `AddHandler` に置換する。
2. `RewriteRule` の置換先頭セグメント・backref の `?` 利用を棚卸しし、
   意図的に維持するものだけ `UnsafePrefixStat` / `UnsafeAllow3F` を付与する。
3. Windows で UNC パス参照を行う構成は `UNCList` を設定する。
4. 1GB を超えるリクエストボディを受けるサイトは `LimitRequestBody` を
   明示設定する。
5. CGI が `Content-Length` を返す必要がある場合は環境変数
   `ap_trust_cgilike_cl` を設定する。
6. `SSLEngine optional` を使っている構成は `SSLEngine on` に置換する。
7. Digest 認証を使う場合、クライアントが RFC 2069 専用でないことを確認
   する（大部分の現行クライアントは問題ない）。
8. `RewriteRule ... [P]` を使うなら mod_proxy をロードする。
9. `server-status` / `md-status` / `UNIQUE_ID` を機械利用している場合は
   形式変更（2.4.58 / 2.4.69 / 2.4.47）を吸収する。
10. `.js` の Content-Type 変化（2.4.58）に対し、キャッシュ・バックエンド・
    ブラウザ互換を確認する。
11. ビルド環境の PCRE2・OpenSSL 3（必要なら 4）・APR 1.5 以上への更新を
    確認する。

## 出典

- [CHANGES_2.4 (Apache HTTP Server 2.4 全変更履歴)](https://downloads.apache.org/httpd/CHANGES_2.4)
  （2.4.44 〜 2.4.69 の全エントリを精査）
- [Apache HTTP Server 2.4.69 Announcement](https://www.apache.org/dist/httpd/Announcement2.4.html)
  （APR 1.5.x 最低要件・リリース日 2026-10-01）
- [Apache HTTP Server Vulnerabilities (2.4)](https://httpd.apache.org/security/vulnerabilities_24.html)
- `httpd-2.4.46` / `httpd-2.4.69` ソース tarball の `configure`
  （APR 1.4 ハード要件の確認。archive.apache.org から取得）
- [httpd 2.4 ドキュメント](https://httpd.apache.org/docs/2.4/)
  （`AddHandler`、`LimitRequestBody`、`UNCList` 等の現行仕様参照先）
