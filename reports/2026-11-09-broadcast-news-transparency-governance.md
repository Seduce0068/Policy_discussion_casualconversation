---
type: Report
title: "放送ニュース透明性制度 現段階フル版"
description: "放送報道の情報種別透明化制度案を検証可能なPoCへ移行するためのGovernance・Manifest・Trust・Cross-review設計の統合ドラフト"
tags:
  - media-policy
  - transparency
  - governance
  - provenance
  - reproducibility
generated:
  by: "GPT-5.6 Sol"
  at: "2026-11-09T12:49:00+09:00"
status: draft
---

## 1. 現在地

この文書は、放送報道における情報種別の透明化制度案を、検証可能なPoCへ移行するためのGovernance設計を統合したドラフトである。法制化案、完成済み研究Protocol、完成済みValidatorを意味しない。

**Current status:** `implementation-planned`。Schema・Policyの初期設計を整理した段階であり、Git署名、Repository Enforcement、Semantic
Validator、3モデルCross-reviewは未実施。

## 2. 制度・PoCの基礎モデル

PoC仕様書の5分類は次のとおり。

| Code | 表示 | 意味 |
| --- | --- | --- |
| `VERIFIED` | 確認済み | 放送局が所定の確認手続・根拠で確認済みとして扱う情報 |
| `CLAIM` | 発言・主張 | 第三者が述べた内容。発言事実と命題内容を分離 |
| `ANALYSIS` | 分析 | 情報・データに基づく推論、比較、解釈、予測 |
| `OPINION` | 論評 | 価値判断、支持・反対、規範的評価 |
| `UNVERIFIED` | 未確認 | 公共的報道価値があるが確認未了の情報 |

`VERIFIED`は国家認定の真実を意味せず、特定放送局・特定時点の確認状態を示す。`CLAIM`には`verification_status`を持たせる。文中に複数種別が混在する場合はspan単位分類を検討する。映像再現、生成メディア、
重要編集、文脈変更、過去映像等は5分類と別のmetadataとして扱う。

## 3. 文書体系

```text
manifest.yaml
├── baseline/ (immutable historical snapshot)
├── active-poc-spec.md
├── study-protocol.md
├── reference-standard-protocol.md
├── sap.md
└── governance.md
```

各文書の正本はAuthority Matrixで管理する。Study Protocol、Reference Standard Protocol、SAPは未作成。Baselineに含まれる旧実験案を現行の研究設計として扱わない。

## 4. Authority Matrix

| Scope | Authoritative document | Conflict action |
| --- | --- | --- |
| 5分類の意味 | Active PoC Spec | block-release / issue / dependent update |
| UI・非言語表示 | Active PoC Spec | 同上 |
| JSON Schema・データモデル | Active PoC Spec | 同上 |
| Research Questions・Population | Study Protocol | 同上 |
| Assignment・Estimand | Study Protocol | 同上 |
| Reference Classification・Adjudication | Reference Standard Protocol | 同上 |
| Estimator・CI | SAP | 同上 |
| Missing Data・Multiplicity | SAP | 同上 |
| Ethics・Rights・AI logs | Governance | 同上 |
| Manifest・Digest・Dependencies | Manifest | 同上 |

正本の所有者は明確にするが、矛盾発生時に片方を自動採用しない。未解決の矛盾はReleaseをblockする。

## 5. Baseline Snapshot

| Item | Value |
| --- | --- |
| File | `broadcast-news-transparency-poc-spec.md` |
| Historical version | `0.1.0-snapshot` |
| SHA-256 | `0a2afab2c22face731c1ce55d6c075e864808e24212e9c728845c548c0167a59` |
| Size | 24,346 bytes |
| Lines | 872 |
| Encoding / newline | UTF-8 / LF |
| BOM | none |
| Package copy | included and rehashed |

このdigestは以前の実ファイル検証で確認されたもの。本Packageに同梱した場合は同梱ファイルも再hashしている。利用時には再度hashを確認する。Snapshotは変更しない履歴記録であり、Active
Specの現行規範を自動的に意味しない。

## 6. Manifest・Digest Policy

- Manifest SchemaとManifest Instanceを分離する。
- SchemaはJSON Schema Draft 2020-12を指定する。
- DigestはSHA-256、raw bytes、canonicalizationなし。
- Encoding、LF、BOM、size、line countは診断情報。
- Manifest自身のdigestはManifest内に埋め込まず、sidecarとsigned release recordで記録。
- Bundle digestは相対pathと各file digestを辞書順で連結した一覧から算出。
- Frozen dependencyではversionとdigestをpinし、`latest`を禁止。

## 7. Versioning

文書にSemVerを適用する場合の意味を次のように定義する。

- MAJOR: normative meaningの非互換変更。
- MINOR: 後方互換なnormative requirement追加。
- PATCH: 意味を変えない誤字、参照、説明、metadata修正。
- prerelease: alpha / beta / rc。

初期Draftは`0.x.y`。`1.0.0`は安定した外部向けnormative contractを承認した場合のみ付与する。日付revision方式を選ぶ場合はSemVerと混在させない。

## 8. Signing / Trust Policy

署名は物理的な改ざん防止ではなく、署名済みReleaseの発行元と対象snapshotを検証可能にするcontrolである。初期backendはOpenPGPに限定する。

Release検証では以下を別々に検査する。

1. Cryptographic signature validity。
2. Signer fingerprintがproject allowlistに存在。
3. Keyがrevokedでない。
4. Key expiry / validity policyに適合。
5. Git verifier configurationに適合。
6. Signed tag objectとcommitがManifest記載値に一致。

OpenPGPのWeb of Trust / local ownertrustはProject固有のSigner Authorizationを代替しない。秘密鍵はManifest・repositoryへ保存しない。fingerprint、
公開鍵参照、validity、revocation statusのみ記録する。Key rotation / revocation distributionは未設定。

## 9. Repository Enforcement

Policy宣言とHosting Provider上のEnforcementは別管理する。必要要件はRelease tag update/deleteの禁止、Protected Branch、bypass actorの最小化、Admin
bypassの監査。Release時点のRuleset snapshot、Rule ID、bypass actors、取得日時、config digestをEvidenceとして保存する。Providerと実設定は未確定。

## 10. Cross-review Policy

必須Reviewer Slotは`qwen-family`、`claude-family`、`chatgpt`。各実行にprovider、product surface、model label、provider model IDの公開可否、
web、memory、connected sources、tools、開始・終了時刻、target bundle digest、brief digest、output digestを記録する。

**同じReview BriefはProvider contextを同一化しない。** 各サービス固有のsystem instruction、safety policy、memory、tool runtime等は揃えられないため、
`provider_context_equivalent: false`とする。

初回レビューは`cross-exposure-isolated first pass`と定義する。他モデルのレビュー結果や統合要約を見せない手続であり、統計的独立性を主張しない。

- Pass A: Artifact-only structural review。内部整合、Schema、版、依存、ロジック。外部検索は無効化可能な場合のみ無効化し、無効化できないときは`not-configurable`と記録。
- Pass B: Evidence verification。公式一次資料、標準、再現可能テスト、研究文献等を確認。
- 多数決は禁止。統合はEvidence-based Adjudication。
- AIはresolutionを提案できるが、最終決定しない。

## 11. Evidence Policy

固定的なsource rankではなくClaim Typeへの適合性を優先する。

- Artifact state: 実ファイル、digest、Schema / Semantic validation。
- Software behavior: 公式仕様 + 再現可能テスト。
- Legal / policy: 公式一次資料・現行条文。
- Empirical effect: 直接実験、systematic review、査読研究。
- Interpretation: 専門資料と明示された推論。

Evidenceには取得日時、URL、発行主体、文書version、可能ならcontent digestを記録する。著作権・機密上許されない全文複製は行わない。

## 12. Issue State Model

```yaml
issue:
  id: "GOV-EXAMPLE-001"
  severity: "high"       # critical | high | medium | low
  blocking: true          # severityと独立
  disposition:
    value: "accepted"    # accepted | partially-accepted | rejected | deferred | superseded
  implementation:
    status: "planned"    # not-started | planned | in-progress | implemented | not-applicable
  verification:
    status: "not-verified" # not-verified | verified | verification-failed
    target_sha256: null
    evidence: []
```

`accepted != implemented != verified`。Verificationには対象digestとEvidenceが必須。Blocking IssueはSeverityに関係なくReleaseを止める。
Blocking Issueは独立Verifierによる検証を要求する。

## 13. Cross-review Exit Criteria

- unresolved blocking issues = 0。
- required reviewer slots completed、または有効なWaiver。
- Integration completed。
- Schema validation pass。
- Semantic validation pass。
- Bundle / Review Brief digest一致。
- Signature verification pass。
- Authorized signer pass。
- Repository Enforcement Evidence verified。
- Project Owner approval。

Severity件数のみでReleaseを決めない。正式Release / PreregistrationでのWaiverは初期設定では不可。Silent substitutionは禁止。

## 14. Semantic Validator

JSON Schemaは構造を検査する。Semantic Validatorは以下を別途検証する。

- Frozen artifactに`latest`依存がない。
- Frozen dependencyにversion + digest pinがある。
- Verified Issueにtarget digestとEvidenceがある。
- 全Common Review ExecutionのBundle digestが一致。
- 全Common First PassのReview Brief digestが一致。
- Integrationは必須First Pass完了後に作成。
- Release signer fingerprintがauthorizedかつ未失効。
- Signed tag / commitがManifestと一致。
- Repository Enforcement EvidenceがRelease時点の設定を表す。
- `generated_at`が許容clock skewを超えて未来でない。
- Authority Conflictが未解決でない。
- Reviewer別Execution Profileがある。
- Release failure modeが`fail-closed`。

Validator実装は未了。

## 15. Timestamp Provenance

各生成物は`generated_at`に加え、時刻source（system-clock / host-generated / user-provided / model-asserted）、timezone、ingestion time、
allowed future skewを記録する。モデルが時計を読めない場合、推測で埋めない。

## 16. Data Governance

分類例: `public`、`internal`、`confidential`、`source-protected`。Raw prompt / responseの保存と公開を分離。匿名取材源、個人情報、著作権素材はアクセス制御下で管理。
暗号化、retention、削除、公開範囲をGovernanceで定義。秘密鍵やAPI credentialはartifactへ含めない。

## 17. OSF / Gitの役割

Git repositoryはActive Workspace、変更履歴、signed releaseを担う。OSF RegistrationはFrozen Protocol / Preregistration
milestoneとして利用する。OSF Registrationが署名者Identityを保証するわけではない。OSFの現行移行状態は実際に登録する時点で公式情報を再確認する。

## 18. 現在の実装状況

| Component | Status |
| --- | --- |
| Baseline digest record | 過去検証済み |
| Manifest / Schema | draft |
| Authority Matrix | draft |
| Release / Trust Policy | draft; key未設定 |
| Repository Enforcement | not configured / not verified |
| Cross-review Brief | draft 0.1.0 |
| Reviewer-specific Execution Profiles | schema draft |
| Semantic Validator | specification only; code未実装 |
| Qwen / Claude / ChatGPT first pass | 未実施 |
| Study Protocol / Reference Standard Protocol / SAP | 未作成 |
| Ethics / Rights Review | 未実施 |

## 19. 次工程

1. JSON SchemaとYAMLの構文検証。
2. Git repository初期化。
3. BaselineとGovernance Packageをcommit。
4. OpenPGP signer identity / key managementを設定。
5. Repository Enforcementと設定snapshotを取得。
6. Target BundleとReview Briefを固定。
7. Qwen・Claude・ChatGPTのPass A。
8. Pass BによるEvidence verification。
9. Evidence-based Integration。
10. Issue実装と独立検証。
11. Exit Criteria確認。
12. Active PoC Spec、Study Protocol、Reference Standard Protocol、SAP作成。

## 20. 未解決事項

- Signer identity、expiry、rotation、revocation distribution。
- Git hosting provider、bypass policy、Ruleset snapshot。
- Semantic Validator実装言語と実行環境。
- Common Review Briefの最終rubric。
- Reviewer model metadataの取得可能範囲。
- Integration Reviewerの具体的担当。
- Target Bundleの最初の構成とdigest。
- Study Protocolの対象母集団、割付、Estimand。
- Reference Standardの分類・Adjudication基準。
- Ethics / Rights / Data Management。
- SAPのSESOI、power、missingness、multiplicity。

## 21. 適用上の注意

本書は現段階の統合ドラフトであり、実装完了・検証完了・研究Protocol凍結・法制化を意味しない。未実装箇所は`planned` / `not-verified`のまま保持する。

## 付録: Package contents

Package ZIPにはREADME、Governance Policy、Manifest、Authority Matrix、Release Policy、Cross-review Policy、Issue Model、Review
Brief、Semantic Validator specification、JSON Schema 3点、Templateを収録する。

## 参照

- 原本: `broadcast-transparency-governance-v0.1.0-draft-full.md`（GPT-5.6 Sol、2026-11-09 12:49 JST、版 0.1.0-draft、SHA-256:
  `3c227370c876010ae70d695c56f946da7637646b5a9d1e463dfa621c92cffd41`）
- 原本パッケージ: `broadcast-transparency-governance-v0.1.0-draft-package.zip`（SHA-256:
  `aea69054fd24d94057b26807181f06b9e869fca49474401ab7f3f52d9c8e9906`）
- パッケージ同梱物: `governance.md`、`baseline/broadcast-news-transparency-poc-spec.md`、`manifest.yaml`、`authority-matrix.yaml`、
  `schemas/`、`templates/` ほか計 19 ファイル
- 本リポジトリへの取り込みに際し、frontmatter・見出し階層・表記（表の整形、行長調整）のみをリポジトリ規約に合わせて調整した。本文の内容は変更していない。
