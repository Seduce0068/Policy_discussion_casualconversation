---
type: Chat
title: "ナノカーボン半導体は2030年代後半に量産されるか"
description: "CNTとグラフェンナノリボンの最新成果、NRAMの原理相違による頓挫を整理し、2030年代後半の商用化を評価した"
tags:
  - chat
  - semiconductor
  - carbon-nanotube
  - graphene
generated:
  by: opencode
  at: "2026-10-23T12:43:23+09:00"
status: "draft"
---

## 文脈

- 2026-10-23 の雑談。ナノカーボン半導体、特にCNTトランジスタを有望視していたが、
  2030年代後半に量産商用化されるかという疑問が出た。
- 「ナノカーボン」はCNT、グラフェン、カーボンナノホーンなどを含むが、
  トランジスタのチャネル材料として現実的な候補はCNTである。
  グラフェンはバンドギャップを持たないため、論理素子にはナノリボン化などの別系統が必要。
- 「量産商用化」を、(1) 先端ロジックの高集積デバイス、(2) ニッチ半導体製品、
  (3) 半導体製造向け材料、の3層に分けて評価する。

## 結論

- 先端ロジック（CPU/GPU/AIアクセラレータ）のCNT量産は、2030年代後半でも
  実現確率は低い。筆者の肌感覚で5〜10%程度。
- CNTトランジスタを使ったニッチ製品の商用化は、2030年代後半に十分あり得る。
  60〜70%程度。
- CNTを半導体製造の材料・部品として使う動きはすでに商用化が始まっており、
  ほぼ確実に拡大する。
- 期待が外れるというより、勝ち筋が「Siの置き換え」から
  「Siが苦手な場所への挿入」にずれる、というのが正確なところ。

## 研究は着実に前進している

- TSMC/Stanford は 2025 VLSI で CNTナノシートPFETを報告。
  Imax 0.9 mA/µm、Imin 20 pA/µm、SS 93 mV/dec（VDS -0.5V）。
- TSMC は 2024 年に、配向CNTアレイでN型/P型の等性能MOSFETを報告。
  接触抵抗は 5.1 kΩ/CNT（接触長20nm、250 CNT/µm換算で20.4 Ω·µm）。
- 北京大学は 2025-10 に、配向CNTで fmax 1.024 THz、ft 551 GHz を達成
  （Lg 35nm、Y型ゲート）。Nature Electronics に掲載。
- 北京大学は 2026-06 に、全CNTのCFETアーキテクチャで5段リング発振器と
  4T-SRAMを試作。CFETはIRDSが2032年頃の実用化を予測する構造。
- SkyWater は 2025-12 の IEDM で、90/130nm・200mmウェハ上に
  CNFET + RRAM のモノリシック3Dを形成（≤415°C）。読み出し帯域4倍。
- imec は 2025 IEDM で、配向CNTのバイオFETアレイを報告。
- NEC は CNT赤外イメージセンサーの2026年実用化を目指す。
- ただし、CNT TPU（北京大学、2024）は3,000トランジスタ。
  最先端Siの1チップは10^10〜10^11トランジスタ規模であり、桁が6〜7違う。

## 量産ロードマップにCNTは入っていない

- imec の2026年ロードマップは A14（2028）→ A10（2030、CPPスケーリング停止）
  → A7（CFET開始、0.7nm級）→ A5（2035-36）→ A3（2038）。
- 2D材料はA7でBEOL周辺デバイスへの挿入が検討されているが、
  CNTは先端ロジックの生産ロードマップに置かれていない。
- TSMC は A16（2027）→ A14（2028）→ A13/A12（2029）を提示。
  CNTは研究テーマであり、生産ノードの計画ではない。
- IRDS は CFET を2032年頃の実用化と予測。CNTはBeyond CMOSの探索枠。
- つまり、2030年代後半のSi側の受け皿は「CFET + 2D材料 + BSPDN + 3D実装」で、
  CNTが入る隙間は限定的。

## ボトルネックは物理より製造

- カイラリティ制御: 単層CNTは約1/3が金属性。ロジックには99.99%〜99.9999%級の
  半導体性分離が必要。分離技術はあるが、コストと検査が重い。
- 密度と配向: 200〜500 CNT/µmが必要。実験室では達成例があるが、
  ウェハ全面で均一に配置する技術は未確立。
- 接触抵抗とばらつき: 5.1 kΩ/CNTでも大きく、直径・カイラリティ分布が
  Vtばらつきに直結する。
- ドーピング: Siのイオン注入に相当する安定したp/n制御がない。
  誘電体ドーピングやドーピングフリーCMOSは再現性と信頼性が課題。
- 検査・計測: ウェハ全面でppm以下の金属性CNTを高速に検査する
  インライン手法は確立していない。
- 信頼性: ヒステリシス、界面準位、BTストレス、長期劣化のデータが薄い。
- エコシステム: 300mm対応の成膜・転写装置、PDK、EDA、歩留まり学習が未成熟。
- Lam Research の David Fried は2021年の時点で、均一性と形成の課題は
  「Si MOSFETの水準に達するとは思えない」と述べている。2026年時点でも
  本質的な構図は変わっていない。

## 勝ち筋は「置き換え」ではなく「挿入」

- EUVペリクル: CNT膜は透過率と耐久性の両立が可能。三井化学とimecが提携し、
  年5,000枚規模の専用設備を計画（2026-03時点の業界報道）。
  LINTECも2026-02にCNTペリクルを発表。これは半導体製造の消耗品であり、
  すでに商用化フェーズ。
- BEOLモノリシック3D: CNTは400°C以下で形成できる。Si CMOSの上に
  低消費電力のロジックやメモリを積む用途に向く。SkyWaterの実証はこの線。
- センサー: バイオFET、赤外イメージセンサー。CNTの表面特性と低ノイズが効く。
- RF/THz: 1 THz級のfmax。6G/THz通信のニッチ。InP系との競合は残る。
- 量子: C12がCNT量子プロセッサのロードマップを2026年に公表。
- フィルム/TFT: フレキシブル、ディスプレイ、塗布型エレクトロニクス。
- 逆に、NRAMはNanteroが2024年に事業停止。加えて、日本ゼオンのNEDO案件も
  2024年度末に中止。CNTメモリは事業化と原理の両面で難しさを示した。

## タイムラインと確率

筆者の肌感覚であり、精密な予測ではない。

| 時期 | 先端ロジック | ニッチ半導体 | 製造材料 |
| --- | --- | --- | --- |
| 2026-2030 | CNTは研究・試作 | ペリクル、センサー、TFT | ペリクル量産開始 |
| 2030-2035 | Si CFET/2Dが主役 | BEOL CNTの商用PoC | CNT膜の採用拡大 |
| 2035-2039 | CNTの割り込みは限定的 | ニッチ量産が成立し得る | 標準材料化 |
| 2039以降 | 置き換えは依然不透明 | 用途次第で拡大 | 確実 |

- 先端ロジックのCNT量産（2039年まで）: 5〜10%。
- CNTトランジスタ製品のニッチ商用化（2039年まで）: 60〜70%。
- 半導体製造でのCNT材料の商用化: 95%以上（すでに開始）。

## 監視指標

- ファウンドリが300mmのCNTパイロットラインとPDKを公表するか。
- CNT CFETで10^5ゲート級の回路とSRAM歩留まりデータが出るか。
- 金属性CNTのインライン計測装置が実用化されるか。
- BEOL 3Dメモリ/ロジック、6G RF、量子のいずれかで量産引き合いが出るか。
- 2D材料（MoS2等）が接触・ばらつきを先に解決しないか。
  先に解決されるとCNTの挿入余地はさらに狭くなる。

## 日本の立ち位置

- 材料・部品では強い。三井化学、LINTEC、日本ゼオン、名城ナノカーボン、
  産総研、九州大学（JST ALCA-Next 2025-2028）など。
- ただし先端ロジックのファウンドリがない。日本で現実的なのは
  CNT CPUではなく、ペリクル、放熱材、センサー、RF部品、製造装置。
- 「ナノカーボン半導体」を先端ロジックの物語として追うと裏切られるが、
  材料・部品の物語として追うと着実に進んでいる。

## 追記: グラフェンナノリボンとNRAMの原理問題

### グラフェンナノリボン（GNR）

- GNRはグラフェンを数nm幅の帯状に切ったもの。幅とエッジ構造でバンドギャップを
  制御できる。アームチェア型は幅Nで3つの族に分かれ、9-AGNRや13-AGNRが
  半導体になる。
- 利点: 原子レベルで平坦、高移動度、バンドギャップを設計できる、放射線耐性。
- 課題:
  - 合成: ボトムアップ（Ullmann結合）は原子精度が高いが、金属基板上での
    合成が主流で、ウェハへの転写が必要。長さは数十nm程度。
  - 幅・エッジ制御: 1原子の幅差でバンドギャップが変わる。エッジ欠陥、酸化、
    水素終端が特性を支配する。
  - コンタクト: 準1次元物質の端に低抵抗コンタクトを付けるのが難しい。
  - 密度・配向: CNTよりさらに難しい。単一デバイスの研究が主流。
  - 計測・信頼性: 原子精度の評価と長期安定性のデータが不足。
- 現状: 2026-02のNature Reviews Materialsレビューは、高移動度と調整可能な
  バンドギャップを評価しつつ、精密な評価、クリーンな転写、信頼できる
  電気的コンタクトを課題として挙げる。2024年には9-AGNR FETの長期安定性、
  2023年にはh-BN封止のエッジコンタクトが報告された。2026年には放射線耐性や
  ツイスト制御の報告もあるが、いずれも単一デバイス・小規模。
- 評価: 先端ロジックの量産はCNTよりさらに遠い。2039年までの確率は2〜5%程度。
  放射線耐性、センサー、量子、極低電力のニッチなら可能性はある。

### NRAMの「原理が違った」件

- 2025-05-30、経済産業省はNEDO GI基金のNRAM開発を2024年度末で中止すると発表。
  日本ゼオン、Nantero、富士通セミコンダクターメモリソリューション（現RAMXEED）、
  産総研、広島大学などが参画。当初は2021〜2030年度、総事業費約91億円、
  助成約56億円の計画だった。
- 当初の想定: CNT同士の接触・分離（静電引力とファンデルワールス接着）による
  機械的な抵抗変化。高速・不揮発・低電力でDRAM代替を狙った。
- 実際: 動作原理と故障原因を調べると、ReRAMと同様の抵抗変化機構だった。
  書き換え回数の限界は約10^6回で、DRAM並みの10^12回には届かない。
- 2024-12に中止を決定。NEDO委員の助言で原理解明を前倒ししたことが、
  早期中止につながった。これは「原理を早く解明する」ことの成功例でもある。
- 教訓:
  - 初期の「原理」は、理想化された単一デバイスのI-Vから作った仮説にすぎない。
  - 実デバイスでは、界面準位、欠陥、ジュール熱、酸化、電極金属の移動など、
    別の機構が支配することがある。
  - 製品化の前に、温度依存性、面積依存性、エンデュランス、リテンション、
    ばらつき、故障解析をセットで示す「メカニズム・ゲート」が必要。
  - 単一デバイスのデモと、量産デバイスの支配機構は別物である。

### 評価への影響

- GNRを加えても、2030年代後半の先端ロジック量産という結論は変わらない。
  むしろGNRはCNTより製造成熟度が低く、確率は下がる。
- CNTはGNRより先に「配向アレイ」「コンタクト」「BEOL低温プロセス」の
  実証が進んでいる。炭素系で最初に量産に近づくのはCNT。
- ただし、CNTもGNRも、次のゲートは「原理の説明」ではなく
  「量産スケールでの支配機構の特定」になる。

## 参照

- [imec 2026 roadmap（Tom's Hardware、2026-06-29）](https://www.tomshardware.com/tech-industry/semiconductors/imecs-2026-roadmap-details-0-3nm-nodes-by-2038-cfet-transistors-become-viable-at-0-7nm-company-redefines-moores-law-as-cell-sizes-gain-importance-for-density)
- [TSMC A13発表（TSMC、2026-04-22）](https://pr.tsmc.com/english/news/3302)
- [TSMC/Stanford CNT nanosheet PFET（VLSI 2025）](https://doi.org/10.23919/vlsitechnologyandcir65189.2025.11074843)
- [TSMC CNT研究ページ](https://research.tsmc.com/page/logic/94.html)
- [北京大学 THz CNT（2025-10-09）](https://news.pku.edu.cn/info/2881/1294871.htm)
- [北京大学 CNT CFET（2026-06-22）](https://news.pku.edu.cn/info/2881/2604501.htm)
- [北京大学 CNT TPU（2024-07-24）](https://newsen.pku.edu.cn/news_events/news/research/14371.html)
- [SkyWater M3D（IEDM 2025）](https://doi.org/10.1109/iedm50572.2025.11353729)
- [imec CNTバイオFET（2026-07-22）](https://www.imec-int.com/en/articles/towards-scalable-single-molecule-biosensing-using-carbon-nanotube-fets)
- [NEC CNT赤外イメージセンサー](https://www.nec.com/en/global/rd/technologies/cnt/index.html)
- [Nantero（Wikipedia）](https://en.wikipedia.org/wiki/Nantero)
- [imec/三井化学 CNTペリクル提携](https://www.imec-int.com/en/press/imec-and-mitsui-chemicals-sign-strategic-partnership-agreement-commercialize-cnt-pellicle)
- [EUVペリクル市場調査（2026-03）](https://www.knowledge-sourcing.com/report/euv-pellicle-market)
- [JST ALCA-Next バンドル型CNTトランジスタ（2025-2028）](https://projectdb.jst.go.jp/grant/JST-PROJECT-25137101/)
- [SemiEngineering "Chasing After Carbon Nanotube FETs"（2021-03-18）](https://semiengineering.com/chasing-after-carbon-nanotube-fets)
- [NRAM開発中止（マイナビ、2025-06-02）](https://news.mynavi.jp/techplus/article/20250602-3343340)
- [NRAM開発中止の背景（ニュースイッチ、2025-05-30）](https://newswitch.jp/p/45868)
- [GNRレビュー（ETH Zurich、2026-02）](https://ee.ethz.ch/news-and-events/d-itet-news-channel/2026/02/the-future-of-nanoelectronics-new-insights-into-graphene-nanoribbons-in-nature-reviews-materials.html)
- [GNRエッジコンタクト（ACS Nano、2023）](https://pubmed.ncbi.nlm.nih.gov/37578964)
- [9-AGNR FETの長期安定性（2024）](https://www.osti.gov/servlets/purl/2469669)
- [GNRの材料科学課題（ACS Nano、2021）](https://pubs.acs.org/doi/abs/10.1021/acsnano.0c07835)
- [GNRレビュー（Springer、2024）](https://link.springer.com/article/10.1007/s44214-024-00050-8)
- [GNRの放射線耐性（phys.org、2026-07）](https://phys.org/news/2026-07-graphene-nanoribbons-survive-gamma-revealing.html)
