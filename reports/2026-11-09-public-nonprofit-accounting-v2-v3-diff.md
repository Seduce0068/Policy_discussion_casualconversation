---
type: Report
title: "会計情報公開標準 v2→v3（WD 0.3）差分評価"
description: "公共・非営利団体会計情報公開標準の旧案 v2 から v3 WD 0.3 への変更を、章対応・新たな規範化・判断変更・後退点の観点で評価した差分評価レポート"
tags:
  - accounting
  - standardization
  - requirements-engineering
  - privacy
  - provenance
generated:
  by: opencode
  at: "2026-11-09T22:20:00+09:00"
status: draft
---

## 1. 目的と範囲

本レポートは、公共・非営利団体会計情報公開標準の旧案 v2（2026-09-20）と、現行の
[v3 WD 0.3](2026-11-09-public-nonprofit-accounting-standard-v3-wd03.md) の差分を要件レベルで
評価する。v3 の内部整合性レビューではなく、同一テーマにおける版間の変化（強化・判断変更・
後退・未達）の整理を目的とする。

| 略称 | 文書 | 位置づけ |
| --- | --- | --- |
| v2 | `public_nonprofit_accounting_standard-1.md`（リポジトリ外、460行） | 実務寄りの標準「案」。JSON 例・ディレクトリ構成・API 例を含む |
| v3 | [公共・非営利団体会計情報公開標準 v3（Requirements Draft WD 0.3）](2026-11-09-public-nonprofit-accounting-standard-v3-wd03.md)（662行） | 16 Blocker・89 Requirements を伴う要件仕様 |

v2 はリポジトリ外（userscript リポジトリ）の資料である。本評価は SHA-256
`d29097248e2eb4c9db01f2325e2e6fdd2e56095c1b642793ce3daf5c76a8730b` の内容を参照した。
行番号は各ファイル先頭からの通し（2026-11-09 時点）。

## 2. 位置づけの変化

v2 は 460行・14章の標準「案」で、JSON インスタンス例、ディレクトリ構成、API 例、Tier 制、
運用ツール要件を含む実務寄りの提案だった（v2 L14-460）。v3 は 662行・24章で、16 Blocker・
89 Requirements・5 Registry・JSON Schema 6点・Validation Pipeline を伴う「要件仕様」へ
移行した（v3 L51-62, L162-195, L535-629）。全要件は BCP 14 の Atomic Requirement として
管理される（v3 L100-115, L227-269）。

評価としては、v2 の「方針と例」を v3 が「検証可能な義務」へ変換した質的転換であり、
方向は妥当である。

## 3. 章対応

| v2 | v3 | 判定 |
| --- | --- | --- |
| §0 改訂履歴（v2 L3-12） | §0 文書状態・成熟度（v3 L27-73） | 移行 |
| §1 基本方針（v2 L14-37） | §1 Scope/Non-goals（v3 L74-98）、§22 PDF（L530-533） | 維持 |
| §2 適用レベル（v2 L39-48） | §4 Conformance / Tier（v3 L135-161） | 強化 |
| §3 標準スタック（v2 L50-106） | §6/§9/§18、Tier 定義（v3 L156-158） | 分割・強化 |
| §3.2.1 JSON-LD 永続性（v2 L80-90） | B-02 context-versioning（v3 L548） | 一部後退 |
| §4 データモデル・プライバシー（v2 L108-178） | §10 Privacy、§11 Accounting（v3 L320-375） | 強化（判断変更あり） |
| §5 承認ログ・外部監査（v2 L180-198） | §10.3、AuditArtifact、B-10（v3 L148, L187, L332-334） | 判断変更 |
| §6 正本データ例（v2 L200-251） | §7.1 要件例（v3 L229-246） | 置換（例が要件に） |
| §7 Web 正本・構成（v2 L253-279） | §9 DCAT、§15 Checkpoint（v3 L295-318, L443-453） | 抽象化 |
| §8 変更履歴（v2 L281-299） | §7.3 / §9 / §17（v3 L263-265, L306-315, L466-470） | 強化 |
| §9 改ざん検知（v2 L301-339） | §12-14、B-05〜B-07（v3 L377-441） | 大幅強化 |
| §10 API（v2 L341-363） | §16 API / DataService（v3 L455-464） | 強化 |
| §11 PDF 位置付け（v2 L365-388） | §22 Informative（v3 L530-533） | 維持（縮小） |
| §12 実装ガイド（v2 L390-418） | §22（v3 L530-533） | 縮小（Informative） |
| §13-14 最終目標・理念（v2 L420-460） | §1、冒頭（v3 L24-25, L74-98） | 弱体化 |

## 4. 強化された点

1. **要件管理の規律**: BCP 14 authoring subset、Atomic Requirement、Requirement ID
   lifecycle、Registries、Closure Gate 13条件（v3 L100-115, L227-269, L492-511）。
   v2 には検証アーキテクチャが存在しなかった。
2. **暗号・完全性**: ES256/JWS/JCS/RFC 3161/trust anchor/Checkpoint/長期検証
   （v3 L377-453）。v2 の「ハッシュ + 署名」から、鍵発見・失効・長期検証まで規定した。
3. **同一性・版管理**: DCAT 3 の DatasetSeries/Logical/Immutable Version、previousVersion、
   correction/redaction/retention（v3 L295-318, L466-470）。
4. **プライバシー**: 安定 pseudonym の禁止、SDC、disclosure class、公開 ID 分離
   （v3 L320-348）。v2 の閾値型 k-匿名性より厳密である。
5. **脅威モデルと多層検証**: threat actor/scenario/control 対応（v3 L482-490）と
   Schema/Semantic/Coverage の3層検証。

## 5. 判断が変わった・要注意の点

| 論点 | v2 | v3 | 評価 |
| --- | --- | --- | --- |
| 寄付者の識別 | HMAC `donor_hash` で追跡（v2 L163-169） | 公開では同一寄付者を安定リンク可能な ID を原則禁止、HMAC 値も不可、`donor_hash_sample` 非公開（v3 L328-330, L543） | プライバシー強化。ただし寄付者追跡という当初目的とのトレードオフが明示されていない |
| 承認者の識別 | `approved_by` に個人識別子必須（v2 L190-192） | 公開 Approval では内部 employee ID 等を禁止、role/org/time 等の最小属性（v3 L334, L544） | 公開面と内部監査を分離。B-15 内部監査履歴（L621）で補完されるが、説明責任論の再整理が必要 |
| k-匿名性 | `donor_count >= 5` で確保（v2 L170-175） | 単一閾値では不十分、SDC の複数手法（v3 L342-343） | 妥当な強化 |
| JSON-LD 永続性 | 数十年の解決性・恒久リダイレクト（v2 L80-90） | immutable versioned context（v3 L548） | 後退。永続識別子の要件が薄い |
| 改ざん検知の選択肢 | RFC 3161 / OpenTimestamps（v2 L311-312） | RFC 3161 を MTI、Checkpoint backend は未決定（v3 L423-441, L450-451, OI-01） | 実装は具体化したが、アンカー選択肢が未決 |
| 実装導線 | OSS コンバーター/CLI の整備（v2 L396-402） | Informative へ縮小（v3 L530-533） | 後退（採用障壁）。Tier 1 の CSVW 移行には変換ツールの規範がない |
| 具体例 | JSON インスタンス・ディレクトリ・API 例（v2 L200-279, L341-355） | 抽象化（v3 L295-318, L455-464） | Informative Annex として復元すべき |
| 理念・普及 | 基本理念（v2 L452-460） | Scope/Non-goals に置換（v3 L74-98） | 理念が薄い。調達・費用対効果の議論が欠けている |

## 6. v3 の未達・リスク

- Requirements Closure の PASS は「生成ソースの検証」に限定され、外部相互運用・独立実装・
  Reference Validator は未実施である（v3 L48-49, L661、原本 validation report）。
- Trust anchor と Checkpoint の MTI backend は未決定である（v3 L421, L451、OI-01/OI-02）。
- 本文の「同梱 `spec/`」表記と、実際のパッケージ構成（`registries/` / `requirements/`）が
  一致していない（v3 L21 と L51-60）。
- Open Issues が12件ある（v3 L513-528）。成熟度モデル上は Requirements Draft 段階で、
  Candidate Specification の前提（test coverage、Reference Validator、test vectors、独立実装、
  Security/Privacy review）は未達である（v3 L64-72, L510-511）。

## 7. 評価サマリと推奨

- 総合評価: 「提案 → 要件仕様」への転換は成功している。v3 は実装・検証の対象として明確で、
  監査可能性は大幅に向上した。
- 課題: v3 単体では v2 の実務導線（具体例・変換ツール・普及理念）を失っており、v3 だけでは
  導入できない。v2 の Informative 要素を v3 の付録として戻すのが妥当である。
- 推奨:
  1. v3 に「Informative Annex A: v2 由来の実装例（JSON-LD instance・directory structure・
     API endpoints・converter workflow）」を追加する。
  2. 「Annex B: 普及・調達」を追加する（教育DXレポートの調達・TCO の議論が参考になる）。
  3. JSON-LD `@context` の永続性要件を復活させる。
  4. 第三者タイムスタンプの選択肢（RFC 3161 以外のアンカー方式）を再整理する。

## 8. 検証と限界

- 方法: v2（460行）と v3（662行）の全文精読、原本パッケージ ZIP の構成確認、同梱の
  validation report / manifest の確認、行単位の引用照合。
- 限界:
  - 単一モデル・単一セッションによる机上レビューであり、v2 全章の逐条評価ではない。
  - 機械可読パッケージの全 YAML/JSON を再検証していない。v3 の PASS は build-time の記録を
    引用し、外部相互運用未実施は原資料の記載どおりである。
  - v2 はリポジトリ外の資料であり、本評価は上記 SHA-256 の内容に基づく。

## 参照

- v3: [公共・非営利団体会計情報公開標準 v3（Requirements Draft WD 0.3）](2026-11-09-public-nonprofit-accounting-standard-v3-wd03.md)
- v2: `public_nonprofit_accounting_standard-1.md`（2026-09-20、SHA-256:
  `d29097248e2eb4c9db01f2325e2e6fdd2e56095c1b642793ce3daf5c76a8730b`、リポジトリ外）
- 原本パッケージ: `public_nonprofit_accounting_v3_wd03_artifacts.zip`（SHA-256:
  `025ff9f6bdb6541ad94e054e344ef06633b5006fda44dd3eeeb2a2cba05397a6`）
- 会計 validation report: `generated/validation-report.json`（PASS、16 Blocker / 89
  Requirements、外部相互運用性テスト未実施）
