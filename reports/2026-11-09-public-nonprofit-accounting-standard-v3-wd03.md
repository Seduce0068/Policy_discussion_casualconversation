---
type: Report
title: "公共・非営利団体会計情報公開標準 v3（Requirements Draft WD 0.3）"
description: "v2レビューからWD 0.3メタ仕様確定までの設計判断を統合した公共・非営利団体会計情報公開標準のフルドラフト"
tags:
  - accounting
  - nonprofit
  - open-data
  - privacy
  - conformance
generated:
  by: "GPT-6"
  at: "2026-11-09T12:51:00+09:00"
status: draft
---

## Requirements Draft — Working Draft 0.3

**文書状態:** Working Draft / Requirements Closure Candidate  
**生成日:** 2026-11-09 (JST)  
**規範的正本:** 同梱 `spec/` のYAML Registry・Requirement Sources  
**対象:** 公共・非営利団体の会計情報の機械可読公開・交換・検証

本書は、v2レビュー、Qwen3.8とGPTのクロスレビュー、WD 0.1 / WD 0.2、WD 0.3メタ仕様レビューで採択された設計判断を統合した現段階のフルドラフトである。正式な標準化機関による承認文書ではない。
Requirements Closureの完了は、本書の宣言ではなく、同梱Validation Pipelineの実測結果によって判定する。

## 0. 文書状態と現段階

### 0.1 現段階

本プロジェクトは、設計レビューの反復からNormative Registries、Requirement Sources、JSON Schema、Semantic Linter、Coverage
Engineを実ファイル化する段階へ移行する。

| 領域 | 現在の状態 | 次の確認 |
| --- | --- | --- |
| Scope / Non-goals | 合意済み | 法域・団体profileごとの調整 |
| B-01〜B-16 | Registry案を統合 | closure categoryの機械coverage |
| Requirement Meta-model | Atomic方式を採用 | schema/lint実行 |
| Restricted YAML | 方針確定 | parser間互換試験 |
| DCAT Identity | モデル方針確定 | RDF/SHACL例の検証 |
| JWS / ES256 / JCS | プロファイル方針確定 | 実署名Test Vector |
| RFC 3161 | 対象byte列の方針確定 | TSA policy / certificate test |
| Checkpoint | 共通interface方針 | backend MTI決定 |
| CSVW / JSON Schema | 責務分担確定 | metadata/schema実装 |
| Accounting semantics | イベントと基本不変条件を定義 | 会計基準別mapping |
| API | snapshot/pagination/digest方針 | contract test |
| XBRL | xBRL-CSV MTI方針 | taxonomy/mapping実装 |
| Reference Validator | 未実装 | prototype |
| Independent interoperability | 未実施 | 独立実装間試験 |

### 0.2 Normative Source of Truth

機械可読な正本は以下とする。

- `registries/blockers.yaml`
- `registries/targets.yaml`
- `registries/verification-methods.yaml`
- `registries/references.yaml`
- `requirements/B-01.yaml` 〜 `B-16.yaml`
- `schemas/*.schema.json`

Markdown本文、表、Coverage Matrixは正本から生成するInformative Viewとし、手編集された表を規範的正本として扱わない。

### 0.3 成熟度モデル

本プロジェクトはW3C Recommendation Trackの考え方を参考にした独自成熟度モデルを採用する。W3C正式プロセスへの参加やW3C statusを意味しない。

| Phase | 成果物 | 完了条件 |
| --- | --- | --- |
| Requirements Draft | Scope、Registry、Atomic Requirements、Closure Gate | Blocker coverageとメタ仕様検証 |
| Candidate Specification | Schema、CSVW、JWS profile、Test Vector、Reference Validator | 実装・検証可能な仕様一式 |
| Stable Release | 独立実装、相互運用、Security/Privacy Review | 再現可能な適合性と移行方針 |

## 1. Scope and Non-goals

### 1.1 Scope

本仕様は、適用される法令・会計基準に基づき作成された会計情報を、機械可読で由来を追跡でき、完全性を検証可能な形で公開・交換するためのApplication Profileを定義する。

対象例:

- 公共団体の予算・決算・事業別支出情報
- NPO法人等の寄付・助成・事業費・管理費情報
- 公益法人・非営利団体の財務・活動報告
- 寄付、助成、委託、補助等の受入れ・配分情報
- 組織、事業、資金、勘定科目、支出、監査成果物の関連情報

### 1.2 Non-goals

- 会計上の認識・測定・表示ルールの新設または置換
- ERP、会計ソフト、内部DB schemaの統一
- 特定ソフトウェア、programming language、cloud providerの強制
- 特定Blockchain/Public Ledgerへの依存
- 公開の必要性がない個人情報・内部統制情報の全面公開
- 監査意見そのものの妥当性を自動判定すること
- 本仕様だけで法令適合性を保証すること

本仕様は公開・交換profileであり、会計処理基準自体を置換しない。

## 2. Normative Language

### 2.1 BCP 14

本文中の “MUST”, “MUST NOT”, “REQUIRED”, “SHALL”, “SHALL NOT”, “SHOULD”, “SHOULD NOT”, “RECOMMENDED”, “NOT RECOMMENDED”,
“MAY”, “OPTIONAL” は、全て大文字表記の場合に限りBCP 14 [RFC 2119] [RFC 8174] の意味で解釈する。

### 2.2 Authoring Subset

Authored Requirementで使用可能な規範語は `MUST`, `MUST NOT`, `SHOULD`, `SHOULD NOT`, `MAY` とする。`SHALL`、`REQUIRED`、`RECOMMENDED`、
`OPTIONAL`等はRequirement Sourceで使用しない。

### 2.3 Atomic Requirement

**1 Requirement ID = 1 Target = 1 independently testable normative obligation** とする。独立した義務を一つのstatementへ連結しない。
`level`はAuthored YAMLへ保存せず、statementから導出する。

## 3. Terminology

| 用語 | 定義 |
| --- | --- |
| DatasetSeries | 共通特性を持つDatasetの系列 |
| Logical Dataset | ある報告期間・年度の論理的Dataset |
| Immutable Dataset Version | 特定時点のDataset内容を固定したsnapshot |
| Distribution | CSV、JSON、xBRL-CSV、PDF等によるDatasetの配布表現 |
| Dataset Package | Manifest、data files、schema references、署名・timestamp evidenceを含む版成果物 |
| Public Distribution | 外部利用者に提供するDistribution |
| Internal Record | 個人情報・内部統制・業務記録等を含む非公開データ |
| Application Profile | 共通仕様を対象ドメインに適用する語彙、型、制約、mappingの集合 |
| Checkpoint | 現在のDataset headを示す外部検証可能な証拠 |
| Trust Anchor | 署名鍵・鍵集合の真正性検証に用いる事前信頼情報 |
| Atomic Requirement | 1つのTargetに対する1つの規範的義務 |
| Closure Category | Blockerの完了に必要な要件カテゴリ |
| MTI | Mandatory-to-Implement。適合実装が必ず実装すべき方式 |

## 4. Conformance Classes and Tiers

### 4.1 Conformance Targets

| Target | Kind | 主な責任 |
| --- | --- | --- |
| Specification | normative-document | 規範本文とprofile |
| Publisher | implementation | 作成、公開、署名、checkpoint、訂正 |
| DatasetPackage | artifact | immutable version一式 |
| PublicDistribution | artifact | 公開CSV/JSON/xBRL-CSV/PDF等 |
| Validator | implementation | 構造、意味、署名、timestamp、checkpoint検証 |
| DataService | implementation | API配信 |
| Consumer | implementation | 取得・解釈 |
| AuditArtifact | artifact | 監査対象version、期間、意見、証跡 |

適合宣言は対象・Tier・profile versionを明示する。「組織全体がv3準拠」のみでは適合対象が不明確なため不十分とする。

### 4.2 Tier Profiles

| Tier | 必須範囲 | 想定用途 |
| --- | --- | --- |
| Tier 1 | Public/Internal separation、CSVW、JSON Schema、manifest、checksum、rights | 基本的機械可読公開 |
| Tier 2 | Tier 1 + JSON-LD 1.1、DCAT 3、OpenAPI、PROV-O、version/history | API・相互参照・由来追跡 |
| Tier 3 | Tier 2 + OIM/xBRL-CSV MTI、Table Constraints、ES256 JWS、RFC 3161、Checkpoint、長期検証 | 高保証の監査可能な公開 |

Tierは累積方式とし、上位Tierは下位Tierを包含する。Privacy・法令適合の最低要件をTierによって弱めない。

## 5. Normative Registries

### 5.1 Registry一覧

| Registry | 正本 |
| --- | --- |
| Blocker Registry | `registries/blockers.yaml` |
| Target Registry | `registries/targets.yaml` |
| Verification Method Registry | `registries/verification-methods.yaml` |
| Normative Reference Registry | `registries/references.yaml` |
| Requirement Sources | `requirements/B-XX.yaml` |

### 5.2 Normative Blocker Registry

| ID | Blocker | Closure Categories |
| --- | --- | --- |
| B-01 | Public / Internal Schema Separation | schema-separation, public-projection, donor-linkage, actor-reference, field-classification |
| B-02 | Normative Application Profile | profile-artifacts, field-definition, context-versioning, mapping-coverage, conformance-constraints |
| B-03 | Dataset / Version / DCAT Identity Model | dataset-series, logical-dataset, immutable-version, version-chain, distribution-binding |
| B-04 | DCAT Responsibilities and Rights | resource-responsibilities, rights-statement, license-declaration, publisher-metadata, distribution-metadata, service-association |
| B-05 | Cryptographic Signing Profile | algorithm-suite, jws-serialization, manifest-canonicalization, detached-payload, protected-header, forbidden-header-extension, outer-artifact-canonicalization, signature-validation |
| B-06 | Key Discovery and Trust Bootstrap | trust-anchor, jwks-authentication, key-identifier, rotation, revocation, historical-key-validation, compromise-recovery |
| B-07 | Timestamp and Long-Term Validation Profile | timestamp-input, timestamp-token-publication, tsa-validation, long-term-validation |
| B-08 | Privacy and Security Threat Model | threat-actors, integrity-threats, privacy-threats, control-mapping |
| B-09 | CSVW / JSON Schema Responsibility | csv-validation, json-validation, rdf-validation, conformance-tests |
| B-10 | Audit Coverage and Version Reference | audit-target, audit-period, auditor-and-opinion, manifest-linkage |
| B-11 | Freshness / Checkpoint / Rollback Resistance | checkpoint-schema, checkpoint-publication, validator-check, state-semantics, rollback-detection, backend-interoperability |
| B-12 | Public Identifier and Information Leakage Policy | internal-id-separation, timestamp-leakage, enumeration, capability-separation, identifier-persistence |
| B-13 | Accounting Semantics and Reconciliation | event-taxonomy, correction-linkage, balance-invariant, amount-and-currency, time-interval, reconciliation |
| B-14 | API and DataService Profile | api-description, immutable-version-retrieval, snapshot-consistency, stable-pagination, etag-semantics, digest-semantics, error-semantics |
| B-15 | Correction, Redaction, and Retention | immutable-correction, public-redaction, internal-audit-history, retention, legal-hold, prepublication-review |
| B-16 | XBRL Serialization and Validation | oim-conformance, xbrl-csv-mti, table-constraints, taxonomy-mapping, alternate-serialization |

各Blockerは1つ以上のclosure categoryを持つ。`source_blocker`はprimary ownership、`related_blockers`はcross-cutting referenceを表す。

## 6. Restricted YAML Profile

### 6.1 Parser Contract

YAML 1.2.2を入力構文とし、以下を強制する。

**許可:** UTF-8、1 file 1 document、文字列keyのmapping、sequence、string、boolean、null、JCS/I-JSON数値制約内の有限JSON number。

**禁止:** duplicate keys、non-string mapping keys、anchors、aliases、explicit tags、TAG directives、merge-key extensions (`<<`)、
NaN/Infinity、parser固有object tags、複数document、暗黙的な日時native type変換。

日付・時刻・version・IDは文字列として明示する。重複keyはparser errorとし、last-write-winsで処理しない。

### 6.2 JCS Compatibility

Registry/Requirementでは数値を原則使用しない。数値を使う場合はRFC 8785 / I-JSONのIEEE 754 double precision要件に適合する。任意精度整数や精密値は文字列にする。
([RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html))

### 6.3 Canonical Build

```text
Restricted YAML
→ JSON-compatible model
→ Structural Validation
→ Semantic Validation
→ Coverage Evaluation
→ RFC 8785 JCS (JSON artifacts)
→ SHA-256 digests
```

## 7. Requirement Meta-model

### 7.1 Authored Record Example

```yaml
id: CONF-B05-001
source_blocker: B-05
related_blockers: [B-06]
closure_category: jws-serialization
target: Publisher
statement: >-
  The Publisher MUST generate signatures using Flattened JWS JSON Serialization.
verification: [cryptographic-test]
test_cases: []
normative_references: [JWS-RFC7515]
introduced_in: WD-0.3
status: active
```

`level`はAuthored Sourceに保存しない。statementからLinterが抽出する。

### 7.2 Field Rules

- `id`: 永続的Requirement ID
- `source_blocker`: primary Blocker
- `related_blockers`: optional cross-cutting links
- `closure_category`: source Blocker内のprimary category
- `target`: Registryに登録された単一Target
- `statement`: 1つのBCP 14 obligation
- `verification`: Registryに登録された1つ以上の検証方式
- `test_cases`: WD段階では空配列可
- `normative_references`: Reference Registry ID
- `introduced_in`: 初出WD版
- `status`: draft / active / deprecated / superseded / withdrawn
- `superseded_by`: superseded時の置換先ID

### 7.3 Lifecycle / ID

公開済みRequirement IDを別意味で再利用しない。superseded要件は`superseded_by`を持ち、参照先の存在とcycle不在をSemantic Linterで検証する。

### 7.4 Atomicity

1 ID = 1 Target = 1 independently testable obligation。許可するBCP 14 phraseを一つだけ含める。`MUST NOT`と`SHOULD NOT`は最長一致で解析する。

## 8. Validation Architecture

### Layer 1: Structural Validation

JSON Schema Draft 2020-12で型、必須属性、pattern、static enum、unknown field、array uniqueness等を検証する。各schemaは`$schema` URIでDraft
2020-12を固定する。([JSON Schema](https://json-schema.org/draft/2020-12))

### Layer 2: Semantic Validation (`closure-lint`)

Registry参照、category所属、Target/Verification/Reference参照、ID重複、BCP 14抽出、atomicity、lifecycle、supersession cycle等を検証する。

### Layer 3: Coverage Engine

Blockerは、Registryに登録された全closure categoryに、同一primary `source_blocker`とcategoryを持つactive Requirementが1件以上存在し、Layer
2を通過した場合のみClosedとする。`related_blockers`はprimary coverageの代替にしない。

### Severity

- `error`: Closure失敗、exit non-zero
- `warning`: Closureを直ちに失敗させないが報告
- `info`: 診断のみ

Broken reference、duplicate ID、uncovered category、BCP 14 parse failure、lifecycle cycleはerrorとする。

## 9. DCAT Identity Model

基本関係:

```text
Accounting DatasetSeries
├── FY2026 Logical Dataset
│   ├── FY2026 Immutable Version 1
│   ├── FY2026 Immutable Version 2
│   └── FY2026 Immutable Version 3 (current)
└── FY2027 Logical Dataset
```

- 年次報告群を `dcat:DatasetSeries` とする。
- 各年度Logical Datasetは `dcat:Dataset` であり、`dcat:inSeries` でSeriesへ関連付ける。
- Immutable Versionは別 `dcat:Dataset` resourceとし、Logical Datasetから `dcat:hasVersion` / `dcat:hasCurrentVersion` で参照する。
- version間は `dcat:previousVersion` で接続する。
- Distributionはimmutable versionへ付属させる。

DCAT 3は年度別DatasetとそのDatasetのrevision versionを別々に表現する例を持つ。`dcat:previousVersion`はversion lineageの意味論であり、rollback
freshness証明はB-11のCheckpointが担う。([DCAT 3](https://www.w3.org/TR/vocab-dcat-3/))

Logical DatasetとImmutable Versionはいずれも`dcat:Dataset`となるため、profile固有class（例：`acct:LogicalAccountingDataset`、
`acct:ImmutableAccountingDatasetVersion`）またはcontrolled vocabularyとSHACL制約で役割を機械判別する。

## 10. Privacy and Disclosure Control

### 10.1 Public/Internal

Internal RecordとPublic Distributionは別schema/DTOを使用する。内部recordをそのまま公開serializationしてはならない。

### 10.2 Donor

Public Distributionには、同じ寄付者を複数寄付record間で安定link可能にするpseudonymous identifierを原則含めない。
HMAC由来の値も同一鍵・方式でlinkabilityを持つため公開匿名IDとして扱わない。法域別の明示profileにより例外を設ける場合は、法的根拠とPrivacy Reviewを必要とする。
`donor_hash_sample`は公開schemaに含めない。

### 10.3 Approval

Public Approval Recordは内部employee ID、internal actor ref、内部DB keyを含めない。公開上必要な場合はrole、organization、approval time等の最小属性を使う。

### 10.4 Disclosure Classes

`public`、`internal_only`、`public_if_legal_basis`、`public_aggregated`を定義し、すべてのprofile fieldに分類を付与する。

### 10.5 Statistical Disclosure Control

minimum cell sizeだけで匿名性を保証しない。small-cell suppression、complementary suppression、differencing assessment、temporal
aggregation、generalisation/rounding、unique combination detectionを扱う。`donor_count >= 5`単独をk-anonymityと呼ばない。

### 10.6 Public Identifier

内部DB keyとPublic Persistent IDを分離する。UUIDv7はtimestampを上位48bitに含むため、生成時刻漏洩を評価する。時刻秘匿が必要ならUUIDv4等CSPRNG方式を使用する。
UUIDをauthorization secretとして使わない。([RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html))

## 11. Accounting Semantics and Reconciliation

### 11.1 Event Model

Profileは少なくともdonation、expense、refund、reversal、correction、transfer、chargeback、write-off、allocation release、fund
reclassificationを区別する。

### 11.2 Linkage

返金・取消・訂正・逆仕訳は元イベントまたは取引を参照する。公開済みimmutable versionのrecordを上書きしない。

### 11.3 Reconciliation

```text
Opening Balance
+ Inflows
- Outflows
+/- Adjustments
= Closing Balance
```

これは公開データ間の整合性確認であり、会計上の認識・測定ルールを新設するものではない。fund、account、currency、periodごとに検証し、配分超過・残高繰越不整合等はsector profileで拡張する。

### 11.4 Amount / Currency / Time

金額は正確なdecimal stringまたはprofile-defined minor-unit integer。通貨はISO 4217。date-onlyとtimestampを区別し、timestampはRFC 3339を基本とする。

## 12. Cryptographic Integrity Profile

### 12.1 Algorithm

初期MTIはJOSE `ES256`（ECDSA P-256 / SHA-256）。algorithm suite version、deprecation、migration window、旧署名の検証期間を定義する。RFC
9864はJOSEのES256をfully-specified algorithm identifierの例に挙げる。([RFC 9864](https://www.rfc-editor.org/rfc/rfc9864.html))

### 12.2 Manifest

Manifestはdataset ID、logical dataset ID、version ID、profile version、issued time、previous version digest、各fileのrelative
path/media type/byte length/SHA-256、schema/context version、signing key IDを含む。

### 12.3 Manifest Signing

Manifest data modelへRFC 8785 JCSを適用し、canonical UTF-8 bytesをJWS payloadとする。

### 12.4 JWS

Flattened JWS JSON SerializationをMTIとする。通常のJWSを生成した後、公開artifactから`payload` memberを削除する。Validatorは正確に同じpayloadを再構築し検証する。
([RFC 7515 Appendix F](https://www.rfc-editor.org/rfc/rfc7515.html))

### 12.5 Protected Header

- `alg`と`kid`はprotected headerに置く。
- `alg=none`を受け入れない。
- unprotected `header` memberを使用しない。
- RFC 7797の`b64` header parameterを使用しない。([RFC 7797](https://www.rfc-editor.org/rfc/rfc7797.html))

### 12.6 Outer Artifact Canonicalization

署名後のdetached Flattened JWS JSON objectにもRFC 8785 JCSを適用する。これはJWS signing inputを変えず、公開・timestamp対象のouter JSON byte
representationを一意化するためである。

### 12.7 Numeric / Unicode

JCS対象JSONはI-JSONに適合させる。JSON numberはIEEE 754 double precisionで表現可能でなければならず、高精度値や長整数はstring化する。Unicode string
dataはcanonicalization前後で変えない。([RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html))

## 13. Key Discovery and Trust Bootstrap

同じWeb hostからdata/JWS/JWKSを取得するだけでは、host侵害時の同時置換を防げない。独立trust anchorを定義し、JWKS authenticity、kid uniqueness、rotation、
revocation、compromise recovery、historical key validationを扱う。

候補: offline root key、third-party signed key statement、independent transparency log、pre-pinned trust anchor。
最終方式はCandidate Specificationで選択する。

## 14. Timestamp / Long-Term Validation

RFC 3161 `messageImprint`は、RFC 8785 canonicalized detached Flattened JWS JSON artifactのUTF-8 bytesに対するSHA-256とする。

```text
Manifest model
→ JCS
→ canonical manifest bytes
→ JWS ES256
→ detached Flattened JWS JSON
→ JCS
→ canonical outer JWS bytes
→ SHA-256
→ RFC 3161 messageImprint
```

Timestamp token、TSA certificate chain、policy identifier、必要なrevocation evidenceを保存する。長期検証ではalgorithm deprecation、
certificate status、evidence renewalを扱う。RFC 3161はtime-stamped datumのhashをmessageImprintに格納する。
([RFC 3161](https://www.rfc-editor.org/rfc/rfc3161.html))

## 15. Freshness / Checkpoint / Rollback

Checkpoint objectはdataset identity、head version、manifest digest、sequence、evidence time、signer/evidence reference、
previous checkpoint digest等を持つ。

Validator status: `fresh`, `stale`, `conflicting`, `invalid`, `unknown`。

Backend候補はTransparency Log、第三者署名Checkpoint、Public Anchor、Trusted Catalog、Verifier-side latest-seen checkpoint。
いずれも共通Evidence Interfaceへ対応させる。MTI backendは未決定。

Hash chainと`dcat:previousVersion`だけでは、初見Verifierへの最新状態証明にならない。独立checkpointまたはlatest-seen stateと組み合わせる。

## 16. API / DataService

API契約はOpenAPIで記述し、現段階の参照版は3.2.1（2026-09-10）とする。([OpenAPI 3.2.1](https://spec.openapis.org/oas/v3.2.1.html))

- immutable versionをURI等で直接取得可能にする。
- paginationは固定snapshotとdeterministic orderingを持つ。
- immutable representationにはstrong ETagを推奨する。
- Content-DigestはHTTP message content、Repr-Digestはselected representationのdigestを表す。
- HTTP digestはJWSによるPublisher認証やprovenanceの代替ではない。([RFC 9530](https://www.rfc-editor.org/rfc/rfc9530.html))
- invalid ID、version unavailable、pagination token invalid、authorization failure等のerror categoryを安定化する。

## 17. Correction / Redaction / Retention

訂正は新しいimmutable versionとして発行し、supersedes等で旧版を参照する。公開履歴は原則append-onlyだが、個人情報・保護情報の漏えい時にはredaction/tombstoneを許容する。
内部監査履歴は適用retention期間内でauditabilityを維持し、永久保存とはしない。適用legal hold中は対象recordの削除を停止する。外部timestamp/public anchor前にprivacy
disclosure reviewを行う。

## 18. XBRL OIM

Tier 3のMTI serializationはOIM conformant xBRL-CSVとする。xBRL-CSV Table Constraints 1.0はserializationではなくvalidation
extensionであり、適用profileの検証規則として扱う。
([XBRL OIM 1.0](https://specifications.xbrl.org/work-product-index-open-information-model-open-information-model.html),
[Table Constraints 1.0](https://www.xbrl.org/Specification/table-constraints/REC-2026-09-10/table-constraints-REC-2026-09-10.html))

Mapping artifactはsource account code/label、target concept URI、mapping version、effective period、status、transformation
ruleを保持する。xBRL-JSON / XBRL-XMLは追加表現として許容する。

## 19. Threat Model

Threat actor: malicious insider、compromised publication host、compromised signing key、unauthorized external party、
rollback attacker、linkage analyst。

Threat scenarios: data substitution、historical tampering、rollback、conflicting checkpoint、key substitution/compromise、
donor re-identification、differencing、accidental disclosure、redaction failure。

各脅威をpreventive/detective/corrective controlに対応付ける。暗号は完全性・署名主体・存在時刻の一部を支えるが、会計内容の真実性や権限の適法性を単独で保証しない。

## 20. Closure Gate

WD 0.3 Requirements Closureの判定条件:

- YAML Profile validation = PASS
- Registry Schema Validation = 100%
- Requirement Schema Validation = 100%
- Cross-Registry Referential Integrity = 100%
- Requirement Atomicity = 100%
- BCP 14 level extraction = 100%
- Blocker / Category Coverage = 100%
- Orphan active requirements = 0
- Duplicate IDs = 0
- Broken lifecycle references = 0
- Supersession cycles = 0
- Unresolved normative references = 0
- Generated artifacts reproducibility = PASS

Candidate Specification移行には、automatable active requirementのtest coverage、Reference Validator、cryptographic test vectors、
独立実装間試験、Security/Privacy review、migration guidanceを追加条件とする。

## 21. Open Issues

| ID | Issue | 現状 | 決定段階 |
| --- | --- | --- | --- |
| OI-01 | Checkpoint MTI backend | 共通interface。複数候補 | Candidate |
| OI-02 | Trust anchor bootstrap | offline root / signed key statement等 | Candidate |
| OI-03 | TSA policy / renewal | RFC 3161、chain、policy、renewal要求 | Candidate |
| OI-04 | Logical/Version RDF class | `acct:` classかcontrolled vocabulary | Candidate |
| OI-05 | Sector taxonomy ownership | 法域・団体profile別 | Sector profile |
| OI-06 | Legal donor publication exception | 原則stable donor IDを公開しない | Legal review |
| OI-07 | Retention period values | 法令・records policyに従う | Sector profile |
| OI-08 | API authentication / rate limit | deployment policy | API profile |
| OI-09 | CSV dialect / encoding | CSVWで表現しprofileでpin | Candidate |
| OI-10 | Table Constraints coverage | applicable constraintsを選択 | Candidate |
| OI-11 | OpenAPI feature set | 3.2.1を参照、subset固定 | Candidate |
| OI-12 | Algorithm migration | ES256 MTI、移行・旧版検証期間 | Candidate |

## 22. Implementation Guidance (Informative)

Rust等のmemory-safe languageは推奨候補だが、言語自体を規範要件にしない。SBOM、dependency lock、signed release、reproducible buildを実装品質管理として推奨する。
PDFは人間向け派生物とし、structured dataを機械利用の中心に置く。

## 23. Normative Requirements Index

この表はYAML正本から生成した一覧であり、Normative Sourceは`requirements/B-XX.yaml`である。

| ID | Blocker | Category | Target | Level | Requirement |
| --- | --- | --- | --- | --- | --- |
| SPEC-B-01-001 | B-01 | schema-separation | Specification | MUST | The specification MUST define separate internal and public data models for publication-relevant entities. |
| CONF-B-01-002 | B-01 | public-projection | Publisher | MUST | The Publisher MUST construct each Public Distribution from an explicit public projection rather than serializing internal records directly. |
| CONF-B-01-003 | B-01 | donor-linkage | PublicDistribution | MUST NOT | A Public Distribution MUST NOT contain a stable pseudonymous identifier that enables the same donor to be linked across donation records, except under an explicitly defined and legally reviewed jurisdiction-specific profile. |
| CONF-B-01-004 | B-01 | actor-reference | PublicDistribution | MUST NOT | A Public Approval Record MUST NOT expose an internal employee identifier, internal actor reference, or internal database key. |
| SPEC-B-01-005 | B-01 | field-classification | Specification | MUST | Every field in the normative Application Profile MUST declare a disclosure classification. |
| SPEC-B-02-001 | B-02 | profile-artifacts | Specification | MUST | The specification MUST provide a machine-readable Application Profile as a normative artifact. |
| SPEC-B-02-002 | B-02 | field-definition | Specification | MUST | Each Application Profile field MUST define its term identifier, datatype, cardinality, required tier, visibility, and privacy class. |
| CONF-B-02-003 | B-02 | context-versioning | Publisher | MUST | The Publisher MUST reference an immutable versioned JSON-LD context for every JSON-LD Distribution. |
| SPEC-B-02-004 | B-02 | mapping-coverage | Specification | MUST | The Application Profile MUST define mappings between JSON properties, CSVW columns, and applicable DCAT, PROV-O, and XBRL concepts. |
| CONF-B-02-005 | B-02 | conformance-constraints | Validator | MUST | The Validator MUST validate a Dataset Package against the exact Application Profile version declared by that package. |
| SPEC-B-03-001 | B-03 | dataset-series | Specification | MUST | The specification MUST model a continuing annual reporting collection as a dcat:DatasetSeries when the collection semantics apply. |
| CONF-B-03-002 | B-03 | logical-dataset | Publisher | MUST | The Publisher MUST represent each reporting period as a logical dcat:Dataset linked to its DatasetSeries by dcat:inSeries. |
| CONF-B-03-003 | B-03 | immutable-version | Publisher | MUST | The Publisher MUST publish each released immutable snapshot as a distinct version resource linked to its logical Dataset. |
| CONF-B-03-004 | B-03 | version-chain | Publisher | MUST | The Publisher MUST link successive immutable versions with dcat:previousVersion and expose the current version with dcat:hasCurrentVersion. |
| CONF-B-03-005 | B-03 | distribution-binding | Publisher | MUST | The Publisher MUST associate every released data Distribution with exactly one immutable Dataset Version. |
| SPEC-B-04-001 | B-04 | resource-responsibilities | Specification | MUST | The specification MUST define distinct responsibilities for dcat:Dataset, dcat:Distribution, and dcat:DataService. |
| CONF-B-04-002 | B-04 | rights-statement | PublicDistribution | MUST | Every Public Distribution MUST provide an explicit rights statement using dcterms:rights or a profile-defined equivalent. |
| CONF-B-04-003 | B-04 | license-declaration | PublicDistribution | MUST | A Public Distribution made available under a specific legal license MUST identify that license using dcterms:license. |
| CONF-B-04-004 | B-04 | publisher-metadata | Publisher | MUST | The Publisher MUST provide publisher, issued, and modified metadata for each logical Dataset and immutable Dataset Version. |
| CONF-B-04-005 | B-04 | distribution-metadata | PublicDistribution | MUST | Each Public Distribution MUST declare its media type and access or download location. |
| CONF-B-04-006 | B-04 | service-association | DataService | MUST | A DataService MUST identify the Dataset or Dataset Version that its API exposes. |
| SPEC-B-05-001 | B-05 | algorithm-suite | Specification | MUST | The initial Mandatory-to-Implement JWS signature algorithm MUST be ES256. |
| CONF-B-05-002 | B-05 | jws-serialization | Publisher | MUST | The Publisher MUST generate signatures using Flattened JWS JSON Serialization. |
| CONF-B-05-003 | B-05 | manifest-canonicalization | Publisher | MUST | The Publisher MUST canonicalize the manifest data model using RFC 8785 before constructing the JWS signing input. |
| CONF-B-05-004 | B-05 | detached-payload | Publisher | MUST | The Publisher MUST publish the Flattened JWS JSON Serialization with the payload member omitted after signing the exact canonical manifest bytes. |
| CONF-B-05-005 | B-05 | protected-header | Publisher | MUST | The Publisher MUST place the alg and kid parameters in the JWS protected header. |
| CONF-B-05-006 | B-05 | forbidden-header-extension | Publisher | MUST NOT | The Publisher MUST NOT include an unprotected header member in the profile-defined Flattened JWS artifact. |
| CONF-B-05-007 | B-05 | forbidden-header-extension | Publisher | MUST NOT | The Publisher MUST NOT include the b64 header parameter defined by RFC 7797 in a profile-conformant JWS. |
| CONF-B-05-008 | B-05 | outer-artifact-canonicalization | Publisher | MUST | The Publisher MUST canonicalize the detached Flattened JWS JSON object using RFC 8785 before publishing the timestamped artifact. |
| CONF-B-05-009 | B-05 | signature-validation | Validator | MUST | The Validator MUST reject a JWS whose alg parameter is none. |
| SPEC-B-06-001 | B-06 | trust-anchor | Specification | MUST | The specification MUST define a trust-anchor mechanism for authenticating a Publisher signing-key set independently of the content host. |
| CONF-B-06-002 | B-06 | jwks-authentication | Validator | MUST | The Validator MUST authenticate a retrieved JWK Set through the trust-anchor mechanism defined by the profile. |
| CONF-B-06-003 | B-06 | key-identifier | Publisher | MUST | The Publisher MUST ensure that each kid uniquely identifies one key within the applicable Publisher key-set version. |
| CONF-B-06-004 | B-06 | rotation | Publisher | MUST | The Publisher MUST publish authenticated key-rotation evidence linking each replacement key to the accepted trust chain. |
| CONF-B-06-005 | B-06 | revocation | Publisher | MUST | The Publisher MUST publish authenticated revocation evidence when a signing key is revoked or declared compromised. |
| CONF-B-06-006 | B-06 | historical-key-validation | Validator | MUST | The Validator MUST evaluate a historical signature against the key validity and revocation state applicable to the signed artifact time. |
| SPEC-B-06-007 | B-06 | compromise-recovery | Specification | MUST | The specification MUST define a recovery procedure for signing-key compromise. |
| SPEC-B-07-001 | B-07 | timestamp-input | Specification | MUST | The RFC 3161 messageImprint MUST be SHA-256 over the UTF-8 bytes of the RFC 8785 canonicalized detached Flattened JWS JSON artifact. |
| CONF-B-07-002 | B-07 | timestamp-token-publication | Publisher | MUST | The Publisher MUST publish the RFC 3161 timestamp token associated with each Tier 3 signed Dataset Package. |
| CONF-B-07-003 | B-07 | tsa-validation | Validator | MUST | The Validator MUST verify the timestamp token messageImprint, TSA signature, certificate path, and applicable TSA policy. |
| SPEC-B-07-004 | B-07 | long-term-validation | Specification | MUST | The specification MUST define evidence retention and renewal rules sufficient to support long-term timestamp validation. |
| SPEC-B-08-001 | B-08 | threat-actors | Specification | MUST | The threat model MUST include malicious insiders, compromised publication hosts, compromised signing keys, and unauthorized external parties. |
| SPEC-B-08-002 | B-08 | integrity-threats | Specification | MUST | The threat model MUST analyze data substitution, historical tampering, rollback, and conflicting checkpoint attacks. |
| SPEC-B-08-003 | B-08 | privacy-threats | Specification | MUST | The threat model MUST analyze donor linkage, re-identification, small-cell disclosure, differencing, and accidental disclosure. |
| SPEC-B-08-004 | B-08 | control-mapping | Specification | MUST | Each in-scope threat MUST map to one or more preventive, detective, or corrective controls in the profile. |
| SPEC-B-09-001 | B-09 | csv-validation | Specification | MUST | The specification MUST use CSVW metadata to define the structure, dialect, datatypes, and validation rules of CSV Distributions. |
| SPEC-B-09-002 | B-09 | json-validation | Specification | MUST | The specification MUST use JSON Schema Draft 2020-12 to define structural validation rules for JSON Distributions. |
| SPEC-B-09-003 | B-09 | rdf-validation | Specification | MAY | The specification MAY define SHACL shapes for validating RDF graph constraints in RDF-based profiles. |
| CONF-B-09-004 | B-09 | conformance-tests | Validator | MUST | The Validator MUST report structural and semantic validation results separately. |
| CONF-B-09-005 | B-09 | csv-validation | Publisher | MUST | A Publisher producing a CSV Distribution MUST publish the matching CSVW metadata. |
| CONF-B-10-001 | B-10 | audit-target | AuditArtifact | MUST | An Audit Artifact MUST identify the exact immutable Dataset Version covered by the audit. |
| CONF-B-10-002 | B-10 | audit-period | AuditArtifact | MUST | An Audit Artifact MUST identify the reporting period and audit coverage period. |
| CONF-B-10-003 | B-10 | auditor-and-opinion | AuditArtifact | MUST | An Audit Artifact MUST identify the auditor and the audit opinion or result category. |
| CONF-B-10-004 | B-10 | manifest-linkage | AuditArtifact | MUST | An Audit Artifact MUST identify the manifest digest of the Dataset Version it covers. |
| SPEC-B-11-001 | B-11 | checkpoint-schema | Specification | MUST | The specification MUST define a machine-readable Checkpoint Evidence object containing dataset identity, head version, manifest digest, sequence, and evidence time. |
| CONF-B-11-002 | B-11 | checkpoint-publication | Publisher | MUST | The Publisher MUST publish checkpoint evidence for the current Dataset Version through an independently verifiable channel. |
| CONF-B-11-003 | B-11 | validator-check | Validator | MUST | The Validator MUST compare a candidate head with an authenticated checkpoint or a trusted latest-seen checkpoint. |
| SPEC-B-11-004 | B-11 | state-semantics | Specification | MUST | The Checkpoint Evidence Interface MUST define the verification states fresh, stale, conflicting, invalid, and unknown. |
| CONF-B-11-005 | B-11 | rollback-detection | Validator | MUST | The Validator MUST reject a superseded version as current when trusted checkpoint evidence proves a later head. |
| SPEC-B-11-006 | B-11 | backend-interoperability | Specification | MUST | All permitted checkpoint backends MUST implement the common evidence and verification semantics defined by the profile. |
| CONF-B-12-001 | B-12 | internal-id-separation | Publisher | MUST NOT | A Publisher MUST NOT expose internal database primary keys or business sequence numbers as Public Persistent Identifiers. |
| CONF-B-12-002 | B-12 | timestamp-leakage | Publisher | MUST | A Publisher using a time-encoding public identifier such as UUIDv7 MUST document a privacy assessment of the creation-time disclosure. |
| SPEC-B-12-003 | B-12 | enumeration | Specification | MUST | The identifier profile MUST define controls against predictable or enumerable public identifiers where enumeration creates a disclosure risk. |
| CONF-B-12-004 | B-12 | capability-separation | Publisher | MUST NOT | A Public Persistent Identifier MUST NOT be used as an authorization secret or bearer capability. |
| CONF-B-12-005 | B-12 | identifier-persistence | Publisher | MUST | A Publisher MUST preserve the identity of a public resource across revisions while assigning a distinct identifier to each immutable version. |
| SPEC-B-13-001 | B-13 | event-taxonomy | Specification | MUST | The accounting event model MUST distinguish donations, expenses, refunds, reversals, transfers, chargebacks, write-offs, allocation releases, and fund reclassifications. |
| CONF-B-13-002 | B-13 | correction-linkage | DatasetPackage | MUST | A correction or reversal event MUST reference the original event or transaction that it corrects or reverses. |
| CONF-B-13-003 | B-13 | balance-invariant | DatasetPackage | MUST | Each account or fund balance for a reporting period MUST satisfy opening balance plus inflows minus outflows plus or minus adjustments equals closing balance. |
| CONF-B-13-004 | B-13 | amount-and-currency | DatasetPackage | MUST | Each monetary amount MUST declare its currency using an ISO 4217 code and a profile-defined exact decimal representation. |
| CONF-B-13-005 | B-13 | time-interval | DatasetPackage | MUST | Each dated accounting event MUST use the date or timestamp representation defined by the profile and RFC 3339 where a timestamp is required. |
| CONF-B-13-006 | B-13 | reconciliation | Validator | MUST | The Validator MUST check the profile-defined reconciliation rules across funds, accounts, periods, and allocations. |
| SPEC-B-14-001 | B-14 | api-description | Specification | MUST | The API profile MUST define a version-pinned OpenAPI description and the supported feature set. |
| CONF-B-14-002 | B-14 | immutable-version-retrieval | DataService | MUST | A DataService MUST provide a stable retrieval method for a specific immutable Dataset Version. |
| CONF-B-14-003 | B-14 | snapshot-consistency | DataService | MUST | A DataService MUST ensure that pagination over an immutable Dataset Version refers to one consistent snapshot. |
| CONF-B-14-004 | B-14 | stable-pagination | DataService | MUST | A DataService MUST provide deterministic ordering and stable continuation semantics for paginated immutable collections. |
| CONF-B-14-005 | B-14 | etag-semantics | DataService | SHOULD | A DataService SHOULD provide a strong ETag for an immutable representation when HTTP conditional requests are supported. |
| CONF-B-14-006 | B-14 | digest-semantics | DataService | SHOULD | A DataService SHOULD use Content-Digest for HTTP message content integrity and Repr-Digest for selected representation integrity according to RFC 9530. |
| SPEC-B-14-007 | B-14 | error-semantics | Specification | MUST | The API profile MUST define stable error categories for invalid identifiers, unavailable versions, invalid pagination tokens, and authorization failures. |
| CONF-B-15-001 | B-15 | immutable-correction | Publisher | MUST | A Publisher MUST publish corrections as a new immutable Dataset Version rather than overwrite a previously released version. |
| CONF-B-15-002 | B-15 | public-redaction | Publisher | MUST | A Publisher MUST provide a redaction or tombstone procedure for previously published personal or protected information that requires removal. |
| CONF-B-15-003 | B-15 | internal-audit-history | Publisher | MUST | A Publisher MUST preserve complete internal auditability for the applicable retention period without making internal records public by default. |
| SPEC-B-15-004 | B-15 | retention | Specification | MUST | The specification MUST require each Publisher to define retention periods in accordance with applicable law and approved records policy. |
| CONF-B-15-005 | B-15 | legal-hold | Publisher | MUST | When an applicable legal hold is in effect, the Publisher MUST suspend deletion of records within the scope of that hold. |
| CONF-B-15-006 | B-15 | prepublication-review | Publisher | MUST | A Publisher MUST complete privacy and disclosure review before anchoring a public artifact in an external append-only or timestamping system. |
| SPEC-B-16-001 | B-16 | oim-conformance | Specification | MUST | The Tier 3 XBRL profile MUST use an Open Information Model conformant representation. |
| CONF-B-16-002 | B-16 | xbrl-csv-mti | Publisher | MUST | A Tier 3 Publisher MUST publish xBRL-CSV as the Mandatory-to-Implement XBRL serialization. |
| CONF-B-16-003 | B-16 | table-constraints | Publisher | MUST | A Tier 3 xBRL-CSV Distribution MUST conform to the applicable profile requirements of xBRL-CSV Table Constraints 1.0. |
| SPEC-B-16-004 | B-16 | taxonomy-mapping | Specification | MUST | The XBRL mapping artifact MUST define source account code, source label, target concept, mapping version, and effective period. |
| SPEC-B-16-005 | B-16 | alternate-serialization | Specification | MAY | The XBRL profile MAY permit xBRL-JSON or xBRL-XML as additional serializations beyond the MTI xBRL-CSV representation. |

## 24. 主要参照資料

- [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.html) / [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174.html) —
  BCP 14
- [RFC 7515](https://www.rfc-editor.org/rfc/rfc7515.html) — JWS
- [RFC 7517](https://www.rfc-editor.org/rfc/rfc7517.html) — JWK
- [RFC 7797](https://www.rfc-editor.org/rfc/rfc7797.html) — JWS b64 extension
- [RFC 8785](https://www.rfc-editor.org/rfc/rfc8785.html) — JCS
- [RFC 3161](https://www.rfc-editor.org/rfc/rfc3161.html) — Timestamp Protocol
- [RFC 9530](https://www.rfc-editor.org/rfc/rfc9530.html) — HTTP Digest Fields
- [RFC 9562](https://www.rfc-editor.org/rfc/rfc9562.html) — UUID
- [RFC 9864](https://www.rfc-editor.org/rfc/rfc9864.html) — Fully-Specified JOSE/COSE Algorithms
- [YAML 1.2.2](https://yaml.org/spec/1.2.2/)
- [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12)
- [W3C DCAT 3](https://www.w3.org/TR/vocab-dcat-3/)
- [W3C CSVW Model](https://www.w3.org/TR/tabular-data-model/) / [Metadata](https://www.w3.org/TR/tabular-metadata/)
- [OpenAPI 3.2.1](https://spec.openapis.org/oas/v3.2.1.html)
- [XBRL OIM 1.0](https://specifications.xbrl.org/work-product-index-open-information-model-open-information-model.html)
- [xBRL-CSV Table Constraints 1.0](https://www.xbrl.org/Specification/table-constraints/REC-2026-09-10/table-constraints-REC-2026-09-10.html)

参照版はCandidate Specificationで再確認し、Registryの変更履歴とともに固定する。

## 参照

- 原本: `public_nonprofit_accounting_v3_wd03_full.md`（GPT-6、2026-11-09 12:51 JST、WD 0.3、SHA-256:
  `5a732dcfb6d636949907bb0b41c376c6ebb2eef999c01686b086c95ce8d922c0`）
- 原本パッケージ: `public_nonprofit_accounting_v3_wd03_artifacts.zip`（SHA-256:
  `025ff9f6bdb6541ad94e054e344ef06633b5006fda44dd3eeeb2a2cba05397a6`）
- パッケージ同梱物: `registries/`（blockers・references・targets・verification-methods）、`requirements/`（B-01〜B-16）、`schemas/`、
  `generated/`（requirements・coverage・validation-report）
- 検証結果: PASS（16 Blocker / 89 Requirements、semantic・schema・YAML エラーなし。外部相互運用性テストは未実施）
- 本リポジトリへの取り込みに際し、frontmatter・見出し階層・表記（表の整形、行長調整）のみをリポジトリ規約に合わせて調整した。本文の内容は変更していない。
