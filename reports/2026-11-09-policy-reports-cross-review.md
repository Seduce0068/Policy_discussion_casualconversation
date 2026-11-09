---
type: Report
title: "政策レポート3本の横断整合性レビューと会計標準 v2→v3 差分評価"
description: "会計透明化・放送透明性・教育DXの3レポートを横断比較し、共通設計原則・相違点・統合課題を整理するとともに、会計標準の v2 から v3 WD 0.3 への変更を要件レベルで評価した横断レビュー"
tags:
  - cross-review
  - interoperability
  - standardization
  - governance
  - provenance
generated:
  by: opencode
  at: "2026-11-09T20:16:00+09:00"
status: draft
---

## 1. 目的と範囲

本レポートは、同日に取り込んだ次の3本の政策・設計レポートを横断比較し、共通する設計原則、
相互に補完できる要素、衝突する設計判断、および欠落している論点を整理する。あわせて、
会計標準の旧版 v2 と現行 v3 WD 0.3 の差分を要件レベルで評価する。

対象文書（行番号は各ファイル先頭からの通し。2026-11-09 時点）。

| 略称 | 文書 | 状態 |
| --- | --- | --- |
| 会計v3 | [公共・非営利団体会計情報公開標準 v3（Requirements Draft WD 0.3）](2026-11-09-public-nonprofit-accounting-standard-v3-wd03.md) | draft |
| 放送 | [放送ニュース透明性制度 現段階フル版](2026-11-09-broadcast-news-transparency-governance.md) | draft |
| 教育 | [オープン教材・Markdown・GitHub・AI家庭教師による教育DX構想](2026-11-09-education-dx-open-textbook-ai-tutor.md) | draft |
| 会計v2 | `public_nonprofit_accounting_standard-1.md`（2026-09-20、リポジトリ外の参照資料） | 参照 |

会計v2 はリポジトリ外（userscript リポジトリ）に置かれた旧ドラフトである。本レビューでは
SHA-256 `d29097248e2eb4c9db01f2325e2e6fdd2e56095c1b642793ce3daf5c76a8730b` の内容を参照した。

本レビューは単一モデル（opencode / DeepSeek V4.1 Flash）による机上レビューであり、
複数モデル・第三者による独立検証ではない。この点は、放送レポート自身が定める
Cross-review Policy（放送 L128-141）の要件を満たさない。限界は第8章に整理する。

## 2. 3レポートの位置づけ

| 項目 | 会計v3 | 放送 | 教育 |
| --- | --- | --- | --- |
| ドメイン | 公共・非営利団体の会計情報公開 | 放送報道の情報種別透明化 | オープン教材とAI家庭教師 |
| 機械可読な正本 | `registries/`・`requirements/`・`schemas/`（会計v3 L51-62） | Authority Matrix 管理の文書群と manifest/digest（放送 L51, L86-94） | Markdown + YAML frontmatter（OKF）（教育 L119-167, L597-632） |
| 人間向けビュー | Markdown 本文・表は Informative View（会計v3 L62） | 文書そのもの（放送 L53-66） | HTML/PDF/EPUB を生成（教育 L173-176, L185） |
| 現在の段階 | WD 0.3 / Requirements Closure Candidate（会計v3 L19, L64-72） | implementation-planned（放送 L21-22） | 構想・実現可能性評価（教育 L889-891） |
| 検証の実装 | 3層検証を実装、Reference Validator 未実装・外部相互運用未実施（会計v3 L271-293, L48-49） | Semantic Validator 仕様のみ・3モデルCross-review未実施（放送 L190-208, L237） | markdownlint・人的検証・実証指標（教育 L196, L697, L780-793） |
| ガバナンス | 要件管理・Open Issues のみ（会計v3 L162-195, L513-528） | Authority Matrix・Release Policy・Cross-review・Issue State（放送 L53-68, L123-188） | 教員・人間によるレビュー中心（教育 L318, L629, L697） |

### 2.1 共通する設計原則

3本は対象ドメインが異なるが、次の4原則で一致している。

1. **構造化データを正本とし、PDF/HTMLは派生物とする。**
   会計v3 は Structured data を機械利用の中心に置き PDF を派生物とする（会計v3 L124, L144,
   L530-533）。教育は Markdown を「編集・管理の原本」、PDF を「出力形式」とする
   （教育 L173-176）。放送は文書ごとの正本を Authority Matrix で管理し、manifest/digest で
   成果物を固定する（放送 L51, L86-94）。
2. **来歴・完全性の記録により検証可能性を確保する。**
   会計v3 は JWS/JCS/RFC 3161/Checkpoint まで規定し（会計v3 L377-453）、放送は manifest と
   digest、署名、Evidence で担保する（放送 L86-126）。教育は生成時刻・検証方法の
   メタデータ記録に留まる（教育 L623-629）。
3. **段階的に導入する。**
   会計v3 は Tier 1〜3 の累積プロファイル（会計v3 L135-161）、放送は implementation-planned
   からの次工程（放送 L17-22, L241-254）、教育は Phase 0〜5 のロードマップ
   （教育 L726-796）を持つ。
4. **既存の国際・業界標準を再利用する。**
   会計v3 は BCP 14・JSON Schema・DCAT 3・CSVW・XBRL OIM・JOSE（会計v3 L100-115,
   L295-318, L472-480）、放送は JSON Schema・SemVer・OpenPGP・Git・OSF（放送 L89, L96-105,
   L107-121, L220-223）、教育は OKF・Markdown・CommonMark・LTI/QTI/CASE/OneRoster
   （教育 L119-167, L235-239）を採用する。

## 3. 横断整合性マトリクス

| 論点 | 会計v3 | 放送 | 教育 | 判定 |
| --- | --- | --- | --- | --- |
| 正本の形式 | YAML Registry + JSON Schema（L51-62） | Authority Matrix + manifest/digest（L51-66, L86-94） | Markdown + YAML frontmatter（L119-167） | 相違（用語・置き場所） |
| PDF/派生物 | 派生物（L124, L144, L530-533） | 言及なし | 出力形式（L173-176） | 補完（放送は未規定） |
| 識別子 | DCAT identity + Requirement ID lifecycle（L263-265, L295-318） | document_id / manifest / snapshot digest（L86-94） | 教材ID・問題ID・学習指導要領コード（L610-614, L740） | 補完 |
| バージョニング | WD番号 + 独自成熟度モデル（L17-19, L64-72） | SemVer 定義（L96-105） | Git 履歴 + 版数（L317, L832） | 相違 |
| digest・正規化 | JCS 正規化後の SHA-256（L391, L407-408, L425） | raw bytes の SHA-256、canonicalization なし（L90） | 規定なし（原本の digest は参照欄のみ L957） | 相違（高リスク） |
| 署名・信頼 | JOSE ES256 / JWS / trust anchor（L377-421, L562-581） | OpenPGP + allowlist + Git 署名（L107-121） | 規定なし | 相違（高リスク） |
| タイムスタンプ | RFC 3161 を Tier 3 MTI（L423-441, L578-580） | 時刻 source / timezone / skew の記録のみ（L210-213） | メタデータのみ（L623-629） | 相違（会計が先行） |
| 検証階層 | Schema / Semantic / Coverage + Closure Gate（L271-293, L492-511） | Schema + Semantic Validator 仕様 + Exit Criteria（L175-208） | markdownlint + 人的検証（L196, L697） | 非対称 |
| 適合性・受入 | Conformance Targets + Tier（L137-161） | Release Exit Criteria 10項目（L175-188） | 実証評価指標（L780-793） | 補完（目的が異なる） |
| ガバナンス | 要件管理・Open Issues（L162-195, L513-528） | Authority Matrix・Repository Enforcement（L53-68, L123-126） | 人的レビュー（L318, L629） | 補完 |
| レビュー主体 | Qwen3.8/GPT クロスレビュー実績（L24） | qwen/claude/chatgpt 必須 slot・隔離 first pass（L128-141） | 教員・人間（L318, L697, L759） | 相違（教育は弱い） |
| 語彙・スキーマ | DCAT 3 / CSVW / JSON-LD / XBRL OIM / JSON Schema（L295-318, L472-480） | JSON Schema Draft 2020-12 / manifest schema（L89, L277） | OKF / 教育拡張 / LTI / QTI / CASE（L119-167, L235-239） | 補完（レイヤ違い） |
| プライバシー | disclosure class / SDC / threat model（L320-348, L482-490） | データ分類・アクセス制御（L215-218） | 分離原則のみ（L327, L527-532） | 非対称（会計が先行） |
| ライセンス・権利 | rights statement を Tier 1 要件化（L156, L181） | 著作権素材のアクセス制御（L153, L217） | CC 表と `license` フィールド（L428-467, L621） | 補完 |
| 未解決管理 | Open Issues OI-01〜12（L513-528） | 未解決11件 + Issue State Model（L155-173, L256-268） | リスク表のみ（L694-705） | 非対称 |

## 4. 重要な相違と対応方針

### 4.1 署名・ダイジェストプロファイルの非互換（重要度: 高）

会計v3 は RFC 8785 JCS で正規化したバイト列に ES256 JWS を適用し、その outer artifact に
RFC 3161 を適用する（会計v3 L391, L407-408, L425）。放送は SHA-256 を raw bytes に対して
計算し、canonicalization なしと明記する（放送 L90）。署名は初期 backend を OpenPGP に限定する
（放送 L109）。教育には署名・digest の規定がない。

このままでは、同一成果物を3プロファイルで検証する共通 Validator が作れない。特に JSON/YAML
bundle の digest 再現性が揃わず、監査証跡の相互運用が阻害される。対応として、次の2層に
分離するのが妥当である。

1. **content digest 層**: 正規化（JCS 等）後のバイト列に対する SHA-256 に統一する。
2. **release signature 層**: リポジトリ運用に合わせて OpenPGP と JWS の両対応とし、
   検証手順（allowlist・失効・履歴鍵）を共通化する。

### 4.2 正本の置き場所・呼称の不一致（重要度: 高）

会計v3 はパッケージ内 YAML Registry を正本とし、Markdown を Informative View とする
（会計v3 L51-62）。ただし本文は「同梱 `spec/`」と表記する（会計v3 L21）が、実際のパッケージ
構成は `registries/` と `requirements/` であり、表記が一致していない。放送は Authority Matrix で
文書ごとの正本を定め、Git repository と OSF Registration を役割分担する（放送 L51, L220-223）。
教育は Markdown を原本、Git を履歴基盤とする（教育 L173, L312-325, L867-873）。

「著者正本（authoritative source）」「派生物（informative view）」「リリース成果物（package）」
の区別が文書ごとに異なるため、共通用語集を定義し、この3層モデルへの対応付けを行うべきである。

### 4.3 バージョニング・成熟度の不統一（重要度: 中）

会計v3 は WD 番号と独自成熟度モデル（Requirements Draft → Candidate Specification →
Stable Release）を使う（会計v3 L17-19, L64-72）。放送は SemVer を定義し、`0.x.y` と `1.0.0`
の条件、日付 revision との混在禁止を定める（放送 L96-105）。教育は Git 履歴と版数のみで
SemVer を持たない（教育 L317, L832）。

レポート間の依存 pin（例: 教材メタデータが会計の DCAT プロファイルを参照する場合）を表現
できないため、SemVer を外部向け contract に限定し、WD/Phase は内部成熟度として対応表を
持たせる。Requirement ID lifecycle（会計v3 L263-265）と Issue state（放送 L155-173）の
状態対応表も必要である。

### 4.4 検証・レビューの非対称（重要度: 中）

会計v3 は3層検証と Closure Gate 13条件を持つ（会計v3 L271-293, L492-511）が、Reference
Validator は未実装で、外部相互運用テストも未実施である。PASS は生成ソースの検証に限定
される（会計v3 L48-49, L661）。放送は Semantic Validator の仕様を持つが実装未了で、3モデル
Cross-review も未実施である（放送 L190-208, L237, L175-188）。教育は markdownlint と人的
検証、実証評価指標のみで、構造・意味の検証規定を持たない（教育 L196, L697, L780-793）。

教育パイプラインには最低限、メタデータ schema 検証、ID 重複検査、学習指導要領コードの
被覆検査、`license` 必須検査を追加すべきである。あわせて3本共通で reviewer metadata の
記録方法を定める。

### 4.5 レビュー主体と独立性（重要度: 中）

放送は qwen-family / claude-family / chatgpt の3モデルを必須 slot とし、
cross-exposure-isolated first pass を要求する（放送 L128-141）。一方、3レポート自体は
GPT-6 と GPT-5.6 Sol が生成したもので、レビュー実行のメタデータは記録されていない
（各 frontmatter の `generated` のみ）。会計v3 は「Qwen3.8 と GPT のクロスレビュー」実績に
言及する（会計v3 L24）が、手続は定義していない。

放送レポートが定める検証原則を、3レポート群自身が満たしていない状態である。本レビューも
単一モデルであり独立検証ではない。3レポートに Cross-review Policy を適用し、provider・
model・digest・実施時刻を記録する必要がある。

### 4.6 プライバシー・権利メタデータの粒度差（重要度: 中）

会計v3 は donor linkability の禁止、統計的開示制御、disclosure class、公開承認の最小化まで
規定する（会計v3 L320-348）。放送は public/internal/confidential/source-protected の分類を
持つ（放送 L215-218）。教育は名簿・成績・学習履歴の分離と Git 混在禁止、ライセンス表と
`license` フィールドに留まる（教育 L327, L527-532, L428-467, L621）。

教育の教材メタデータにも disclosure class と rights statement（DCAT/SPDX）を追加し、
会計・放送の分類語彙へマッピングするのが望ましい。

### 4.7 タイムスタンプの第三者性（重要度: 低〜中）

会計v3 は RFC 3161 を Tier 3 MTI とし、JWS の outer artifact に適用する
（会計v3 L423-441, L578-580）。放送は `generated_at` の時刻 source・timezone・許容 skew を
記録する（放送 L210-213）が、第三者タイムスタンプを要求しない。会計v2 は「自己署名・
自己生成のタイムスタンプのみでは第三者性を欠く」と明記していた（会計v2 L305-315）。

放送の Release にも RFC 3161 または透明性ログ（会計v3 の Checkpoint backend と共通化）を
検討すべきである。

## 5. 補完関係と統合アーキテクチャ案

3本は競合する仕様ではなく、異なるレイヤを担う補完関係にあると整理できる。

| レイヤ | 主担当 | 主要な成果物 | 他レポートを補完する点 |
| --- | --- | --- | --- |
| コンテンツ層 | 教育 | OKF/Markdown 運用、教材・問題ID、LTI/QTI、PDF 変換 | 会計・放送の人間向けビュー生成 |
| データ層 | 会計v3 | DCAT/CSVW/XBRL、JWS/JCS/RFC 3161、Tier | 放送の Baseline/意味論、教育の教材データ配布 |
| リリース・検証層 | 放送 | Authority Matrix、manifest/digest、Cross-review、Issue State、Evidence | 会計のレビュー工程・Issue 管理、教育の公開手続 |

- 放送の Cross-review Policy・Issue State Model・Evidence Policy（放送 L128-153, L155-173）は、
  会計v3 の Closure Gate と教育の人的レビューを補完する。
- 会計v3 の Requirement Registry・Blocker・Coverage Engine（会計v3 L162-195, L271-293）は、
  放送の Semantic Validator 実装（放送 L190-208）の雛形として再利用できる。
- 教育の OKF/Markdown は、会計・放送の Informative View 生成パイプラインとして使える。

3本で重複する共通要素（識別子、digest、manifest、バージョニング、reviewer metadata、
issue/evidence model、disclosure/rights metadata）を小さな共通プロファイルとして切り出すのが
効率的である。

## 6. 会計標準 v2 → v3 WD 0.3 差分評価

### 6.1 位置づけの変化

会計v2 は 460行・14章の標準「案」で、JSON インスタンス例、ディレクトリ構成、API 例、
Tier 制、運用ツール要件を含む実務寄りの提案だった（会計v2 L14-460）。会計v3 は 662行・
24章で、16 Blocker・89 Requirements・5 Registry・JSON Schema 6点・Validation Pipeline を
伴う「要件仕様」へ移行した（会計v3 L51-62, L162-195, L535-629）。全要件は BCP 14 の
Atomic Requirement として管理される（会計v3 L100-115, L227-269）。

評価としては、v2 の「方針と例」を v3 が「検証可能な義務」へ変換した質的転換であり、
方向は妥当である。

### 6.2 章対応

| v2 | v3 | 判定 |
| --- | --- | --- |
| §0 改訂履歴（会計v2 L3-12） | §0 文書状態・成熟度（会計v3 L27-73） | 移行 |
| §1 基本方針（会計v2 L14-37） | §1 Scope/Non-goals（会計v3 L74-98）、§22 PDF（L530-533） | 維持 |
| §2 適用レベル（会計v2 L39-48） | §4 Conformance / Tier（会計v3 L135-161） | 強化 |
| §3 標準スタック（会計v2 L50-106） | §6/§9/§18、Tier 定義（会計v3 L156-158） | 分割・強化 |
| §3.2.1 JSON-LD 永続性（会計v2 L80-90） | B-02 context-versioning（会計v3 L548） | 一部後退 |
| §4 データモデル・プライバシー（会計v2 L108-178） | §10 Privacy、§11 Accounting（会計v3 L320-375） | 強化（判断変更あり） |
| §5 承認ログ・外部監査（会計v2 L180-198） | §10.3、AuditArtifact、B-10（会計v3 L148, L187, L332-334） | 判断変更 |
| §6 正本データ例（会計v2 L200-251） | §7.1 要件例（会計v3 L229-246） | 置換（例が要件に） |
| §7 Web 正本・構成（会計v2 L253-279） | §9 DCAT、§15 Checkpoint（会計v3 L295-318, L443-453） | 抽象化 |
| §8 変更履歴（会計v2 L281-299） | §7.3 / §9 / §17（会計v3 L263-265, L306-315, L466-470） | 強化 |
| §9 改ざん検知（会計v2 L301-339） | §12-14、B-05〜B-07（会計v3 L377-441） | 大幅強化 |
| §10 API（会計v2 L341-363） | §16 API / DataService（会計v3 L455-464） | 強化 |
| §11 PDF 位置付け（会計v2 L365-388） | §22 Informative（会計v3 L530-533） | 維持（縮小） |
| §12 実装ガイド（会計v2 L390-418） | §22（会計v3 L530-533） | 縮小（Informative） |
| §13-14 最終目標・理念（会計v2 L420-460） | §1、冒頭（会計v3 L24-25, L74-98） | 弱体化 |

### 6.3 強化された点

1. **要件管理の規律**: BCP 14 authoring subset、Atomic Requirement、Requirement ID
   lifecycle、Registries、Closure Gate 13条件（会計v3 L100-115, L227-269, L492-511）。
   v2 には検証アーキテクチャが存在しなかった。
2. **暗号・完全性**: ES256/JWS/JCS/RFC 3161/trust anchor/Checkpoint/長期検証
   （会計v3 L377-453）。v2 の「ハッシュ + 署名」から、鍵発見・失効・長期検証まで規定した。
3. **同一性・版管理**: DCAT 3 の DatasetSeries/Logical/Immutable Version、previousVersion、
   correction/redaction/retention（会計v3 L295-318, L466-470）。
4. **プライバシー**: 安定 pseudonym の禁止、SDC、disclosure class、公開 ID 分離
   （会計v3 L320-348）。v2 の閾値型 k-匿名性より厳密である。
5. **脅威モデルと多層検証**: threat actor/scenario/control 対応（会計v3 L482-490）と
   Schema/Semantic/Coverage の3層検証。

### 6.4 判断が変わった・要注意の点

| 論点 | v2 | v3 | 評価 |
| --- | --- | --- | --- |
| 寄付者の識別 | HMAC `donor_hash` で追跡（会計v2 L163-169） | 公開では同一寄付者を安定リンク可能な ID を原則禁止、HMAC 値も不可、`donor_hash_sample` 非公開（会計v3 L328-330, L543） | プライバシー強化。ただし寄付者追跡という当初目的とのトレードオフが明示されていない |
| 承認者の識別 | `approved_by` に個人識別子必須（会計v2 L190-192） | 公開 Approval では内部 employee ID 等を禁止、role/org/time 等の最小属性（会計v3 L334, L544） | 公開面と内部監査を分離。B-15 内部監査履歴（L621）で補完されるが、説明責任論の再整理が必要 |
| k-匿名性 | `donor_count >= 5` で確保（会計v2 L170-175） | 単一閾値では不十分、SDC の複数手法（会計v3 L342-343） | 妥当な強化 |
| JSON-LD 永続性 | 数十年の解決性・恒久リダイレクト（会計v2 L80-90） | immutable versioned context（会計v3 L548） | 後退。永続識別子の要件が薄い |
| 改ざん検知の選択肢 | RFC 3161 / OpenTimestamps（会計v2 L311-312） | RFC 3161 を MTI、Checkpoint backend は未決定（会計v3 L423-441, L450-451, OI-01） | 実装は具体化したが、アンカー選択肢が未決 |
| 実装導線 | OSS コンバーター/CLI の整備（会計v2 L396-402） | Informative へ縮小（会計v3 L530-533） | 後退（採用障壁）。Tier 1 の CSVW 移行には変換ツールの規範がない |
| 具体例 | JSON インスタンス・ディレクトリ・API 例（会計v2 L200-279, L341-355） | 抽象化（会計v3 L295-318, L455-464） | Informative Annex として復元すべき |
| 理念・普及 | 基本理念（会計v2 L452-460） | Scope/Non-goals に置換（会計v3 L74-98） | 理念が薄い。調達・費用対効果は教育レポートの第13章（教育 L797-837）が補完できる |

### 6.5 v3 の未達・リスク

- Requirements Closure の PASS は「生成ソースの検証」に限定され、外部相互運用・独立実装・
  Reference Validator は未実施である（会計v3 L48-49, L661、原本 validation report）。
- Trust anchor と Checkpoint の MTI backend は未決定である（会計v3 L421, L451、OI-01/OI-02）。
- 正本パッケージがリポジトリ未収載で、本文の「同梱 `spec/`」表記と実構成
  （`registries/` / `requirements/`）が一致していない（会計v3 L21 と L51-60）。
- Open Issues が12件ある（会計v3 L513-528）。成熟度モデル上は Requirements Draft 段階で、
  Candidate Specification の前提（test coverage、Reference Validator、test vectors、独立実装、
  Security/Privacy review）は未達である（会計v3 L64-72, L510-511）。

### 6.6 v2→v3 評価サマリ

- 総合評価: 「提案 → 要件仕様」への転換は成功している。v3 は実装・検証の対象として明確で、
  監査可能性は大幅に向上した。
- 課題: v3 単体では v2 の実務導線（具体例・変換ツール・普及理念）を失っており、v3 だけでは
  導入できない。v2 の Informative 要素を v3 の付録として戻すのが妥当である。
- 推奨: v3 に「Informative Annex A: v2 由来の実装例（JSON-LD instance・directory structure・
  API endpoints・converter workflow）」と「Annex B: 普及・調達」を追加する。あわせて
  JSON-LD context の永続性要件を復活させ、第三者タイムスタンプの選択肢を再整理する。

## 7. 推奨アクション（優先度順）

### 高

1. 3レポート共通の「識別子・digest・manifest・バージョニング」最小プロファイルを
   1本の短い文書にまとめる。
2. 署名方式を content digest 層（JCS/SHA-256）と release signature 層（OpenPGP/JWS 両対応）に
   分離し、相互運用表を作る。
3. 機械可読ソース（会計パッケージ、放送パッケージ、教育のサンプルツリー）を同一の
   バージョン管理下に置く。少なくとも会計v3 と放送の ZIP は `reports/` 配下または
   別リポジトリで版管理する。

### 中

1. 用語集と成熟度・状態モデルの対応表（SemVer ↔ WD/Phase ↔ OKF status ↔ Issue state）
   を作る。
2. 教育パイプラインに構造・意味検証（メタデータ schema、ID 重複、学習指導要領コード被覆、
   `license` 必須）を追加する。
3. 会計v3 に v2 由来の Informative Annex（例・ディレクトリ・API・コンバーター）を復元する。
4. 放送の Release に RFC 3161 または透明性ログを追加検討する（会計v3・v2 の第三者性原則と
   整合させる）。

### 低

1. 教材メタデータの `license` を SPDX 識別子化し、disclosure class と rights statement を付与する。
2. 3レポート自身に Cross-review Policy を適用し、reviewer metadata を記録する
   （本レビューも対象）。

## 8. 検証と限界

- 方法: 4文書（3レポート + 会計v2）の全文精読、原本パッケージ ZIP の構成確認、同梱の
  validation report / manifest の確認、行単位の引用照合。
- 引用行番号は 2026-11-09 時点の各ファイル先頭からの通し行番号である。
- 限界:
  - 単一モデル・単一セッションによる机上レビューであり、独立検証ではない。
  - 機械可読パッケージの全 YAML/JSON を再検証していない。会計の PASS は build-time の
    記録を引用し、外部相互運用未実施は原資料の記載どおりである。
  - 放送の Baseline 仕様（`broadcast-news-transparency-poc-spec.md`）の全文評価と、
    会計v2 全章の逐条評価は行っていない。
  - 3レポートはいずれも `status: draft` であり、本レビューもドラフト文書に対する
    ドラフト評価である。

## 参照

- 会計v3: [公共・非営利団体会計情報公開標準 v3（Requirements Draft WD 0.3）](2026-11-09-public-nonprofit-accounting-standard-v3-wd03.md)
- 放送: [放送ニュース透明性制度 現段階フル版](2026-11-09-broadcast-news-transparency-governance.md)
- 教育: [オープン教材・Markdown・GitHub・AI家庭教師による教育DX構想](2026-11-09-education-dx-open-textbook-ai-tutor.md)
- 会計v2: `public_nonprofit_accounting_standard-1.md`（2026-09-20、SHA-256:
  `d29097248e2eb4c9db01f2325e2e6fdd2e56095c1b642793ce3daf5c76a8730b`、リポジトリ外）
- 原本パッケージ: `public_nonprofit_accounting_v3_wd03_artifacts.zip`（SHA-256:
  `025ff9f6bdb6541ad94e054e344ef06633b5006fda44dd3eeeb2a2cba05397a6`）、
  `broadcast-transparency-governance-v0.1.0-draft-package.zip`（SHA-256:
  `aea69054fd24d94057b26807181f06b9e869fca49474401ab7f3f52d9c8e9906`）
- 会計 validation report: `generated/validation-report.json`（PASS、16 Blocker / 89
  Requirements、外部相互運用性テスト未実施）
