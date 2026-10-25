---
type: Report
title: "opencodego モデルルーティング調査報告: サブエージェント別割り当て"
description: "OpenCode Go 加入環境の全モデル候補をコスト・速度・精度・役割適正の4軸で評価し、サブエージェント5種とグローバルデフォルトのモデル割り当てを決定・適用した調査報告。"
tags:
  - opencode
  - model-routing
  - cost-optimization
  - subagent
generated:
  by: opencode
  at: "2026-10-25T23:17:11+09:00"
verified:
  by: opencode
  at: "2026-10-26T00:02:22+09:00"
status: review
---

## 目的と範囲

opencodego（OpenCode Go）加入により利用可能になった多数のモデルを、
サブエージェントの役割ごとに割り当て、開発効率・トークン消費・コストを
最適化する。本報告は選定の根拠、採用・不採用の判断、適用結果、残課題を
記録する。

## 調査方法とデータソース

| データ | ソース | 種別 |
| --- | --- | --- |
| 30 日間の実測使用量 | `opencode_console.get_usage`（リクエスト数・トークン・コスト） | 一次情報 |
| モデルカタログと単価 | `opencode models` / `tools.opencode.models`（全プロバイダ分） | 一次情報 |
| Go プラン料金・上限 | [OpenCode Go 公式ガイド](https://opencode.ai/v2/docs/console/go) | 一次情報 |
| サブエージェント定義 | `~/.config/opencode/agents/*.md` の役割・権限・出力形式 | 一次情報 |
| エージェント設定仕様 | [OpenCode V2 Agents ガイド](https://opencode.ai/v2/docs/agents) | 一次情報 |

## 実測ベースの現状

30 日間の実測は次のとおり。

| 指標 | 値 |
| --- | --- |
| 総リクエスト数 | 9,256 |
| 総コスト | $2.56 |
| キャッシュリード | 1.30B トークン（入力の約 25 倍） |
| `opencode-go/deepseek-v4.1-flash` | 1,133 req / $2.03（全コストの 79%） |
| `opencode-go/deepseek-v4-pro` | 57 req / $0.53（直近 24h に集中） |
| 無料モデル群（`*-free`） | 約 8,000 req / $0 |

直近 24 時間の観察:

- 24h のコストは $2.36 で、30 日累計の 92% を 1 日で消費した。つまり
  「30 日間の実測」は代表性が低く、月次コストの外挿（月 $2〜3 台）は
  根拠が薄い。今後のコスト予測には使わないこと
- `deepseek-v4-pro` の 57 req（$0.53）はほぼ全て直近 24h に発生しており、
  本セッション（親 = pro 手動選択）が主因
- `muse-spark-1.3-contributor` は 24h / 7d / 30d いずれも 0 req だったが、
  2026-10-25T23:45+09:00 の実リクエストで稼働確認済み（末尾の
  「実施状況」を参照）

導出される事実:

- コストの実勢は月 $2〜3 台（直近 24h の特異値を含む）であり、Go プラン
  （$10/月）のモデル別月間枠（例: `deepseek-v4.1-flash` は $80/月）に
  対して大きな余裕がある。**ただし 1 リクエスト当たりの単価差は極小で、
  コストは割り当ての拘束条件にならない**（後述の「評価軸ごとの根拠と
  強度」を参照）
- キャッシュリードがトークン量の支配因子であり、**キャッシュリード単価が
  モデル選定の最重要指標**
- `deepseek-v4.1-flash` は既に 30 日間の実運用で主力として機能しており、
  実証済みのワークホースである
- 親セッションを `deepseek-v4-pro` で長時間使うと、1 日で月間枠 $15 の
  約 3.5% を消費する。ただし月間枠 $15 は約 5,200 req に相当し、直近の
  高負荷日（57 req）を 30 日続けても約 1,700 req（枠の 33%）で収まる
  ため、レビュー系への pro 割当の余地は十分にある

## 評価軸ごとの根拠と強度

選定は 4 軸で行った。各軸の根拠の強度を明示する。外部レビュー
（2026-10-25、Claude Sonnet 5.5）の指摘を受け、速度・精度の強度を
「弱」へ訂正した。

### コスト（根拠: 強）

実測使用量と公式料金表の定量的照合に基づく。キャッシュリード単価
（$/1M）とモデル別月間枠を比較対象とした。ただし注意: Go は月額定額
$10 であり、実測月 $2〜3 ではどのモデルを選んでも枠に達しない。
公式の 1 リクエスト推定トークン数から計算すると、flash と
muse-spark の差は約 $0.0002/req（1,000 req で約 $0.2）にすぎず、
**コストは実質的に非拘束な制約である**。したがって最終的なモデル選択は
精度で決めるべきであり、コストは精度が拮抗した場合のタイブレークと
位置づける。

### 役割適正（根拠: 強）

各サブエージェントのシステムプロンプト・権限（読み取り専用か否か）・
トークン消費パターンを分析して割り当てた。探索・レビュー系は入力
（読み込み）支配、親エージェントは統合判断が主責務。

### 速度（根拠: 弱）

レイテンシの自前計測は未実施。根拠は (1) Go カタログ自体が
「OpenCode チームが coding agent 用途でテスト・ベンチマーク済みの
組み合わせ」という公式の選定表明、(2) flash 系モデルが軽量・低レイテンシ
設計であることの間接的根拠、(3) 実測 1,133 req の処理実績。いずれも
間接的であり、定量的裏付けはない。

### 精度（根拠: 弱）

自前ベンチマークは未実施。根拠は (1) Go 公式のベンチマーク選定表明、
(2) モデル命名の性能階層（flash < pro）、(3) variant による reasoning
effort 制御。精度の定量的検証は残課題として「残課題: 速度・精度の
定量的検証」節に記載する。

## 決定事項

### 割り当て表

| 対象 | モデル | 階層 |
| --- | --- | --- |
| グローバルデフォルト（`build` 親） | `opencode-go/deepseek-v4.1-flash` | flash |
| `explore` | `opencode/deepseek-v4.1-flash-free` | 無料 |
| `general` | `opencode-go/muse-spark-1.3-contributor` | contributor |
| `arch-reviewer` | `opencode-go/muse-spark-1.3-contributor` | contributor |
| `ops-reviewer` | `opencode-go/muse-spark-1.3-contributor` | contributor |
| `security-reviewer` | `opencode-go/deepseek-v4-pro#high` | pro |
| `test-author` | `opencode-go/deepseek-v4.1-flash` | flash |
| `test-reviewer` | `opencode-go/muse-spark-1.3-contributor` | contributor |

### Muse Spark Contributor 採用の判断

プロンプト・補完の学習利用は許容する方針（ユーザー判断）のため、
`muse-spark-1.3-contributor`（input $0.10 / output $0.20 / cache read
$0.002、月間枠 $60）を出力量の多い 3 役割に採用した。deepseek flash 比で
出力単価が 1/3 であり、レビューレポートや調査結果のような出力支配型
タスクで最大の削減効果がある。

採用にあたっての前提条件:

- **コンソールのプライバシー設定で有効化が必要だった**（2026-10-25T23:45
  に実リクエストで確認済み）。Go ガイドによると「paid endpoints that
  train on request data」はデフォルトでブロックされており、
  `Settings › Privacy` から「Allow paid endpoints that train on request
  data」を有効化する必要がある
- 地域制限は日本からは問題なし（Meta の Restricted Territories に
  日本は含まれず、モデル固有制限は Muse Image のみ）

なお外部レビュー（2026-10-25）の指摘どおり、muse-spark と deepseek
flash の 1 リクエスト当たりのコスト差は約 $0.0002 と小さく、採用判断の
実質的な根拠は「出力単価が低い」ことではなく「学習利用を許容できる
なら同等以下のコストで使える」ことにある。精度面の優劣は未検証であり、
A/B 比較の対象である（残課題を参照）。

### テスト系エージェントの設計（追加）

テストを「生成」「実行」「レビュー」に分解し、次の割り当てとした。

- **`test-author`（生成）** — `opencode-go/deepseek-v4.1-flash`。
  テスト仕様書・テストコード・テストデータの生成を担当。edit 許可・
  shell 禁止。コード生成の品質実績がある flash を割当
- **`test-reviewer`（レビュー）** — `opencode-go/muse-spark-1.3-contributor`。
  読み取り専用で既存 3 レビュアーと同一権限・同一モデル階層
- **テスト実行は親エージェントの責務** — `AGENTS.md` の
  「決定論的検証 (compile、lint、test) は親が実行する」方針に従い、
  実行系サブエージェントは作らない

### グローバルデフォルトを flash に変更した理由

当初案では無料モデル（`opencode/deepseek-v4.1-flash-free`）をデフォルトに
残す判断だったが、次の理由で Go 版 flash へ変更した。

- プライマリエージェントは統合判断を担い、無料モデルの rate limit や
  品質のばらつきがセッション全体のボトルネックになる
- `deepseek-v4.1-flash` は実測で主力実績があり、コスト増分は極小
  （30 日実測は直近 24h に偏っており、月次外挿の根拠は薄い点に注意）
- 月間枠 $80 に対し実測は 2.5% 使用であり、ピーク時間帯の 2 倍価格を
  考慮しても枯渇リスクは低い
- 設計・統合判断など高難度タスクでは、セッション単位で
  `opencode-go/deepseek-v4-pro` を手動選択する運用を併用する

## 候補モデルの比較と不採用理由

### opencode-go 内の不採用モデル

| モデル | 単価（$ in/out） | 不採用理由 |
| --- | --- | --- |
| `deepseek-v4-pro` | 0.66 / 1.98 | flash の 4.4 倍。月間枠 $15 と小さく、レビュー頻度が高い役割では枯渇リスク。`security-reviewer` のみ採用 |
| `glm-5.3-flash` | 0.15 / 0.50 | キャッシュリード $0.03 が deepseek flash の 10 倍。読み込み型タスクで不利 |
| `glm-5.3` | 1.40 / 4.40 | コスト高。月間枠 $15 |
| `qwen3.8-flash` | 0.15 / 0.47 | キャッシュリード $0.016（5 倍）。月間枠 $30 と小さい |
| `qwen3.8-max` | 2.00 / 6.00 | コスト高。月間枠 $15 |
| `qwen3.7-plus` | 0.40 / 1.60 | flash と pro の中間で、明確な適用役割がない |
| `minimax-m3.1-flash` | 0.15 / 0.60 | キャッシュリード $0.015。月間枠 $20 と小さい |
| `minimax-m3` | 0.30 / 1.20 | 中間帯。flash で足りる役割に割高 |
| `mimo-v2.6-flash` | 0.14 / 0.28 | キャッシュリード $0.0028（flash より安）、出力 $0.28（半額）、ピーク割増なし、月間枠 $60。コスト基準では deepseek flash より優位。保留理由は「自環境の実測実績ゼロ」のみであり、試用で解消できる。A/B 検証の最有力候補として残課題に記載 |
| `mimo-v2.5` / `mimo-v2.5-pro` | 0.14 / 0.28 ほか | 旧世代。V2.6 系が存在するため不採用 |
| `kimi-k3.1-pro` | 2.00 / 10.00 | コスト高。月間枠 $15 |
| `kimi-k2.7-code` | 0.95 / 4.00 | code 特化だがコスト中間。既存役割に必要な優位性なし |
| `claude-haiku-5-5` | 0.10 / 0.50 | 入力単価は最安級だが、月間枠 $15 と小さい |
| `gpt-6-luna` | 0.10 / 0.50 | 同上。実測も 7 req とほぼ未使用 |
| `gemini-4-argon` | 2.00 / 10.00 | コスト高（50% 割引期間でも $2/$10） |
| `grok-4.7` | 2.00 / 6.00 | コスト高。月間枠 $15 |
| `step-5-preview` | 1.00 / 2.70 | コスト高。preview 段階 |
| `hy4-preview` | 0.834 / 2.501 | コスト高め。preview 段階で実績なし |
| `hy3` | 0.14 / 0.58 | 入出力単価は安価な部類だが、キャッシュリード $0.035 が flash の 12 倍。読み込み型タスクで不利 |
| `longcat-2.0` | 0.30 / 1.20 | 中間帯。明確な適用役割がない |
| `muse-spark-1.3-contributor` | 0.10 / 0.20 | **採用**（general / arch / ops-reviewer）。学習利用は許容方針のため、出力単価最安級の本モデルを出力支配型 3 役割に割当 |
| `beam-free` | 無料 | 期間限定・地域制限あり。恒久運用の前提にできない |
| `iceman-contributor-free` | 無料 | 品質未検証。explore 用途の無料層は実績のある deepseek free で統一 |

### 無料モデル群（`opencode` プロバイダ）

`deepseek-v4.1-flash-free` を `explore` に採用。他（`glm-5.3-flash-free`、
`minimax-m3.1-flash-free`、`nemotron-3-ultra-free` など）は実測でも利用
実績があるが、無料層の役割を explore に集約するため、プライマリ・レビュー
系には不採用。rate limit と品質のばらつきが主因。

注意: Console 公式ガイドによると `deepseek-v4.1-flash-free` を含む無料
モデルは**全て期間限定**である。`beam-free` を期間限定を理由に退けたのと
同じ基準では、`explore` のモデルも恒久運用の前提にできない。無料提供が
終了した場合の `explore` のフォールバック先を定めること（「制約と
リスク」を参照）。

`minimax-m3.1-flash-free` と `iceman-contributor-free` は Console の
料金表一覧には掲載されていないが、Go カタログと実測使用量の両方に存在
する。カタログ側でのみ提供されるモデルである可能性が高い。

### 他プロバイダ（agentrouter / orcarouter / ollama / console-*）

| プロバイダ | 不採用理由 |
| --- | --- |
| `agentrouter` | 別ゲートウェイのサブスク（`AGENTIC_API_KEY`）。opencodego と二重管理になる。現状定義済みのため障害時のフォールバックとして温存 |
| `orcarouter` | 同上。`gpt-5.6-sol` のみで冗長 |
| `ollama`（`gemma4:4b` / `gemma4:26b`） | Termux 環境での性能・安定性が不確実。外部送信不可の要件があるタスク限定で検討 |
| `console-openai` など | 従量課金でコスト管理が Go の定額枠より複雑。Go で足りる役割には使わない |

## 制約とリスク

- **ピーク価格**: DeepSeek 系は平日 01:00–04:00 / 06:00–10:00 UTC
  （日本時間 10:00–13:00 / 15:00–19:00）が 2 倍。長時間タスクは時間帯を
  選ぶと実質コスト半減
- **モデル別月間枠**: 枠はモデルごとに独立。`security-reviewer` の
  `deepseek-v4-pro` は月間 $15（約 5,200 req 相当）。直近の高負荷日
  （57 req）を 30 日続けても枠の約 33% で、枯渇リスクは限定的
- **DeepSeek の ZDR 契約**: 月次更新（現契約は 2026-10-31 まで）。契約
  失効時のプライバシー影響を追跡する
- **上限到達時の挙動**: 上限到達後も無料モデルは引き続き利用できる
  （手動選択）。ただし Go のモデルに固定したエージェントは上限到達で
  ブロックされる。自動フォールバックはない。残高がある場合の Extra
  Usage（従量課金への切替）は未設定
- **無料モデルの期間限定**: `explore` が依存する
  `deepseek-v4.1-flash-free` を含む無料モデルは全て期間限定。終了時の
  フォールバック先を `opencode-go/muse-spark-1.3-contributor`（または
  `mimo-v2.6-flash` の A/B 結果次第でそちら）に定める

## 残課題: 速度・精度の定量的検証

現時点の選定は速度・精度について間接的根拠に依存する。外部レビューの
指摘どおり、コスト差は極小（約 $0.0002/req）で拘束条件にならないため、
**残る選定基準は精度のみ**である。次の方法で定量化する。

1. **同一レビュータスクの A/B 比較** — 同一 diff を `muse-spark-1.3-
   contributor` と `deepseek-v4.1-flash`（必要なら `deepseek-v4-pro`）で
   レビューさせ、検出数・誤検知数・所要時間を計測する。特に
   `arch-reviewer` は判断の質が価値の中心であり、最も安いモデルを当てる
   役割ではないため、結果次第で pro への昇格を検討する
2. **`test-author` での mimo 試用** — `mimo-v2.6-flash` はコスト基準で
   flash より優位で、出力がテスト実行で検証されるため試用リスクが低い。
   `test-author` を mimo に切り替えて 1 週間試し、検出数・所要時間・
   失敗率を比較する
3. **レイテンシ計測** — `opencode run --model <m> ...` で同一プロンプトを
   各モデルに投げ、TTFT・総時間を比較する

## 適用済み変更

| ファイル | 変更 |
| --- | --- |
| `opencode.jsonc`（グローバル） | デフォルトモデルを `opencode-go/deepseek-v4.1-flash` に変更。`explore` / `general` のモデルを追加 |
| `agents/arch-reviewer.md` | `model: opencode-go/muse-spark-1.3-contributor` |
| `agents/ops-reviewer.md` | 同上 |
| `agents/security-reviewer.md` | `model: opencode-go/deepseek-v4-pro#high` |
| `agents/test-author.md`（追加） | `model: opencode-go/deepseek-v4.1-flash` |
| `agents/test-reviewer.md`（追加） | `model: opencode-go/muse-spark-1.3-contributor` |

コミット: `1018016`（割り当て）、`a58b501`（グローバルデフォルト変更）、
`eb725f9`（muse-spark contributor への切替）、`1a42644`（テスト系
エージェント追加）。

`#variant` 構文は公式ドキュメントで確認済み。Models ガイドに
`opencode run --model openai/gpt-6.1-sol#high`、Agents ガイドに
`model: anthropic/claude-sonnet-5-5#high` の例があり、
`deepseek-v4-pro` の variant 一覧（`high` / `max`）もカタログで
確認している。

## 実施状況: コンソールのプライバシー設定（確認済み）

`muse-spark-1.3-contributor` を有効にするには、ユーザーによるコンソール
操作が 1 件必要だった。

1. [OpenCode Console](https://opencode.ai/console) にログイン
2. `Settings › Privacy` を開く
3. 「Allow paid endpoints that train on request data」を有効化

2026-10-25T23:45+09:00 に実リクエストで検証し、**設定有効化済みであること
を確認した**。結果は次のとおり。

- `opencode run --model opencode-go/muse-spark-1.3-contributor` で
  正常応答（ブロックなし）
- 24h 実測に 1 req / $0.0022 として記録され、単価も理論値どおり
- 以降、muse-spark 割り当てのサブエージェントはそのまま機能する

## 参照

- [OpenCode Go ガイド](https://opencode.ai/v2/docs/console/go)
- [OpenCode V2 Agents](https://opencode.ai/v2/docs/agents)
- [OpenCode V2 Models](https://opencode.ai/v2/docs/models)
- [OpenCode V2 Config](https://opencode.ai/v2/docs/config)
