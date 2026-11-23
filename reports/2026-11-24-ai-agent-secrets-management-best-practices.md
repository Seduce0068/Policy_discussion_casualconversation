---
type: Report
title: "AIエージェント時代の秘密・機微情報管理のベストプラクティス調査"
description: "2026年11月時点の一次情報をもとに、エージェント認証・動的シークレット・実行時制御・ローカル保管・規制要件を横断調査した"
tags:
  - ai-agent
  - secrets-management
  - security
  - best-practices
generated:
  by: lead
  at: "2026-11-24T00:16:52+09:00"
status: "draft"
---

## 概要

AIエージェントが外部APIや社内システムを自律的に呼ぶ構成では、APIキーをモデルへ渡す従来の方法がそのまま漏洩経路になる。本報告は2026年11月24日時点の一次情報をもとに、設計・運用・規制の
要件を横断して整理した。

結論は次の4点である。

- 秘密はモデルコンテキストへ入れず、ツール境界で注入する。
- 永続的な秘密の配布をやめ、エージェント個別のアイデンティティと短命トークンを使う。
- プロンプトインジェクションは完全防止が困難であるため、設定ファイルの拒否ルール、エグレス制御、人による承認、監査で多重に守る。
- 漏洩検知時の第一対応は履歴の書き換えではなく、認証情報の失効とローテーションである。

調査対象はOWASP、MITRE ATLAS、IETF、MCP仕様、主要クラウド3社、Vault系、エージェントSDK、シークレットスキャナ、Kubuntu向けローカル保管、および日米欧の規格・法令
である。暗号アルゴリズム自体の評価、実機ベンチマーク、個別製品の導入費用の算定は対象外とした。

## 調査方法

`swarm-research` のR1〜R6に従い、R1で論点と情報源を割り当て、R2で8観点を並列調査した。R3で2観点を深掘りし、R4で別の4エージェントが原典を敵対的に再確認した。R5で2観点
の未確認事項を補完し、R6で親エージェントが重複と矛盾を吸収して本報告を確定した。子エージェントの総数は16体である。

公式ドキュメント、GitHubの原典、IETF Datatracker、OWASP、NIST、日本の官公庁文書を優先した。AGENTS.md §16に従い、`web_search` と
`mcp__rustxng__*` は呼ばず、`web_fetch` とGitHub MCPのみで取得した。各主張には取得元と原文引用を付け、取得日をRFC 3339で記録した。R4の検証で、日付、規
格名、製品機能の範囲に複数の修正が生じている。修正内容は該当箇所に反映した。

## 確認された脅威モデル

### OWASP LLM Top 10 の2026年版

最新版ではLLM07がMisinformation、LLM08がHidden Context Exposureである。システムプロンプトや隠しコンテキストへの認証情報の埋め込みは、2025年版の
LLM07 System Prompt LeakageからLLM08へ移った。LLM08は「Do not embed credentials, secrets, or
security-critical configuration directly in the system prompts or hidden context. Assume all context
available to the LLM could also be available to users.」と明記する[llm08-2026]。2025年版も「Avoid embedding any
sensitive information (e.g. API keys, auth keys, database names, user roles, permission structure of
the application) directly in the system prompts.」としていた[llm07-2025]。

LLM01 Prompt Injectionは、機微情報の開示、AIシステム基盤やシステムプロンプトの露出、関数への不正アクセス、接続システムでの任意コマンド実行を招く[llm01-2025]。
LLM06 Excessive Agencyの原因は、過剰な機能、過剰な権限、過剰な自律性のいずれかまたは複数である[llm06-2025]。OWASPは、プロンプトインジェクションの完全な防止方法は
現時点で明らかでないと述べる[llm01-2025]。

### OWASP Top 10 for Agentic Applications 2026

ASI01からASI10までの正式名は、Agent Goal Hijack、Tool Misuse and Exploitation、Identity and Privilege Abuse、
Agentic Supply Chain Vulnerabilities、Unexpected Code Execution (RCE)、Memory & Context Poisoning、
Insecure Inter-Agent Communication、Cascading Failures、Human-Agent Trust Exploitation、Rogue Agentsである
[asi-list]。

認証情報に直接関わるのはASI03 Identity and Privilege Abuseである。原文は「Identity in this context includes both the
agent's assigned persona and any authentication material (API keys, OAuth tokens, delegated user
sessions) that represent it.」とし、対策として「Bind OAuth tokens to a signed intent that includes subject,
audience, purpose, and session.」および「Issue short-lived, narrowly scoped tokens per task and cap
rights with permission boundaries」を挙げる[asi03]。ASI10 Rogue Agentsは「Keys must never be directly
available to agents; instead, orchestrators should mediate signing operations」とし、per-runの一時資格情報と1回限り
のaudience束縛を求める[asi10]。エージェント間の通信にはASI07が「Use end-to-end encryption with per-agent credentials and
mutual authentication.」を求める[asi07]。

公開日は2025年12月10日である。2026年10月28日は専用ランディングページの公開日であり、同ページの最終更新は2026年11月16日である[asi-dates]。R2の調査では「
Cross-Agent Trust Exploitation」をASI03とする記述が出たが、R4の検証で誤りと判定し、ASI07の領域として扱う。

### OWASP MCP Top 10 2025

本プロジェクトは「Phase 3 – Beta Release and Pilot Testing - We are here right now」の段階にある[mcp-top10]。MCP01は
Token Mismanagement & Secret Exposureで、長寿命セッションと状態の永続化により、トークンが「inadvertently stored, indexed, or
retrieved later through user prompts, system recalls, or log inspection」されると指摘する[mcp01]。ログ、テレメトリ、ベクト
ルストアがリダクションなしでプロンプト全体を記録する事例も挙げる[mcp01]。MCP03 Tool Poisoningは、ツール説明に`~/.ssh`、`id_rsa`、`.env`、
`.aws/credentials`といった秘密のパスを埋め込む手口を扱う[mcp03]。MCP10 Context Injection & Over-Sharingは、セッションやエージェント、テナ
ント間の情報漏洩を扱う[mcp10]。

### MITRE ATLAS

最新リリースは2026.10（2026年10月28日、format-version 6.1.0）である[atlas]。AML.T0083はCredentials from AI Agent
Configuration、AML.T0080.001はAI Agent Context Poisoning: Memoryであり、設定ファイル内のAPIキーやトークン、メモリ汚染が攻撃手法として収録
されている[atlas]。自社のツール呼び出しとメモリ更新をこれらの手法へマッピングすると、検知ルールの設計に使える。

## 認証・認可の設計原則

### エージェント個別のアイデンティティ

Microsoft Entra Agent IDは、エージェント種別ごとのBlueprintとインスタンスごとのAgent Identityを提供する。ダウンストリームのトークンは「preserves
the user's identity as the subject and identifies the agent identity as the actor」であり、「Access is
scoped to delegated permissions assigned to the agent identity, ensuring the agent can't exceed its
granted permissions even when operating with user context」である[entra]。ドキュメントの最終更新は2026年11月17日である
[entra]。

AWS Bedrock AgentCore Identityは、エージェントを「workload identities with specialized attributes」として扱い、「
Centralized credential management eliminates the need to embed secrets in agent code or
configuration files」と述べる[bedrock]。Google SAIFは「In extreme cases, agents can even reveal credentials
and API keys they have been trusted with.」と警告し、ツールにはleast-privilegeを求める[saif]。

一方、Microsoft Agent 365のConnected platformsは、現時点では「Every currently supported platform requires a
key-based credential, such as an API key, secret, or private key. Keyless authentication options,
such as workload identity federation or role assumption, aren't currently supported」である[m365]。ベンダ機能だ
けでkeyless化できる前提には立たない。

### MCP認可仕様 2026-07-28

MCP認可仕様の最新版は2026-07-28である[mcp-auth]。OAuth 2.1は2026年10月31日付のdraft-17が最新だが[oauth21]、MCP仕様はdraft-13を参照し
たままであり、Refresh Tokens節のリンクのみdraft-14が混在する[mcp-auth]。OAuth 2.1はInternet-Draftであり、まだRFCではない。

クライアントはPKCEの実装と、認可サーバがPKCEに対応するかの確認が必須である。メタデータに`code_challenge_methods_supported`が無ければ認可へ進んではならない。S256は「
when technically capable」という条件付きの必須である[mcp-auth-sec]。`resource`パラメータは認可要求とトークン要求の双方に含め、RFC 8707に従ってト
ークンのaudienceを制限する[mcp-auth]。

トークンパススルーは明示的に禁止されている。「The MCP server MUST NOT pass through the token it received from the MCP
client.」および「MCP servers MUST NOT accept or transit any other tokens.」とある[mcp-auth-sec]。上流APIへは、認可サーバ
が発行した別のトークンを使う。

秘密の入力にはelicitationのURL modeを使う。form modeでパスワード、APIキー、アクセストークン、決済情報を要求してはならず、URL modeでは「sensitive
credentials never pass through the LLM context, MCP client or any intermediate MCP servers」と明記される
[mcp-elicit]。クライアントはURLの内容とユーザー入力を検査できない方法で開く必要がある[mcp-elicit]。

2026-07-28では、OAuth 2.0 Dynamic Client Registrationが非推奨となり、Client ID Metadata Documents（CIMD）へ移行する。削除
は2027年7月28日以降の最初のリビジョンである[mcp-changelog]。併せてRFC 9207のissuer検証と、client credentialsの発行者への束縛が追加された
[mcp-ga]。MCPプロキシが静的client IDを使う場合、第三者認可サーバへ転送する前にクライアントごとのユーザー同意が必須である[mcp-auth-sec]。

### OAuthの既存基盤と標準化動向

AIエージェント専用のRFCはまだない。IETF OAuth WGに`draft-ietf-oauth-ai-*`という名称のドラフトは存在せず、Datatrackerの検索結果は0件である
[oauth-ai-none]。Individual Submissionとしては、エージェント認可のユースケース分析[oauth-agent-use]、トークン交換による権限の減衰と
resource-bound・sender-constrainedトークンを扱う提案[oauth-agent-grants]、署名済みの意図を表すIntent Token[oauth-intent]、
トークン無効化と権限クロージャを区別する提案[oauth-revocation]がある。いずれもWG採用ドラフトではない。

採用済みの基盤は次の3点セットである。RFC 8693は委譲と偽装を区別し、`act`クレームで委譲チェーンを表す。権限乱用の緩和策としてscopeと短いトークン寿命を挙げる[rfc8693]。RFC
8707は対象resourceを明示する[rfc8707]。RFC 9449（DPoP）は「binding a token to a public key upon issuance and
requiring that the client proves possession of the corresponding private key」により漏洩トークンの再利用を防ぐが、HTTPS
の代替にはならない[rfc9449]。DPoPはMCP 2026-07-28の準拠リストに含まれず、任意拡張として扱えるかは未確認である。

### A2Aプロトコル

A2A v1.0はエージェントを標準的なエンタープライズアプリケーションとして扱い、資格情報を帯域外で取得する[a2a]。帯域内で資格情報を交換すると、連鎖する複数のエージェントへ資格情報が露出する
ためである。in-task認可では「Credentials SHOULD be bound to the agent which originated the request」とされ、v1.0で
`pkce_required`フィールドが追加された[a2a]。v1.0の公開日は一次情報で未確認である。

## シークレット管理基盤

### 動的シークレットとSecret Zero

Vaultの動的シークレットは必ずリース（TTL）を持ち、期限切れ時に自動失効する。利用者は「check in with Vault routinely to either renew the
lease (if allowed) or request a replacement secret」が必要である[vault-lease]。この仕組みにより監査ログの価値が上がる。

VaultのAWSシークレットエンジンは、IDトークンをSTS認証情報へ交換するWIFを提供する。WIFはEnterprise機能で、既定TTLは1時間、期限切れ時に自動更新される
[vault-aws]。AppRoleではPull modeとResponse Wrappingが推奨され、`secret_id_num_uses`を1、`secret_id_bound_cidrs`
による送信元制限が推奨される。信頼できるサードパーティ認証が使える場合は、AppRoleよりそちらを優先するよう公式が明記する[vault-approle]。KubernetesではPodの
ServiceAccountトークンがSecret Zeroとなり、別のアイデンティティを配布しない[vault-k8s]。自動更新が必要ならSidecar Injector、起動時マウントで十分ならCSI
Driverを選ぶ。後者は自動更新に対応しない[vault-k8s]。

Response Wrappingは、通信路へ実シークレットではなく参照トークンを流す。「Information stored in logs or captured along the way do
not directly see the sensitive information.」とあり、wrapperのlookupで未unwrapや期限切れを検知できる[vault-wrap]。

他製品の動向は次のとおりである。SPIFFE/SPIREは短寿命のSVIDをWorkload APIで配り、node attestationの後にworkload attestationを行う。最新の
SPIREはv1.16.0（2026年11月5日）である[spire]。CyberArk Secretless Brokerは、設計上クライアント認証を行わないため、ネットワーク隔離が「a hard
deployment requirement, not an optional hardening step」である[secretless]。Dopplerの動的シークレットは既定TTLが30分で、サ
ービスごとに新しいリースを取得する[doppler]。1PasswordのCredential BrokerはOIDCでワークロードを検証し、ワークフロー実行中のみ公開するがpublic
previewである[onepassword]。Infisicalの動的シークレットはAdvancedプラン機能で、self-hostでは営業への問い合わせが必要である[infisical]。

### クラウドの既存機能

| 領域 | 確認済みの事実 | 出典 |
| --- | --- | --- |
| AWS Secrets Manager | ローテーションは最短4時間、最長999日。CloudTrailコンソールのEvent historyは直近90日のmanagement eventsのみで、data eventsは表示されない | [aws-rotate], [aws-cloudtrail] |
| AWS Workload Credentials Provider | 旧Secrets Manager Agent。既定TTLは300秒。アプリからのローカル呼び出しはCloudTrailに出ないが、ProviderがSecrets Manager/ACMへ発行する呼び出しは記録される | [aws-provider] |
| AWS EKS IRSA / STS | OIDCの署名鍵は7日ごとにローテーション。AssumeRoleWithWebIdentityの既定は1時間、指定範囲は900〜43200秒 | [irsa], [sts] |
| Google Cloud | サービスアカウントキーのエクスポートよりWorkload Identity Federationを推奨。Secret Managerのバージョンは不変で、本番ではバージョン番号にピン留めする | [gcp-best], [gcp-rotation] |
| Google Cloud ローテーション | `rotation_period`は1時間以上、`next_rotation_time`は現在から5分以上先。Data Access監査ログで読み取りも追跡できる | [gcp-rotation], [gcp-audit] |
| Azure | Azure内はManaged Identity、外部はWorkload Identity Federation。シークレットの自動ローテーションはEvent GridとFunctionsによるイベント駆動 | [azure-mi], [azure-wif], [azure-rotation] |
| Azure Key Vault / App Service | 最低7日は「キー」のローテーション間隔に限られる。シークレットに固定間隔の下限はない。Key Vault参照は新バージョンを24時間以内に反映 | [azure-key-rotation], [azure-ref] |

R2の調査では「Azure Key Vaultの自動ローテーションは最低7日」と一般化されていたが、R4の検証でキーに限られることが判明し、表のように修正した。

### GitOpsとシークレットスキャン

Gitへ置く暗号化方式と、外部から注入する方式がある。Sealed Secretsは「Encrypt your Secret into a SealedSecret, which *is* safe
to store - even inside a public repository.」としつつ、「The *sealing key* renewal and SealedSecret
rotation are **not a substitute** for rotating your actual secrets.」と明記する[sealed]。SOPSはageを推奨し[sops]
、External Secrets OperatorとSecrets Store CSI Driverは外部ストアの値をPodへ注入する[eso]、[csi]。暗号化とローテーションは別の作業である。

GitHub Secret Scanningは「scans your entire Git history on all branches」であり、アラートを受けたら認証情報を直ちにローテーションすべ
きと述べる[ghs]。Push Protectionはシークレットを含むpushをリポジトリ到達前にブロックする[ghp]。バイパス時にもアラートが残る[ghs]。

ツールの保守状況は選定に直結する。Gitleaksは「feature complete. I'm not merging new features into Gitleaks. Future
releases will be security patches only.」であり、開発陣はBetterleaksへ移行している[gitleaks]。Betterleaksの安定版はv1.10.1
（2026年11月12日）で、v2はrc.9（2026年11月17日、prerelease）である[betterleaks]。TruffleHogはv3.100.0（2026年11月19日）で、検証に
よる誤検知の排除と`--fail`によるCIゲートを提供する[trufflehog]。detect-secretsはv1.5.0（2024年5月6日）で、新規差分のみを検出するbaseline方式であ
る[detect]。

ローカルフックは`SKIP=gitleaks`や`--no-verify`で回避できるため、CIを最終ゲートとして併設する[gitleaks]、[git-secrets]。Canary tokenをス
キャナの検知対象に含め、ゲートの実効性を定期的に確認する方法もある[trufflehog]。

## 実行時制御

### 設定で強制する境界（Claude Code）

権限ルールは「deny, then ask, then allow」の順で評価され、広いdenyは狭いallowで例外化できない[claude-perm]。公式の設定例は`.env`と
`secrets/**`の読み取りをdenyする[claude-example]。ただし`Read(./.env)`は`cat .env`のような明示的な参照を止めても`grep -r`によるディレク
トリ走査は止められないため、sandboxのdenyReadと併用する[claude-example]。

managed settingsはCLI引数を含む他の設定階層から上書きできない。`allowManagedPermissionRulesOnly`と
`disableBypassPermissionsMode: "disable"`を組み合わせると、権限ルールの出所を固定できる[claude-perm]、[claude-example]。ネットワー
クは`sandbox.network.strictAllowlist`で許可リスト外を拒否でき、`allowManagedDomainsOnly`で管理設定だけがドメインを追加できる。前者はユーザー・
managed・CLIの`--settings`に置く必要があり、リポジトリ内の設定では効かない。v2.1.219以降が必要である[claude-sandbox]。カスタムプロキシを使う場合は、
Claude Code側のドメイン制御がその通信へ適用されなくなる点に注意する[claude-sandbox]。

`apiKeyHelper`はVaultから短命トークンを取得する用途に使えるが、サンドボックス外でフルアクセスで実行され、信頼未確定のフォルダでも動くため、それ自体は秘密を守る境界にならない
[claude-auth]、[claude-sandbox]。AnthropicのHIPAA用設定例は、`~/.ssh`、`~/.aws`、`.npmrc`、`.git-credentials`や主要
トークン環境変数をRead/Edit/credentialsの三重にdenyする[hipaa-example]。ただし同例のREADMEは、承認済みのプラグイン、MCPサーバ、フックはサンドボックス外
で動き、注入された命令が開発者のシェルとGitを使って内容を移動できると警告する[hipaa-readme]。設定例は出発点であり、完全な境界ではない。

CIでは、baseブランチの`apiKeyHelper`や`statusLine`がPR側のファイルを解決して実行される問題への対処として、固定バージョンの直接呼び出しが推奨される。認証情報は
GitHub secretsへ置き、`show_full_output`は既定のfalseを維持する[claude-action]。

### エージェントSDKの制御（OpenAI Agents SDK）

最新のv0.25.0（2026年11月17日）では、機微ツールに`needs_approval=True`を付け、実行前に人へ差し戻せる[oasdk-hitl]、[oasdk-release]。
`always_approve`はエージェント単位ではなくツールの種類と名前で一致するため、ハンドオフ先の別エージェントへ波及する恐れがある。per-callの決定を使う[oasdk-hitl]。

シリアライズされたRunStateには`tool_input`とアプリケーションコンテキストが含まれる。秘密を`RunContextWrapper.context`へ置くと、保存や送信の対象になる
[oasdk-hitl]。トレースは既定で入出力を記録するため、`OPENAI_AGENTS_TRACE_INCLUDE_SENSITIVE_DATA=0`を標準化する。リダクションの成功をエクスポー
トの条件にするなら、`add_trace_processor()`ではなく`set_trace_processors()`で自前のexporterへ差し替える[oasdk-tracing]。ツール引数
へ秘密が流れる経路には、エージェント単位の入力ガードレールではなく`tool_input_guardrails`を付ける。公式例はJSON引数に`sk-`を含める呼び出しを拒否する
[oasdk-guardrails]。

## Kubuntuローカル環境での保管

| 手段 | 守れるもの | 守れないもの・注意 | 出典 |
| --- | --- | --- | --- |
| GNOME Keyring / libsecret | パスワード本文。属性で項目を検索できる | libsecretは属性値を非機密とみなすため、属性へrefresh tokenや有効期限を入れられない | [secret-tool], [git-libsecret] |
| git-credential-libsecret | OSのSecret Service連携による資格情報の保管 | `git-credential-store`は`.git-credentials`へ平文保存するため、対話環境では使わない | [git-store], [git-libsecret] |
| pass | 各パスワードのGPG暗号化ファイル | ファイル名（サイト名）とGitコミット履歴は暗号化されない。gpg-agentの既定キャッシュは600秒 | [pass], [gnupg] |
| age | ファイル内容の暗号化 | SSH公開鍵宛は公開鍵タグで追跡可能。ssh-agentとYubiKey上のSSH鍵は復号に使えない | [age] |
| direnv | `.envrc`の明示的なallow | allow後は環境変数として全子プロセスへ渡る。`.envrc`をコミットせず、`.envrc.local`をignoreする | [direnv] |
| systemd-creds | AES256-GCMによる機密性と完全性 | `--with-key=null`では機密性も真正性も提供されない | [systemd-creds] |
| Podman secrets | イメージとソース管理への混入の防止 | 既定のfileドライバは暗号化しない。長期保存には`--driver=pass`、コンテナ渡しには`--secret`を使う | [podman] |
| ssh-agent / gpg-agent | 秘密鍵をネットワークへ送らず認証 | ssh-agentソケットは同一ユーザーの別プロセスやrootに悪用され得る。gpg-agentの既定は復号600秒、SSH 1800秒 | [ssh-agent], [gnupg] |
| YubiKey FIDO2 / PIV | デバイス内での署名・復号 | 既定PINは123456、PUKは12345678。テスト以外で使う前に変更する | [yubico] |

## 規格・法令上の要件

| 文書 | 確認済みの要件・版 | 出典 |
| --- | --- | --- |
| NIST SP 800-53 Rev.5.2.0 | 2025年8月27日発行のマイナーリリース。IA-5は差分に含まれない | [nist53] |
| NIST SP 800-63B-4 | 2025年7月31日最終版。パスワードはsalted hashで保存し、検証者の秘密鍵はHSMまたはTEEに置く | [nist63b] |
| NIST SP 800-218 / 218A | SSDF v1.1は認証情報管理を含む高リスク領域の厳格な評価を求める。218A（Generative AI Profile）は2024年7月26日最終版 | [ssdf], [ssdfa] |
| NIST SP 800-207 | ネットワーク位置による暗黙の信頼を否定し、セッション前に主体とデバイスの双方を認証・認可する（2020年8月） | [zt] |
| NIST AI RMF 1.0 | 2023年1月26日公開。機微データのガバナンス整合を求める。現在改訂中 | [airmf] |
| GDPR 第32条 | 仮名化・暗号化、機密性/完全性/可用性/レジリエンス、復旧、定期的な有効性評価を求める | [gdpr32] |
| 個人情報保護法 | 2条3項=要配慮個人情報、20条2項=同意なき取得の禁止、23条=安全管理措置、25条=委託先の監督 | [appi] |
| 個人情報保護委員会 | 2023年6月2日の注意喚起で、生成AIへの入力が利用目的の範囲内か、学習に使われないかを確認するよう求める | [ppc] |
| AI事業者ガイドライン第1.2版 | 2026年3月31日。「AIシステム・サービスに機密情報等を不適切に入力することがないよう注意を払う」と利用者側の義務を明記 | [aig-guideline] |
| IPA 手引書 | 2026年7月31日更新。MFA必須化、APIキーは専用ツールと定期ローテーション、廃棄時に外部連携の認可設定も削除 | [ipa] |

R4とR5の検証で、次の修正を行った。第一に、ISO/IEC 27001:2022 Annex Aの8.24は「Use of cryptography（暗号の使用）」であり、「秘密の使用」ではない。第
二に、UK GDPR第32条は採択時のEU版と異なり、3項の文言が2025年8月20日に、4項の文言が2020年12月31日に置き換えられている[ukgdpr]。第三に、法務省の公式英訳は2003年版
のままで、現行日本語原文と条文番号が一致しないため、対外資料では日英の条番号を併記する必要がある[appi-en]。

## 推奨される次の一手

優先度P0は今すぐ設計へ反映すべき項目である。

1. APIキー、アクセストークン、データベース名、権限構造をシステムプロンプトとプロンプト文字列から排除する。秘密の入力にはMCPのURL modeを使う[llm08-2026]、
   [mcp-elicit]。
1. エージェント種別ごとにアイデンティティを分け、タスク単位の短命トークンを発行する。トークンにはsubject、audience、purpose、sessionを束縛する[asi03]、[entra]。
1. 権限ルールを設定ファイル側へ移し、deny、エグレス許可リスト、人による承認を組み合わせる。プロンプトによる自制に依存しない[claude-perm]、[oasdk-hitl]。
1. pre-commitとCIの二段スキャンを設け、push protectionを有効化する。検出した秘密はまず失効・ローテーションする[ghs]、[ghp]。

優先度P1は基盤の刷新である。

1. 永続的な秘密の代わりに、動的シークレットとWorkload Identity Federationを使う。配送にはResponse Wrapping、アイデンティティにはSVIDまたはマネージドアイデンティ
   ティを検討する[vault-wrap]、[spire]、[gcp-best]。
1. ログ、テレメトリ、ベクトルストアへの書き込み前にリダクションし、トレースへの機微データ記録を既定で無効化する[mcp01]、[oasdk-tracing]。
1. Kubuntuではlibsecretを既定とし、pass/ageのファイル名とGit履歴、direnvの環境変数継承、gpg-agentのキャッシュ時間を個別に点検する
   [git-libsecret]、[pass]、[direnv]。

優先度P2は文書と追跡である。未確認事項を解消し、MCPのCIMD移行（2027年7月28日以降）とAI RMFの改訂を四半期ごとに追跡する[mcp-changelog]、[airmf]。

## 未確認事項

- PCI DSS v4.0.1の要件3および要件8の公式原文は、取得経路がリダイレクトまたは404となり未確認である。
- ISO/IEC 27001:2022 Annex Aの5.15および5.17の公式原文は、ISO本体がHTTP 403で未取得である。8.24の名称はコミュニティ情報に基づき修正したが、一次情報で
  の再確認が必要である。
- NIST SP 800-53 Rev.5.2.0のIA-5統制文の全文は、OSCALカタログが大容量のため取得できなかった。
- EU GDPR第32条の現行統合版は、EUR-LexがHTTP 202を返すため未確認である。採択時原文とUK現行版のみを確認した。
- DPoPがMCP仕様で任意拡張として扱えるかは、仕様原典で確認できなかった。
- デジタル庁DS-920の本文PDFが取得できず、APIキー管理・エージェント認証の明示記述は未確認である。
- A2A v1.0の公開日、OpenAI Agents SDKドキュメントのバージョン、Google SAIFの最終更新日は未確認である。

## 参照

[llm08-2026]: https://genai.owasp.org/wp-json/wp/v2/llm_2026/58447
[llm07-2025]: https://raw.githubusercontent.com/OWASP/www-project-top-10-for-large-language-model-applications/main/2_0_vulns/LLM07_SystemPromptLeakage.md
[llm01-2025]: https://raw.githubusercontent.com/OWASP/www-project-top-10-for-large-language-model-applications/main/2_0_vulns/LLM01_PromptInjection.md
[llm06-2025]: https://raw.githubusercontent.com/OWASP/www-project-top-10-for-large-language-model-applications/main/2_0_vulns/LLM06_ExcessiveAgency.md
[asi-list]: https://genai.owasp.org/wp-json/wp/v2/search?search=ASI&per_page=20
[asi03]: https://genai.owasp.org/agentic-applications-2026/asi03-identity-and-privilege-abuse/
[asi07]: https://genai.owasp.org/agentic-applications-2026/asi07-insecure-inter-agent-communication/
[asi10]: https://genai.owasp.org/agentic-applications-2026/asi10-rogue-agents/
[asi-dates]: https://genai.owasp.org/wp-json/wp/v2/pages/58546
[mcp-top10]: https://owasp.org/projects/mcp-top-10
[mcp01]: https://raw.githubusercontent.com/OWASP/www-project-mcp-top-10/main/2025/MCP01-2025-Token-Mismanagement-and-Secret-Exposure.md
[mcp03]: https://raw.githubusercontent.com/OWASP/www-project-mcp-top-10/main/2025/MCP03-2025%E2%80%93Tool-Poisoning.md
[mcp10]: https://raw.githubusercontent.com/OWASP/www-project-mcp-top-10/main/2025/MCP10-2025%E2%80%93ContextInjection%26OverSharing.md
[atlas]: https://raw.githubusercontent.com/mitre-atlas/atlas-data/v6.1.0/dist/manifest.yaml
[entra]: https://learn.microsoft.com/en-us/entra/agent-id/agent-tokens
[bedrock]: https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/key-features-and-benefits.html
[saif]: https://saif.google/secure-ai-framework/focus-on-agents
[m365]: https://learn.microsoft.com/en-us/microsoft-agent-365/admin/connected-platforms
[mcp-auth]: https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization
[mcp-auth-sec]: https://raw.githubusercontent.com/modelcontextprotocol/modelcontextprotocol/2026-07-28/docs/specification/2026-07-28/basic/authorization/security-considerations.mdx
[mcp-elicit]: https://raw.githubusercontent.com/modelcontextprotocol/modelcontextprotocol/2026-07-28/docs/specification/2026-07-28/client/elicitation.mdx
[mcp-changelog]: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/docs/specification/2026-07-28/changelog.mdx
[mcp-ga]: https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/blog/content/posts/2026-07-28-spec-ga/index.md
[oauth21]: https://datatracker.ietf.org/doc/draft-ietf-oauth-v2-1/
[oauth-ai-none]: https://datatracker.ietf.org/api/v1/doc/document/?name__contains=draft-ietf-oauth-ai
[oauth-agent-use]: https://datatracker.ietf.org/api/v1/doc/document/draft-chen-oauth-agent-authz-use-cases/
[oauth-agent-grants]: https://datatracker.ietf.org/api/v1/doc/document/draft-mishra-oauth-agent-grants/
[oauth-intent]: https://datatracker.ietf.org/api/v1/doc/document/draft-chen-oauth-agent-intent-token/
[oauth-revocation]: https://datatracker.ietf.org/api/v1/doc/document/draft-watts-oauth-agent-revocation-closure/
[rfc8693]: https://www.rfc-editor.org/rfc/rfc8693.html
[rfc8707]: https://www.rfc-editor.org/rfc/rfc8707.html
[rfc9449]: https://www.rfc-editor.org/rfc/rfc9449.html
[a2a]: https://raw.githubusercontent.com/a2aproject/A2A/5f931b8a25fe66789f987ec4a55fc16ee9fe41ee/docs/specification.md
[vault-lease]: https://raw.githubusercontent.com/hashicorp/web-unified-docs/main/content/vault/v2.x/content/docs/concepts/lease.mdx
[vault-aws]: https://raw.githubusercontent.com/hashicorp/web-unified-docs/main/content/vault/v2.x/content/docs/secrets/aws.mdx
[vault-approle]: https://raw.githubusercontent.com/hashicorp/web-unified-docs/main/content/vault/v2.x/content/docs/auth/approle/approle-pattern.mdx
[vault-k8s]: https://raw.githubusercontent.com/hashicorp/web-unified-docs/main/content/vault/v2.x/content/docs/deploy/kubernetes/injector-csi.mdx
[vault-wrap]: https://raw.githubusercontent.com/hashicorp/web-unified-docs/main/content/vault/v2.x/content/docs/concepts/response-wrapping.mdx
[spire]: https://github.com/spiffe/spire/releases/tag/v1.16.0
[secretless]: https://raw.githubusercontent.com/cyberark/secretless-broker/master/README.md
[doppler]: https://docs.doppler.com/docs/dynamic-secrets
[onepassword]: https://www.1password.dev/brokered-access
[infisical]: https://raw.githubusercontent.com/Infisical/infisical/main/docs/documentation/platform/dynamic-secrets/overview.mdx
[aws-rotate]: https://docs.aws.amazon.com/secretsmanager/latest/userguide/rotate-secrets_schedule.html
[aws-cloudtrail]: https://docs.aws.amazon.com/awscloudtrail/latest/userguide/view-cloudtrail-events-console.html
[aws-provider]: https://raw.githubusercontent.com/aws/aws-workload-credentials-provider/7c6836be5ee2cea0d16ba2f79e53e8cfd7ba2a38/README.md
[irsa]: https://docs.aws.amazon.com/eks/latest/userguide/iam-roles-for-service-accounts.html
[sts]: https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoleWithWebIdentity.html
[gcp-best]: https://docs.cloud.google.com/secret-manager/docs/best-practices
[gcp-rotation]: https://docs.cloud.google.com/secret-manager/docs/secret-rotation
[gcp-audit]: https://docs.cloud.google.com/secret-manager/docs/audit-logging
[azure-mi]: https://learn.microsoft.com/en-us/entra/identity/managed-identities-azure-resources/overview
[azure-wif]: https://learn.microsoft.com/en-us/entra/workload-id/workload-identity-federation
[azure-rotation]: https://learn.microsoft.com/en-us/azure/key-vault/general/autorotation
[azure-key-rotation]: https://learn.microsoft.com/en-us/azure/key-vault/keys/how-to-configure-key-rotation
[azure-ref]: https://learn.microsoft.com/en-us/azure/app-service/app-service-key-vault-references
[sealed]: https://github.com/bitnami-labs/sealed-secrets/blob/main/README.md
[sops]: https://getsops.io/docs/usage/identities/age/
[eso]: https://github.com/external-secrets/external-secrets/blob/main/README.md
[csi]: https://github.com/kubernetes-sigs/secrets-store-csi-driver/blob/master/README.md
[ghs]: https://docs.github.com/en/code-security/concepts/secret-security/secret-scanning
[ghp]: https://docs.github.com/en/code-security/concepts/secret-security/push-protection
[gitleaks]: https://github.com/gitleaks/gitleaks/blob/master/README.md
[betterleaks]: https://github.com/betterleaks/betterleaks/blob/main/README.md
[trufflehog]: https://github.com/trufflesecurity/trufflehog/blob/main/README.md
[detect]: https://github.com/Yelp/detect-secrets/blob/master/README.md
[git-secrets]: https://github.com/awslabs/git-secrets/blob/master/README.rst
[claude-perm]: https://code.claude.com/docs/en/permissions.md
[claude-example]: https://code.claude.com/docs/en/settings-example.md
[claude-sandbox]: https://code.claude.com/docs/en/sandboxing.md
[claude-auth]: https://code.claude.com/docs/en/authentication.md
[hipaa-example]: https://github.com/anthropics/claude-code/blob/main/examples/settings/settings-hipaa.json
[hipaa-readme]: https://github.com/anthropics/claude-code/blob/main/examples/settings/README-hipaa.md
[claude-action]: https://github.com/anthropics/claude-code-action/blob/main/docs/security.md
[oasdk-hitl]: https://github.com/openai/openai-agents-python/blob/main/docs/human_in_the_loop.md
[oasdk-release]: https://github.com/openai/openai-agents-python/releases/tag/v0.25.0
[oasdk-tracing]: https://github.com/openai/openai-agents-python/blob/main/docs/tracing.md
[oasdk-guardrails]: https://github.com/openai/openai-agents-python/blob/main/docs/guardrails.md
[secret-tool]: https://dyn.manpages.debian.org/trixie/libsecret-tools/secret-tool.1.en.html
[git-libsecret]: https://raw.githubusercontent.com/git/git/master/contrib/credential/libsecret/git-credential-libsecret.c
[git-store]: https://git-scm.com/docs/git-credential-store
[pass]: https://www.passwordstore.org/
[gnupg]: https://www.gnupg.org/documentation/manuals/gnupg/Agent-Options.html
[age]: https://raw.githubusercontent.com/FiloSottile/age/main/README.md
[direnv]: https://direnv.net/
[systemd-creds]: https://www.freedesktop.org/software/systemd/man/latest/systemd-creds.html
[podman]: https://docs.podman.io/en/latest/markdown/podman-secret-create.1.html
[ssh-agent]: https://man.openbsd.org/ssh-agent.1
[yubico]: https://developers.yubico.com/PIV/Introduction/YubiKey_and_PIV.html
[nist53]: https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
[nist63b]: https://csrc.nist.gov/pubs/sp/800/63/b/4/final
[ssdf]: https://csrc.nist.gov/projects/ssdf
[ssdfa]: https://csrc.nist.gov/pubs/sp/800/218/a/final
[zt]: https://csrc.nist.gov/pubs/sp/800/207/final
[airmf]: https://www.nist.gov/itl/ai-risk-management-framework
[gdpr32]: https://www.legislation.gov.uk/eur/2016/679/article/32/adopted
[ukgdpr]: https://www.legislation.gov.uk/eur/2016/679/article/32
[appi]: https://laws.e-gov.go.jp/api/1/lawdata/415AC0000000057
[appi-en]: https://www.japaneselawtranslation.go.jp/en/laws/view/130
[ppc]: https://www.ppc.go.jp/news/careful_information/230602_AI_utilize_alert/
[aig-guideline]: https://www.soumu.go.jp/main_content/001099163.pdf
[ipa]: https://www.ipa.go.jp/jinzai/ics/core_human_resource/final_project/2026/ai-security.html
