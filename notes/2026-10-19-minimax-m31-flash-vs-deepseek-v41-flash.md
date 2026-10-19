---
type: Note
title: "MiniMax M3.1 Flash と DeepSeek V4.1 Flash の比較"
description: "OpenCode Console で提供される 2 つのフラッシュ系モデルの仕様・ベンチマーク・価格を比較し、用途別の使い分け基準を整理した検討メモ"
tags:
  - ai-models
  - model-selection
  - opencode
generated:
  by: opencode
  at: "2026-10-19T21:48:00+09:00"
status: "draft"
---

## 背景と目的

- 2026-10-19 に MiniMax M3.1 Flash が一般提供となり、OpenCode Console には
  無償版 `opencode/minimax-m3.1-flash-free` が追加された。
- DeepSeek V4.1 Flash は 2026-09-10 公開で、有償版
  `opencode/deepseek-v4.1-flash` と無償版 `opencode/deepseek-v4.1-flash-free` が
  利用できる。
- 日常のコーディングと調査作業を 2 モデルでどう分担するかを決めるため、
  一次情報（公式ドキュメント、モデルカード、価格表）に基づいて比較する。
- 二次情報で「SWE-bench 73.8%」「165 tps」などが広く引用されているが、
  MiniMax の一次情報まで追跡できないため、本稿の比較には採用しない。

## 比較対象の同定

| 観点 | MiniMax M3.1 Flash | DeepSeek V4.1 Flash |
| --- | --- | --- |
| OpenCode 無償 ID | `opencode/minimax-m3.1-flash-free` | `opencode/deepseek-v4.1-flash-free` |
| OpenCode 有償 ID | カタログに未掲載 | `opencode/deepseek-v4.1-flash` |
| 思考量の指定 | low / medium / high / xhigh / max | low / high / max |
| 公開日 | 2026-10-19 | 2026-09-10 |
| 公式 API 上の名称 | `MiniMax-M3.1-Flash` | `deepseek-flash` |

## 仕様比較

| 項目 | MiniMax M3.1 Flash | DeepSeek V4.1 Flash |
| --- | --- | --- |
| コンテキスト長 | 1,000,000 トークン | 1,000,000 トークン |
| 最大出力 | 512,000 トークン | 384,000 トークン |
| 入力モダリティ | テキスト・画像・動画 | テキスト・画像 |
| 思考の停止 | 不可（adaptive thinking 必須、`effort: none` は 400） | 可能（non-thinking / thinking の切替可） |
| 思考量の粒度 | 5 段階、無指定時は `max` | 1〜100 の連続値（OpenCode は 3 段階） |
| ツール利用 | 対応（interleaved thinking で各ツール呼び出しの間に推論） | 対応（公式は SFT→RL→OPD） |
| 対応プロトコル | Anthropic 互換 / OpenAI 互換 / Responses | OpenAI 互換 / Anthropic 互換 / Responses |
| ウェイト | 「オープンウェイト」と公告。Hugging Face には M3.1 リポジトリ未掲載 | MIT ライセンスで Hugging Face 公開済み |
| 主要構造 | M3 系 MSA 疎注意力。428B 総・約 23B アクティブ（M3.1 ガイドと M3 公式の記載） | 552B backbone + 196B Engram、prefill 8B / decode 16B アクティブ、40 層 CED |
| KV キャッシュ | 1M トークン対応（詳細は非公開） | CSA2 + FP4 KV で 890 bytes/token。V4-Flash 比 1/4、永続分は 1/8 |
| 学習データ | 未公表 | 45T トークンのマルチモーダルコーパス |

## ベンチマーク

### DeepSeek V4.1 Flash（公式モデルカード、推論 effort=100）

- GPQA Diamond 90.9、Codeforces 3471、MathArena Apex 65.6。
- Terminal-Bench 2.1 90.6、Terminal-Bench 3.0 30.0、Terminal-Bench 4.0 31.2。
- DeepSWE v1.1 74.2（mini-SWE ハーネス）。OpenCode ハーネスでは
  DeepSWE 65.5 / Terminal-Bench 2.1 85.0。
- Base モデルの MMLU-Pro 74.1、HumanEval 79.4。

### MiniMax M3.1 Flash

- 公式ドキュメントは「SWE、BrowseCamp、xBench で SOTA」と主張するが、指標値は
  未掲載。
- 参考値として M3（2026-06-01 公開）は SWE-Bench Pro 59.0%、
  Terminal-Bench 2.1 66.0%。M3.1 との直接比較にはならない。
- 第三次情報の「SWE-bench Verified 73.8%」「165 tps」「入力 $0.10」は一次情報から
  追跡できず、未検証として扱う。

### 実測の欠落

- 両者の TTFT / tokens per second を同一条件で比較した一次情報は未取得。
- 同一タスクでの成功率・修正率比較は未実施（本メモの最大の弱点）。

## 価格

| 観点 | MiniMax M3.1 Flash | DeepSeek V4.1 Flash |
| --- | --- | --- |
| 公式 API 入力 | $0.15 / 1M | $0.30 / 1M（ピーク）、$0.15（オフピーク） |
| 公式 API 出力 | $0.60 / 1M | $1.20 / 1M（ピーク）、$0.60（オフピーク） |
| 公式 API キャッシュ読 | $0.015 / 1M | $0.006 / 1M（ピーク）、$0.003（オフピーク） |
| OpenCode Console | 無料 | 無料 |
| OpenCode Console（有償） | M3.1 Flash は有償版がカタログに未掲載 | $0.30 / $1.20 / $0.006 |

- DeepSeek のピーク時間帯は UTC 01:00-04:00 と 06:00-10:00 の平日。
  オフピークはピークの半額で、キャッシュ読みの単価も下がる。
- OpenCode Console の無料版はいずれも期間限定で、プロバイダはゼロリテンション
  （学習利用なし）を明記している。
- 無料版の差（models.dev の OpenCode カタログ登録値）:
  `minimax-m3.1-flash-free` はコンテキスト 1,000,000 / 出力 512,000、
  `deepseek-v4.1-flash-free` はコンテキスト 200,000 / 出力 32,000。
  長文・長時間エージェント作業では、この制約が実効的な差になる。

## 見立て

- 品質根拠の透明性では DeepSeek が上。ベンチマークをハーネス込みで公開している。
- 無料版で扱えるコンテキストと出力上限では MiniMax が上。
- 公式 API の実額は MiniMax が一律 $0.15 / $0.60、DeepSeek はオフピークで
  半額になる。24 時間稼働するエージェントなら DeepSeek のオフピーク活用が有利。
- 入力モダリティでは動画を含む MiniMax のみ。
- 自行ホストとライセンスの明確さでは MIT の DeepSeek が上。
- M3.1 Flash は本日公開の直後で、ベンチマークとウェイトの公開が未完了。
  現時点で品質を断定してよい証拠が不足している。

## 適用指針

- 大規模コードベースの読解、長時間エージェントループ: 無償の
  `minimax-m3.1-flash-free`（1M コンテキスト）。
- 推論・数学・難問題の解答: `deepseek-v4.1-flash`（有償）か、素の
  `deepseek-flash` API を effort 100 で直接叩く。
- 予算ゼロの日常作業: どちらの無料版でも可。コンテキスト制約を確認して選ぶ。
- 画像・動画を含む入力: MiniMax M3.1 Flash。
- 自分の環境で動かす、ライセンスを自分で確認したい: DeepSeek V4.1 Flash（MIT）。

## フォローアップ

- [ ] 同一タスクで両モデルを実行し、TTFT / 出力速度 / 成功率 / 実コストを実測する。
- [ ] MiniMax M3.1 Flash のモデルカードとベンチマーク表の公開を確認する。
- [ ] Hugging Face に M3.1 のウェイトが掲載されたら自行ホスト条件を評価する。
- [ ] 無料版から有償版へ切り替える基準（月額利用量と回答品質）を決める。

## 参照

- [MiniMax モデル一覧（リリースノート）](https://platform.minimax.io/docs/release-notes/models)
- [MiniMax テキスト生成（モデル仕様・effort）](https://platform.minimax.io/docs/guides/text-generation)
- [MiniMax ツール利用と interleaved thinking](https://platform.minimax.io/docs/guides/text-m3-function-call)
- [MiniMax 価格](https://platform.minimax.io/docs/pricing/overview)
- [DeepSeek V4.1-Flash モデルカード](https://huggingface.co/deepseek-ai/DeepSeek-V4.1-Flash)
- [DeepSeek モデルと価格](https://api-docs.deepseek.com/quick_start/pricing)
- [DeepSeek V4.1-Flash リリース告知](https://www.deepseek.com/en/news/deepseek-v4-1-flash/)
- [OpenCode Console モデルと価格](https://opencode.ai/docs/zen/)
- [models.dev カタログ](https://models.dev/)
