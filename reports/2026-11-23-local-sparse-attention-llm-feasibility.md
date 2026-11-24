---
type: Report
title: "ローカル LLM におけるスパースアテンション活用の実現可能性評価"
description: "GTX 1660 SUPER 6GB / Turing sm_75 環境で、スパース・線形アテンション系モデルの速度、エージェント適性、コミュニティ派生のリスクを実測と一次情報で評価した。"
tags:
  - llm
  - sparse-attention
  - local-inference
  - agent
  - feasibility
generated:
  by: "DeepSeek Harness"
  at: "2026-11-23T17:38:45+09:00"
status: draft
---

## 結論

**条件付きで作れる。** 厳密な top-k 型スパースアテンションは本環境の 6GB VRAM では
動かない。ただしアテンションを線形化した Qwen3.5-4B を llama.cpp で動かせば、
実機で 63〜65 tok/s の生成速度が出た。

- エージェント目的の推奨: llama.cpp + `Qwen3.5-4B-Q4_K_M`。工具呼び出し、
  JSON スキーマ準拠、tool response 後の追従を実機で確認済み。
- 速度最優先: `Jackrong 2B Claude 蒸留`。実測 125 tok/s / 2.85 GB。
  ただし tool response 後の出力が冗長。
- 速度・省メモリのバランス: `gemma-4-E2B-it-Q4_K_M`。実測 95.8 tok/s / 2.83 GB。
- STEM 特化: `Jackrong DeepSeek-V4-Pro-Qwen3.5-4B`。実測 65 tok/s / 4.89 GB。
  ツールは動作するが JSON 内容は Qwen3.5-4B より雑。
- 即時利用: 既存 Ollama の `qwen2.5:3b`。実測 87.9 tok/s。
- 真のスパースアテンション（DSA / NSA / QSA）: 巨大モデル限定で本環境では不可。

重要な訂正が 2 点ある。第一に、Ollama 0.21.0 は `qwen3.5:4b` と `gemma4:e2b` を
HTTP 412（要新版）で拒否した。Ollama 経由ではなく llama.cpp の直接利用が必要である。
第二に、Qwen3.5 の Gated DeltaNet は線形アテンションであり、厳密な意味での
スパースアテンションではない。用語の整理は後述する。

## 調査範囲と方法

swarm-research を完全無制限プロファイルで実行した。R1 で論点分解を行い、
R2 で 6 観点、R3 で 2 観点、R4 で 2 観点の敵対検証を並列実施し、
R6 で親が重複・矛盾を吸収した。使用モデルは既定の `hy4-flash-free`、
同時起動数は最大 8、深度は 1 である。

情報源は GitHub の一次情報（コード・issue・PR・release）、
Hugging Face のモデルカード、Ollama と NVIDIA の公式ドキュメント、
および親による実機計測である。ハーネス指示 §16 に従い、
`web_search` と `rustxng` MCP は使用していない。

## 実環境と実測（Runtime Proof）

### 環境

| 項目 | 値 |
| --- | --- |
| OS | Ubuntu 26.04.1 LTS |
| CPU | Ryzen 5 3600（6C/12T, AVX2） |
| メモリ | 62 GiB RAM + 63 GiB swap |
| GPU | GTX 1660 SUPER, sm_75 (Turing), 6144 MiB |
| ドライバ | 595.99.02（CUDA 13.2 対応） |
| Ollama | 0.21.0。モデルは `/usr/share/ollama/.ollama/models` |
| 既存モデル | `qwen2.5:7b`（4.7 GB）、`qwen2.5:3b`（1.9 GB） |

### 生成速度の実測

| 構成 | 配置 | decode | prompt eval |
| --- | --- | --- | --- |
| `qwen2.5:3b` Q4（Ollama） | 100% GPU, 2.4 GB | 87.9 tok/s | 265〜290 tok/s |
| `qwen2.5:7b` Q4（Ollama） | 22% CPU / 78% GPU, 5.4 GB | 13.9 tok/s | 135 tok/s |
| 7b + FA/q8 KV（別ポート実験） | 22% CPU / 78% GPU, 5.3 GB | 16.8 tok/s | 未計測 |
| `Qwen3.5-4B-Q4_K_M`（llama.cpp） | 全 99 層 GPU, 2.6 GB ファイル | 63.7〜64.9 tok/s | 82〜121 tok/s |
| `gemma-4-E2B-it-Q4_K_M`（llama.cpp） | 全 99 層 GPU, 3.1 GB ファイル | 95.8 tok/s | 259〜270 tok/s |
| `gemma-4-E2B-it-UD-japanese-imatrix` Q4_K_M（llama.cpp） | 全 99 層 GPU, 3.1 GB ファイル | 97.0〜99.6 tok/s | 264〜277 tok/s |
| `Jackrong 2B Claude 蒸留` Q4_K_M（llama.cpp） | 全 99 層 GPU, 1.27 GB ファイル | 119.3〜126.0 tok/s | 129〜266 tok/s |
| `Jackrong DeepSeek-V4-Pro-Qwen3.5-4B` Q4_K_M（llama.cpp） | 全 99 層 GPU, 2.78 GB ファイル | 63.5〜65.5 tok/s | 69〜115 tok/s |
| `Qwen3.5-4B` を `-ngl 20` に変更 | 20 層 GPU, 3.4 GB 使用 | 9.2〜18.9 tok/s | 38〜39 tok/s |

### unsloth 量子化レベルの比較

同一モデル（Qwen3.5-4B）の unsloth GGUF を 4 種類実測した。

| 量子化 | ファイルサイズ | decode | VRAM 使用 | 工具呼び出し |
| --- | --- | --- | --- | --- |
| IQ4_XS | 2.48 GB | 58.6〜61.5 tok/s | 5.17 GB | 正常 |
| **Q4_K_M** | 2.74 GB | **63.7〜64.9 tok/s** | 5.30 GB | 正常 |
| UD-Q4_K_XL | 2.91 GB | 58.5〜60.2 tok/s | 5.25 GB | 正常 |
| Q5_K_M | 3.14 GB | 53.2〜55.1 tok/s | 5.57 GB | 正常 |

**推奨は Q4_K_M**。最速で、VRAM も 6GB に収まる。
ファイルを小さくしても（IQ4_XS）VRAM はほぼ減らず、速度だけが落ちた。
計算バッファと KV が VRAM の大半を占めるためである。
Q5_K_M は品質上是利だが、生成速度が約 17% 低下した。
Q6_K 以上は Q5_K_M の 5.57 GB を超えるため 6GB では実用的ではない。
Q5_K_M の JSON スキーマ出力は `max_tokens=1200` で切れたが、
Q4_K_M も `max_tokens=400` で切れており、思考トークン量に依存する。

llama.cpp の起動条件は `-ngl 99 -c 16384 -ctk q8_0 -ctv q8_0` である。
バイナリは [llama.cpp b12359 の CUDA 12.8 ビルド](https://github.com/ggml-org/llama.cpp/releases/tag/b12359)
を一時ディレクトリに配置して使用した。初回プロセスでは PTX JIT により
prompt eval が 0.24〜0.30 tok/s に低下したが、CUDA キャッシュ温存後に
プロセスを再起動すると 82〜121 tok/s に回復した。
Turing 向け SASS は同梱されないため、この初回ペナルティは残存する。

## 用語の整理：3 つの「スパース」は別物

ユーザーが求めた「スパースアテンション」は、少なくとも 3 種類の機構と混同されやすい。

### スパースアテンション（top-k 選択型）

indexer がトークンまたはブロックを選択し、KV の読み出しを間引く方式である。
DeepSeek の DSA は `index_n_heads=64` / `index_topk=2048` の lightning indexer を使い、
選択位置以外に `-inf` を足す。NSA、MoBA、QSA もこの分類に入る。
公開モデルは 685B クラスか、実験的な 2B に限られる。

### 線形アテンション / recurrent

Gated DeltaNet（GDN）と Kimi Delta Attention（KDA）は、全トークンを
固定サイズの状態に圧縮する線形・再帰型である。top-k 選択は行わない。
[GDN の原論文](https://arxiv.org/abs/2412.06464)は Linear Transformers 系として
自らを定義しており、[FLA の README](https://github.com/fla-org/flash-linear-attention)
も linear と sparse を独立カテゴリとして併記する。

### MoE のスパース性

FFN の専門家を選択する機構で、注意計算とは無関係である。
Qwen3-30B-A3B は総 30.5B / 活性 3.3B、128 専門家中 8 個を選択する。

Qwen3.5-4B の [モデルカード](https://huggingface.co/Qwen/Qwen3.5-4B)は
`8 × (3 × (Gated DeltaNet → FFN) → 1 × (Gated Attention → FFN))` を明記し、
`config.json` の `layer_types` も `linear_attention` と `full_attention` を
3:1 で並べる。4B は dense で MoE を持たない。したがって本レポートの
本命構成は「スパースアテンション」ではなく「線形アテンションの活用」である。

## 技術動向と本環境での制約

### 真のスパースアテンションは GPU 側で弾かれる

[DeepSeek-V3.2-Exp](https://github.com/deepseek-ai/DeepSeek-V3.2-Exp) の DSA は
DeepGEMM と FlashMLA に依存する。FlashMLA は 2026-09-30 リリースで
Hopper サポートも削除し、SM100/SM103（Blackwell）専用となった。
[vLLM の issue](https://github.com/vllm-project/vllm/issues/66960)では
A100（SM80）でも DSA モデルが起動できないと報告され、
SGLang は [RFC](https://github.com/sgl-project/sglang/issues/6006)で
sm75 サポートを v0.5 で廃止する方針を示した。

llama.cpp は [deepseek41.cpp](https://github.com/ggml-org/llama.cpp/blob/master/src/models/deepseek41.cpp)
に DSA 経路（`LLAMA_DSV41_SPARSE` / `indexer_top_k`）を実装済みだが、
対象は DeepSeek-V4.1-Flash（552B backbone）や Kimi-K3.1-Pro（総 2.8T）である。
6GB VRAM では論外である。

### 小型の top-k スパースは「存在するが本番向けではない」

2B の QSA 実験 GGUF（`LLAMA_QSA_TOP_K=2048`）が公開されているが、
自前の llama.cpp パッチと Vulkan/ROCm を要し、10GiB カードでの報告である。
作者自身が not a production model と明言しており、
本環境（6GB / sm_75）での実用性は未確認である。

### CPU-GPU ヘテロの代替も本環境では厳しい

KTransformers は [kt-kernel](https://github.com/kvcache-ai/ktransformers/blob/main/kt-kernel/README.md)
で compute capability 8.0+ を要求し、Turing 7.5 は明示的に非対応である。
AVX2 チュートリアルも GPU 24GB+ を要件とする。
Ryzen 5 3600 の AVX2 は満たすが、6GB VRAM と sm_75 がボトルネックになる。

### FlashAttention 系も Turing では限定的

[FlashAttention-2](https://github.com/Dao-AILab/flash-attention) の公式 CUDA ビルドは
Ampere/Ada/Hopper 限定で、Turing は別リポジトリの機能制限版になる。
FlashAttention-3/4 は Hopper/Blackwell 専用である。
Ollama の Flash Attention は sm_75 で有効化できるが、7B の実測では
16.8 tok/s と改善は限定的だった。KV cache の q8_0 量子化は
トークンあたり 112 MiB に抑えられたものの、CPU オフロード比率は変わらなかった。

## VRAM とメインメモリの併用

仕組み上は可能だが、本環境（6GB VRAM / 62GB RAM）では実用的ではない。

llama.cpp ではスパース用の KV と indexer キャッシュも通常の KV と同じ
オフロード経路を使い、`-ngl` と `-kvo` が適用される。
[issue #26549](https://github.com/ggml-org/llama.cpp/issues/26549) では
DSA モデルを `--n-cpu-moe 35` で CPU に逃がし、32GB VRAM + 96GB DDR4 で
7.2〜7.4 tok/s を出した例がある。しかし次の障壁がある。

- スパース化は KV の読み出し行数を間引くだけで、確保サイズは `n_ctx` 由来のまま。
  むしろ indexer 用 KV が追加される（[llama-kv-cache-dsa.cpp](https://github.com/ggml-org/llama.cpp/blob/master/src/llama-kv-cache-dsa.cpp)）。
- SGLang の HiSparse は DSA の KV を CPU pinned memory に置く公式機能だが、
  ホストメモリ目安は約 1TB で、PD 分散の decode インスタンス専用である。
- DSA カーネル自体が Hopper / Blackwell 向けで、Turing sm_75 では動かない。
- 本環境での実測では `-ngl 20` にすると VRAM は 3.4GB に減ったが、
  生成速度は 9.2〜18.9 tok/s に低下した。

## エージェント目的の品質比較

llama.cpp の OpenAI 互換 API を使い、4 モデルに同一のタスクを投げた。
工具呼び出し、JSON スキーマ準拠、tool response 後の最終回答のすべてで
4 モデルとも正常に動作した。生成速度では Jackrong 2B が最も速く、
ベンチマークとコンテキスト長では Qwen3.5-4B が有利である。

| 指標 | Qwen3.5-4B | Gemma 4 E2B | Jackrong 2B Claude 蒸留 | Jackrong DeepSeek-V4-Pro 4B |
| --- | --- | --- | --- | --- |
| 工具呼び出し（実機） | 正常 | 正常 | 正常 | 正常 |
| JSON スキーマ準拠（実機） | 正常 | 正常 | 正常 | 正常 |
| tool response 後の追従（実機） | 正常 | 正常 | 正常 | 正常 |
| MMLU-Pro（自己報告） | 79.1 | 60.0 | 未掲載 | STEM サブセットのみ |
| GPQA Diamond（自己報告） | 76.2 | 43.4 | 未掲載 | 未掲載 |
| LiveCodeBench v6（自己報告） | 55.8 | 44.0 | 未掲載 | 未掲載（コード学習なし） |
| IFEval（自己報告） | 89.8 | 未掲載 | 未掲載 | 未掲載 |
| BFCL-V4（自己報告） | 50.3 | 未掲載 | 未掲載 | 未掲載 |
| TAU2-Bench（自己報告） | 79.9 | 24.5 | 未掲載 | 未掲載 |
| コンテキスト | 262,144 | 128,000 | 262,144 | 262,144 |
| decode（実機） | 64 tok/s | 96 tok/s | 125 tok/s | 65 tok/s |
| VRAM（実機） | 5.3 GB | 2.83 GB | 2.85 GB | 4.89 GB |

4 モデルとも工具呼び出し・JSON・tool response 後の追従は実機で正常だった。
Jackrong 4B のモデルカードは `coding and tool-use transfer have not yet been
established for the 4B release` と明記するが、今回の単純なツール定義では
正常に動作した。ただし JSON の内容は Qwen3.5-4B より雑で、
季節の説明に重複や不自然な項目（`お中元` が夏と秋の両方に出現）が見られた。
Jackrong 2B は tool response 後に見出しと「ツールによる取得」節を
自主的に追加するなど、出力が冗長である。

TAU2-Bench は Qwen が airline ドメインに独自修正を適用しており、
Gemma の「average over 3」と条件が揃っていない。差の方向性は見えるが、
数値の差自体は自前再現がない限り断定しない。独立ベンチは両モデルとも未掲載である。

実装上の注意点は次のとおりである。

- Qwen3.5 は既定で thinking モード。`max_tokens=400` では JSON 出力が
  トークン上限で切れたため、実測では 1200 を指定した。マルチターンでは
  履歴に思考内容を含めないこと。
- Gemma 4 は `tool_calls[].function.arguments` を JSON 文字列ではなく辞書で
  渡す必要がある。llama.cpp の tool call grammar には未修正バグがあり、
  再帰的 JSON 引数で無限生成のリスクがある
  （[issue #32561](https://github.com/ggml-org/llama.cpp/issues/32561)）。

## Qwen 系譜と小型版の有無

Qwen は 3.5 以降もバージョンアップしているが、**3B〜9B の小型版は 3.5 が最後**である。

| 世代 | 公開 | モデル |
| --- | --- | --- |
| Qwen3.5 | 2026-02〜03 | 397B-A17B / 122B-A10B / 35B-A3B / 27B / 9B / 4B / 2B / 0.8B |
| Qwen3.6 | 2026-04 | 27B / 35B-A3B |
| Qwen3.8 | 2026-08 | 2.4T-A95B / 27B / Flash-Next（125B） |

出典は [QwenLM/Qwen3.8 の README](https://github.com/QwenLM/Qwen3.8/blob/main/README.md) の
News セクションである。Qwen3.8-Flash-Next は Qwen4 のプレビューで
Gated DeltaNet + QSA（Qwen Sparse Attention）に進化しているが、
125B 総パラメータ（6B 活性）+ 51B n-gram embedding + 4B MTP で本環境では論外である。
ライセンスは 3.5 全サイズと 3.6/3.8-27B が Apache-2.0、
Flash-Next は `qwen-community-1.0` で条項確認が必要である。

## コミュニティ派生モデル

公式以外にも多数の派生がある。6GB 環境では次の 5 類に分けられる。

| 分類 | 代表例 | 6GB 適合 | エージェント用途 |
| --- | --- | --- | --- |
| 量子化のみ | unsloth / ggml-org / bartowski / Google 公式 QAT | ○ | 推奨 |
| 蒸留（Heretic なし） | Jackrong DeepSeek-V4-Pro-Qwen3.5-4B、Jackrong 2B Claude 蒸留 | ○ | 検証済み（下記） |
| 検閲解除 | HauhauCS Uncensored、xtools-at DeepSeek-V4-Pro-Qwen3.5-4B-UNCENSORED-heretic、Heretic / abliterated | ○ | 除外 |
| スパース実験 | tvall43 2B QSA、zephel01 9B QSA | × | 除外 |
| Draft モデル | nvidia Qwen3.5-4B-Qwen3.8-27B-Draft | × | 対象外 |

Jackrong の蒸留版と Heretic 版は別物である。
[Jackrong の DeepSeek-V4-Pro-Qwen3.5-4B](https://huggingface.co/Jackrong/DeepSeek-V4-Pro-Qwen3.5-4B)
は DeepSeek-V4-Pro の応答約 25 万件で蒸留したもので、Heretic は適用されていない。
これに [Heretic v2.0.0 を適用した xtools-at 版](https://huggingface.co/xtools-at/DeepSeek-V4-Pro-Qwen3.5-4B-UNCENSORED-heretic)
は拒否率が 5/100（ベースは 99/100）、KL 分散は 0.0378 である。
したがって除外すべきは Heretic 版であり、蒸留版そのものではない。

[Jackrong 2B Claude 蒸留](https://huggingface.co/Jackrong/Qwen3.5-2B-Claude-4.6-Opus-Reasoning-Distilled-GGUF)
のモデルカードには Heretic や検閲解除の記述はなく、ライセンスは Apache-2.0 である。
固有の安全性リスクは確認できなかった。両モデルとも実機検証を実施し、
速度・工具呼び出し・JSON 遵守・tool response 追従のすべてで正常に動作した
（詳細は「エージェント目的の品質比較」を参照）。
Claude の出力を教師データにしたことのライセンス上の扱いは出典がなく未確認である。

量子化のみの派生では unsloth の Qwen3.5-4B Q4_K_M（2.74GB）が
最も実績がある。ggml-org 版は 2026-11-12 公開で 3.03GB だが、
モデルカードが自動変換のまま TODO である。

[nvidia の Draft モデル](https://huggingface.co/nvidia/Qwen3.5-4B-Qwen3.8-27B-Draft)
は **Qwen3.8-27B の外部ドラフト**であり、4B 本体を速くするモデルではない。
6GB 環境では対象外で、ライセンスも NVIDIA Open Model Agreement である。

スパース実験系は本環境では使えない。
[2B QSA](https://huggingface.co/tvall43/Qwen3.5-2B-heretic-QSA-experiments) は
自前の llama.cpp パッチと Vulkan/ROCm が必要で、作者自身が
`it is not a production model` と明言している。
K=2048 で PPL が +20.2%、短上下文の NIAH は 1/5 に崩壊した。
9B QSA は 128K コンテキストに 11GB 超の GPU を要し、
stock llama.cpp は indexer テンソルを読み込めない。
確認した範囲では、**6GB / sm_75 で本物の top-k スパースを実用レベルで
動かせるコミュニティ版は存在しない**。

日本語特化版として
[dahara1/gemma-4-E2B-it-UD-japanese-imatrix](https://huggingface.co/dahara1/gemma-4-E2B-it-UD-japanese-imatrix)
も試した。日本語キャリブレーションと Unsloth Dynamic Quantization 2.0 を
使った Q4_K_M（3.11GB）で、実測は 97.0〜99.6 tok/s / VRAM 3.01GB だった。
公式 E2B の 95.8 tok/s / 2.83GB とほぼ同等で、明確な優位性は確認できなかった。

### リスクと軽減策

エージェント用途では次のリスクが敵対検証で確認された。

- **安全性**: Heretic は既定で拒否抑圧を行い、HauhauCS 版は
  `0/465 refusals` を謳う。拒否が消えてもツール実行能力は残るため、
  モデル側の安全に依存せずツール層で拒否・承認を実装する。
- **ライセンス**: Apache-2.0 派生でも改変通知と LICENSE 同梱が必須。
  HF の license タグは自己申告で、base_model チェーンと本文を照合する。
- **サプライチェーン**: GGUF には署名・完全性検証の規定がなく、
  chat_template がモデル内に埋め込まれる。細工された GGUF で
  llama.cpp が `GGML_ASSERT` で abort する例もある
  （[issue #32821](https://github.com/ggml-org/llama.cpp/issues/32821)）。
- **信頼性**: MTP/draft 混在でコードの重複・欠落、
  低ビット量子化で NaN の `GGGG` 出力、speculative decoding で
  greedy 出力が変わる例が報告されている。

軽減策は、公式 GGUF のみを採用して sha256 を固定すること、
`uncensored` / `abliterated` / `heretic` 系をレジストリから除外すること、
エージェントはネットワーク無効のコンテナ・読み取り専用 FS で起動し、
破壊操作には Human-in-the-loop を必須とすることである
（[OWASP LLM06](https://github.com/OWASP/www-project-top-10-for-large-language-model-applications/blob/main/2_0_vulns/LLM06_ExcessiveAgency.md)）。

## スマホ（Galaxy S26 Ultra）への展開

PC 版 Q4_K_M をスマホでも動かす場合の調査と実機確認結果をまとめる。

### 実機の確認済み事実

ワイヤレス ADB で接続した実機の Runtime Proof:

| 項目 | 値 |
| --- | --- |
| 型番 | SM-S948Q（`m3q` / `m3qjpnw`、日本向け SIM フリー） |
| OS | Android 17（ユーザー訂正。出荷時は Android 16 で、2026-09 に 17 へ更新） |
| ABI | arm64-v8a |
| CPU | ARMv9-A（SME2 / SVE2 / BF16 / i8mm 対応） |
| GPU | Adreno（`/dev/kgsl-3d0`、`ro.hardware.egl=adreno`） |
| RAM | MemTotal 14.7 GiB / **MemAvailable 2.42 GiB**（確認時点） |
| zRAM | SwapTotal 20 GiB |
| ストレージ | /data 空き 304 GB |
| 既存アプリ | Termux 0.118.3、Shizuku、Tasker / AutoInput |
| ビルド | S948QOPU1BZJC |

Samsung 公式 JP 仕様ページのプロセッサ欄はクロック（4.74 GHz / 3.6 GHz）と
オクタコアのみを記載し、SoC 名は記載しない。Snapdragon 8 Elite Gen 5 for Galaxy
（SM8850）/ Adreno 840 はコミュニティ情報であり、SM-S948Q での同一性は推定である。

Qwen3.5-4B Q4_K_M（2,740,937,888 バイト）を
`/sdcard/Download/qwen35-4b-Q4_K_M.gguf` へ adb push 済み（17.1 MB/s、約 2 分 33 秒）。
実機の `/data/data/com.termux` は adb から Permission denied で、
Termux 内部の確認・操作は別途ユーザーの画面操作が必要である。

### 推奨構成

エージェント用途（工具呼び出し・OpenAI 互換 API）では **Termux + llama-server** が
事実上の選択肢である。PocketPal AI は GGUF をオフライン実行でき、
Galaxy S26 Ultra でのベンチ提出もあるが、工具呼び出しと OpenAI 互換
エンドポイントは feature request 段階で、エージェント用途には向かない。
LM Studio は Android 版を提供していない。Ollama も公式 Android アプリはない。

| 用途 | 選択肢 |
| --- | --- |
| チャット・簡易推論 | PocketPal AI（[GitHub](https://github.com/a-ghorbani/pocketpal-ai)、MIT、GGUF 直接読込可） |
| エージェント・API | Termux + llama-server（`--jinja`、`/v1/chat/completions`） |

Termux では公式手順で llama.cpp をビルドできる
（[android.md](https://github.com/ggml-org/llama.cpp/blob/master/docs/android.md)）。

```bash
apt update && apt upgrade -y
apt install git cmake libandroid-spawn
git clone https://github.com/ggml-org/llama.cpp
cmake -B build -DGGML_VULKAN=ON -DGGML_CPU_KLEIDIAI=ON
cmake --build build --config Release -j$(nproc)
cp /sdcard/Download/qwen35-4b-Q4_K_M.gguf ~/
./build/bin/llama-server -m ~/qwen35-4b-Q4_K_M.gguf \
  -ngl 0 -c 8192 -ctk q8_0 -ctv q8_0 \
  --host 127.0.0.1 --port 18080
```

起動前に `MemAvailable` が 2.42 GiB しかないため、他アプリを停止して
空きを 4 GB 以上確保する必要がある。初回は `-c 4096〜8192` から始める。
公式モデルカードは 128K 維持を推奨するが、スマホでは非現実的である。

### バックエンドの注意

8 Elite Gen 5 / Adreno 840 では backend ごとに問題が報告されている。

- Vulkan は量子化 MUL_MAT の誤計算と全層オフロード時の SIGSEGV 例がある。
  `-ngl 63` のように 1 層だけ CPU に残すと動く例もある。
- CPU は Qwen3.5 の Gated DeltaNet ハイブリッドで garbage output の報告がある
  一方、Vulkan では同一モデルが正常だった例もあり、常に正しいとは限らない。
- Hexagon NPU は公称 45 TOPS に対し llama.cpp の実測が約 0.5 TOPS の報告や、
  garbled output の報告がある。エージェント用途では NPU を既定にしない。
- OpenCL は CPU より速く正常な例（24 t/s vs 4.4 t/s）があるがドライバ依存である。

したがって **`-ngl 0`（CPU）と `-dev GPUOpenCL` の両方で固定プロンプトの
出力正しさを確認してから、速度の速い方を採用する** のが安全である。

### 期待速度

| 出典 | 条件 | 速度 |
| --- | --- | --- |
| [PocketPal Leaderboard](https://pocketpal.dev/leaderboard/device/Galaxy%20S26%20Ultra) | S26 Ultra / 4B IQ4_NL | tg 13.19 tok/s、pp 271 tok/s |
| 同 | S26 Ultra / 4B UD-Q4_K_XL | tg 7.10 tok/s |
| [LiteRT-LM モデルカード](https://huggingface.co/litert-community/gemma-4-E2B-it-litert-lm) | S26 Ultra / Gemma 4 E2B GPU | tg 52.1 tok/s（投機的デコーディング時 91.7） |
| [llama.cpp issue](https://github.com/ggml-org/llama.cpp/issues/26957) | S23 Ultra / 2B Q4_K_M CPU | tg 15.48 tok/s |

PC の 64 tok/s に対し、スマホの 4B Q4 は **7〜13 tok/s 程度** と見積もられる。
同一条件の実機ベンチは未実施で、発熱・クロックスロットルで大きく変動する。

## 推奨構成

### 段階 1：既存資産で即座に使う

追加インストールなしで `ollama run qwen2.5:3b` を使う。
実測 87.9 tok/s、100% GPU、消費 VRAM 2.4 GB で最も高速である。
品質より速度を優先する対話・要約・分類タスク向け。

### 段階 2：線形アテンションを活用する本命構成（構築済み）

2026-11-23 時点で本環境に構築済みである。配置先は次のとおり。

| 項目 | パス |
| --- | --- |
| モデル | `~/models/qwen35-4b-Q4_K_M.gguf`（2.74 GB、SHA256 先頭 `00fe7986ff5f6b463e624558`） |
| llama.cpp | `~/.local/lib/llama.cpp/b12359/`（248 MB、CUDA 12.8 ビルド） |
| 起動スクリプト | `~/.local/bin/llm-qwen` |

起動は次の 1 コマンドで行う。

```bash
~/.local/bin/llm-qwen
```

スクリプトは `-ngl 99 -c 16384 -ctk q8_0 -ctv q8_0 --host 127.0.0.1
--port 18080` を固定し、`CUDA_CACHE_MAXSIZE=4294967296` を設定する。
`LLM_PORT` / `LLM_CTX` 環境変数、または追加引数で上書きできる。
`bash -n` / `shellcheck` / `shfmt -d` はすべて通過している。

起動後の検証では `/health` が `{"status":"ok"}` を返し、
工具呼び出し（`get_weather(city=東京)`）も正常だった。
VRAM 使用量は 4.7 GB で、6 GB 内に収まる。
実測の decode は 63.7〜64.9 tok/s である。
初回起動時は PTX JIT で prompt eval が遅れるため、一度暖機してから
プロセスを再起動すると 82〜121 tok/s に回復する。
モデルは [unsloth/Qwen3.5-4B-GGUF](https://huggingface.co/unsloth/Qwen3.5-4B-GGUF)
から取得した。同アーキテクチャの RTX 2060 6GB での
[56.1 tok/s 報告](https://github.com/ggml-org/llama.cpp/issues/29513)もある。

### 段階 3：真のスパースアテンションを使う場合

Hopper（H100/H200）または Blackwell と、数十〜数百 GB の VRAM が必要である。
GTX 1660 SUPER では構成変更では解決しない。62GB RAM を活かす
MoE CPU オフロードは 17〜20 tok/s の報告があり、実用性は段階 2 に劣る。

## 未確認事項

- BFCL / SWE-bench / TAU2-Bench の独立スコア。モデルカードの自己報告のみで、
  自前のゴールデンセットによる定量評価は未実施。
- 量子化レベル間の品質差。速度・VRAM・工具呼び出しは実測したが、
  IQ4_XS / Q4_K_M / Q5_K_M のタスク成功率は比較していない。
- 日本語長文のコーディング品質。実測は日本語の工具呼び出し・JSON ・
  天気回答に限られる。
- コミュニティ蒸留版のタスク成功率。実機では工具呼び出し・JSON・
  tool response 追従が正常なことを確認したが、ゴールデンセットによる
  定量比較は未実施。Jackrong 4B は JSON の内容に不自然な項目が見られた。
- 32K / 128K コンテキストでの安定性と VRAM 消費。
- Ollama を最新版へ更新した場合の `qwen3.5:4b` / `gemma4:e2b` pull 可否。
- スマホ実機での llama-server 起動・速度・工具呼び出しの実測。
  モデル転送と端末情報の確認のみ実施し、Termux 内部は adb から
  Permission denied のため未検証。
- スマホのバックエンド別（CPU / OpenCL / Vulkan / Hexagon）速度と
  出力正しさの実機比較。
- 2B QSA 実験モデルの sm_75 動作と実用速度。
- 長時間稼働時の発熱・消費電力と、JIT キャッシュの永続性。

## 参照

- [DeepSeek-V3.2-Exp](https://github.com/deepseek-ai/DeepSeek-V3.2-Exp)
- [FlashMLA](https://github.com/deepseek-ai/FlashMLA)
- [vLLM DSA 対応 issue](https://github.com/vllm-project/vllm/issues/66960)
- [SGLang sm75 廃止 RFC](https://github.com/sgl-project/sglang/issues/6006)
- [SGLang HiSparse guide](https://github.com/sgl-project/sglang/blob/main/docs/docs/advanced_features/hisparse_guide.mdx)
- [llama.cpp の Qwen3.5 サポートコミット](https://github.com/ggml-org/llama.cpp/commit/fc0fe4004985d6749a7a05e250d161f9dbe41d65)
- [llama.cpp の GDN CUDA カーネル](https://github.com/ggml-org/llama.cpp/blob/master/ggml/src/ggml-cuda/gated_delta_net.cu)
- [llama.cpp の DSA KV キャッシュ](https://github.com/ggml-org/llama.cpp/blob/master/src/llama-kv-cache-dsa.cpp)
- [llama.cpp の Gemma 4 chat parser](https://github.com/ggml-org/llama.cpp/pull/21418)
- [Ollama qwen3.5 ライブラリ](https://ollama.com/library/qwen3.5)
- [Qwen3.5-4B モデルカード](https://huggingface.co/Qwen/Qwen3.5-4B)
- [Gemma 4 E2B モデルカード](https://huggingface.co/google/gemma-4-E2B-it)
- [QwenLM/Qwen3.8 系譜](https://github.com/QwenLM/Qwen3.8/blob/main/README.md)
- [unsloth Qwen3.5-4B GGUF](https://huggingface.co/unsloth/Qwen3.5-4B-GGUF)
- [HauhauCS Qwen3.5-4B Uncensored](https://huggingface.co/HauhauCS/Qwen3.5-4B-Uncensored-HauhauCS-Aggressive)
- [2B QSA 実験](https://huggingface.co/tvall43/Qwen3.5-2B-heretic-QSA-experiments)
- [Gemma 4 E2B 日本語 imatrix](https://huggingface.co/dahara1/gemma-4-E2B-it-UD-japanese-imatrix)
- [Heretic プロジェクト](https://github.com/p-e-w/heretic)
- [OWASP LLM06 Excessive Agency](https://github.com/OWASP/www-project-top-10-for-large-language-model-applications/blob/main/2_0_vulns/LLM06_ExcessiveAgency.md)
- [KTransformers kt-kernel](https://github.com/kvcache-ai/ktransformers/blob/main/kt-kernel/README.md)
- [NVIDIA CUDA 13.0 リリースノート](https://docs.nvidia.com/cuda/archive/13.0.0/cuda-toolkit-release-notes/index.html)
