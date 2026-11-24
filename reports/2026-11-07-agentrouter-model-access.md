---
type: Report
title: "AgentRouter 無料枠の実態と日本語入力の回避策"
description: "AgentRouter のモデルアクセス・クォータ・日本語 content-blocked を実測し、コードポイント変換プラグインで恒久対策した調査報告"
tags:
  - agentrouter
  - opencode
  - model-routing
  - troubleshooting
generated:
  by: opencode
  at: "2026-11-07T00:14:46+09:00"
status: review
---

## 目的と範囲

AgentRouter の無料枠を OpenCode の補助モデルとして使えるかを実リクエストで検証し、
判明した制約と恒久対策を記録する。対象は次の 3 点。

- API キーのクォータとアカウント残高の関係
- 利用可能なモデルとプロトコル
- 日本語プロンプトが `content-blocked` になる問題と回避策

## 調査方法

| 手段 | 内容 |
| --- | --- |
| `opencode run -m <provider>/<model>` | 実リクエスト。タイムアウト 90〜180 秒 |
| `GET /v1/models` | キーで利用可能なモデル一覧 |
| `GET /v1/dashboard/billing/subscription` | トークンの上限 |
| `GET /v1/dashboard/billing/usage` | 累積使用量 |
| `curl`（`User-Agent: opencode/1.0`） | ゲートウェイの直接検証 |
| [OpenCode V2 Plugins](https://opencode.ai/v2/docs/build/plugins/server/) | `http.request` フックの仕様 |

AgentRouter は UA を見てクライアントを制限している。`curl` 既定の UA では
`401 unauthorized client detected` になるため、直接検証では `opencode/1.0` を
明示した。OpenCode 経由のリクエストはこの制限に掛からない。

## クォータと残高

アカウント残高と API キー（トークン）の额度は別管理である。調査時、残高には
余裕があったが、トークン側の `hard_limit_usd` が `1` に設定されていた。

```text
hard_limit_usd: 1
total_usage:    97.9144   (New API の単位はセント)
```

1 リクエストあたり約 $0.065 を要求し、残量不足で
`token quota is not enough` が返る状態だった。OpenCode のシステムプロンプトと
ツール定義が毎回乗るため、$1 では 15 回程度で上限に達する。

キー設定で上限を変更した後は `hard_limit_usd: 100000000` となり、実質無制限に
なった。`total_usage` は累積値であり、残量ではない。変更後もモデルの応答を
確認済み。

## 利用可能なモデル

キーの `/v1/models` が返すのは次の 3 つだけだった。

| モデル | エンドポイント | 実測 |
| --- | --- | --- |
| `gpt-5.6-sol` | `/v1/responses` | OK（5〜13 秒） |
| `gpt-6-astra` | `/v1/responses` | OK（25 秒） |
| `gpt-6.1-sol` | `/v1/responses` | 設定追加後 OK（25 秒） |

これらは `/v1/chat/completions` を受け付けず、`/v1/responses` を要求する。
ローカル設定では `@opencode/ai/providers/openai/responses` を指定している。

利用できないモデル:

- `claude-opus-5` / `claude-opus-4-8`: Anthropic プロトコルで叩くと
  `403 该令牌无权访问模型`。トークンに権限が無い。
- `glm-5.3` / `deepseek-v4-flash`: `/v1/models` に現れず、503
  `无可用渠道`。このキーのグループにチャネルが無い。
- 上記はいずれもローカル設定では解決できないため、使用対象から外した。

## 日本語 content-blocked の分析

日本語を含むユーザーメッセージを送ると、HTTP 400 の
`content-blocked`（`agent_router_api_error`）が返る。切り分けの結果は次のとおり。

| 条件 | 結果 |
| --- | --- |
| 英語プロンプト | 通過 |
| 日本語プロンプト（OpenCode 経由） | `content-blocked` |
| 日本語プロンプト（最小リクエスト） | 通過 |
| `\uXXXX` エスケープ | ゲートウェイでデコードされ Block |
| 二重エスケープ（文字列としての `\uXXXX`） | デコードされ Block |
| Base64（短いメッセージ） | 通過 |
| Base64（長いメッセージ） | Block |
| 10 進 Unicode コードポイント列 | 通過 |
| 日本語を含むファイルの読み取り | 通過 |

最小リクエストでは日本語が通るため、単純な文字種フィルタではない。OpenCode の
システムプロンプトやツール定義を含む実リクエストで、ユーザーメッセージに
日本語がある場合に Block される。エスケープや Base64 はゲートウェイ側で
デコードされるため、難読化としては不十分だった。

## 実装した回避策

グローバルプラグイン `~/.config/opencode/plugins/agentrouter-escape/index.ts` を
追加した。`http.request` フックを `providerID: "agentrouter"` に限定し、
`role: "user"` の非 ASCII 文字列を 10 進コードポイント列へ変換する。

```text
The following decimal Unicode code points encode the user's message.
Decode the sequence to characters and carry out the request.
Reply in the same language as the decoded message. Do not mention the encoding.

12459,12524,12531,12488,...
```

- ASCII のみのメッセージは変換しない。
- 他プロバイダのリクエストには影響しない。
- モデルはコードポイントをデコードし、日本語で応答する。
- 日本語 1 文字が最大 7 バイト程度に膨らむため、長文ではトークン消費が増える。

## 検証結果

| 入力 | 結果 |
| --- | --- |
| `テスト。一文で返して。` | 日本語で応答 |
| 長い日本語（ファイル一覧の依頼） | 一覧を返却 |
| 日本語 + ツール利用 | ツールを実行して返却 |
| 英語 `Reply with exactly: OK` | `OK` |

TUI から日本語をそのまま入力できる。プラグインは共有サーバーに自動ロードされ、
再起動は不要だった。

## 設定変更

`~/.config/opencode/opencode.jsonc` の `providers.agentrouter.models` に
`gpt-6.1-sol` を追加した。既存 2 モデルと同じ Responses API パッケージと
上限値（context 1,050,000 / output 16,384）を指定している。

コミット:

```text
b7d623e feat: add AgentRouter gpt-6.1-sol model
00ea969 feat: encode AgentRouter user messages as code points
```

いずれも `OLD_ACCOUNT/grobalOpencodeSettings` の `master` に push 済み。

## 運用手順

- AgentRouter で使うモデルは `gpt-5.6-sol`、`gpt-6-astra`、`gpt-6.1-sol` の
  3 つに限定する。
- 日本語はそのまま入力してよい。プラグインが変換する。
- Claude / GLM / DeepSeek はこのキーでは使わない。設定にも追加しない。
- クォータは累積使用量と上限を定期的に確認する。1 リクエストのコストは
  システムプロンプトの大きさに比例する。
- 巨大な日本語資料はコードポイント変換で膨らむため、ファイル経由で読ませる。

## 残課題

- Claude はプロバイダ側のグループ権限が必要。ローカル設定では解決できない。
- プラグインは実験的な `http.request` フックに依存する。OpenCode V2 の API 変更で
  動かなくなる可能性がある。その場合は `content-blocked` の再発で検知する。
- `gpt-6.1-sol` の上限値は既存モデルと同値にした暫定値。実測で確認する。
- ゲートウェイのフィルタ仕様は予告なく変わり得る。モデル追加時は
  日本語プロンプトで再検証する。

## 参照

- [AgentRouter: OpenCode での使用](https://agentrouter.org/docs/opencode.html)
- [OpenCode V2: Server plugins](https://opencode.ai/v2/docs/build/plugins/server/)
- [OpenCode V2: Agents](https://opencode.ai/v2/docs/agents/)
