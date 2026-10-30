---
type: Chat
title: "opencode.jsonc の deny ルールが黙って無視される —— パス表記不一致の系"
description: "「パーミッションの後出しdenyが機能しない」という伝聞を公式ドキュメントとissueで裏取り、opencode.jsonc の permissions の resource パス表記がツールの検査パスとリテラル不一致だと deny が静かに失効する実在バグを確認した"
tags:
  - chat
  - opencode
  - permissions
  - access-control
  - quiet-failure
generated:
  by: opencode
  at: "2026-10-30T18:12:35+09:00"
status: "draft"
---

## 文脈

- 2026-10-30 の雑談。「最新のopencodeのバグで、パーミッションの後出しdenyが
  機能しないらしい」という伝聞から、対象が `opencode.jsonc` の permissions
  の話と特定し、公式ドキュメント（V2）と issue トラッカーで裏を取った。
- ローカル環境は opencode v2.2.1。開発リポジトリは `sst/opencode` から
  `anomalyco/opencode` に移転しており、issue の一次情報は移転先にある。

## 結論

- 伝聞は実在するバグに対応する。ただし「後出し」の時間的解釈
  （セッション途中の設定変更が反映されない）に該当する open issue は
  見つけられず、確認できたのは「`opencode.jsonc` に書いた deny ルールが
  静かに無視される」系のバグだった。
- 最有力は [anomalyco/opencode#55728](https://github.com/anomalyco/opencode/issues/55728)
  （open、`reproduced` ラベル付き）。`read` の deny ルールの `resource` を
  `./Workshop/Checklist.md` のように `./` 付きで書くと、read ツールは
  Location 相対パス（`Workshop/Checklist.md`）でパーミッション検査するため、
  リテラル比較で絶対にマッチせず、deny が効かずファイル内容がモデルに返る。
  v2.0.20 の報告だが、メンテナが最新 `dev` でも再現を確認済みで、
  v2.2.1 への残存可能性が高い。
- 同根の不一致として [PR #53365](https://github.com/anomalyco/opencode/pull/53365)
  （open）がある。絶対パスのルール（`<home>/.opencode/plan/*`）がプロジェクト内
  の相対パス検査（`.opencode/plan/work.md`）にマッチせず、Plan agent の allow
  が死んで先頭の `edit *` deny が勝つ。方向は逆だが根は同じ
  「ルールのパス表記と検査パスの形式不一致」である。
- 公式ドキュメント（Permissions）は「保存済み approval（Allow always）は
  設定済み `deny` を決して上書きしない」と明記しており、設計上は
  `opencode.jsonc` の deny が常に最優先のはずである。それが効かないのは
  優先度ではなくマッチ自体が失敗しているためで、スキーマ検証も通り、
  エラーも警告も出ない。

## やり取り

### 伝聞の裏取り

- 「後出しdenyが機能しないらしい」という伝聞を、以下の順で検証した。
  - ローカルバージョン確認（v2.2.1）
  - リポジトリ特定（`sst/opencode` の検索が失敗し、公式ドキュメントの
    報告先と `gh search repos` で `anomalyco/opencode` と特定）
  - 公式 Permissions ドキュメントで設計原則を確認
  - issue トラッカーで deny 系バグを横断検索
- 結果として open な deny 系バグは複数確認できた。#55728 のほか、
  agent レベルの deny browser が効かない #54044、`git -C` 等で
  `external_directory` 検査を回避される #59564、symlink バイパス #58638
  がある。API 生成セッションの deny 不履行（#56502 / #56613）は
  修正済み（closed）。

### 「opencode.jsonc の話」と特定してからの深掘り

- #55728 のメンテナコメントで原因が特定されていた。read ツールは
  Location 相対パスで検査し、ルールテキストはリテラル比較されるため、
  `./` 始まりの `resource` は絶対にマッチしない。`./` を外せば
  同じ deny は正しく発動した。
- PR #53365 は絶対パス↔相対パスの同種不一致への修正（`read`/`edit` で
  絶対ルールを解決済み絶対パスにもマッチさせる）だが、`./` ケースは
  未カバーと明記されている。
- 過去には beta 期の順序問題
  [#43669](https://github.com/anomalyco/opencode/issues/43669)
  （ビルトイン agent ポリシーがユーザールールに勝つ）もあったが、
  2026-10-20 にクローズ済みである。

### 絶対パス指定と Windows パス表記の追試（同日追記）

- 雑談の続きで、絶対パスで `resource` を書き `service restart` で
  再起動しても deny が効かなかったという実体験が加わった。
  職場の Windows PC ではパス区切りのスラッシュも原因の一つの
  可能性があるという指摘もあった。
- 絶対パス系は [anomalyco/opencode#50241](https://github.com/anomalyco/opencode/issues/50241)
  （closed）で実証済みの構造である。Plan agent の allow ルールは
  絶対パス（`/home/qasha/.opencode/plan/*`）に解決されているのに、
  実際の検査ではプロジェクト内パスが Location 相対形式
  （`.opencode/plan/...`）で渡るため、絶対パスのルールはマッチせず
  先頭の deny が勝つ。 deny/allow が絶対パスで機能しないのは
  優先度や反映タイミングの問題ではなくマッチング形式の不一致であり、
  `service restart` で設定を読み込み直してもマッチングロジック自体は
  変わらないため効かない。本件の修正 PR #53365 は open のまま。
- Windows 系は [anomalyco/opencode#36681](https://github.com/anomalyco/opencode/issues/36681)
  （closed）に実験記録がある。要点は次の通り。
  - 報告者の実験では `C:\Users\...`（バックスラッシュ）形式は機能し、
    `/c/Users/` 形式は機能しなかった
  - 実行時の検査パスは「正規化 + glob 付き形式」
    （例: `C:\\Projects\\AI\\lm-studio-assistant\\*`）に変換されるため、
    設定に書いたリテラル文字列とは文字列一致しない
    （該当 issue の m13v による分析コメント）
  - `external_directory` の設定がホーム側設定と累積されず、
    `~/.config/opencode/` 側の allow が静かに落ちる観測もあった
    （当時のドキュメント「merged together, not replaced」に矛盾）
  - `touch` や `glob` が `external_directory` の判定に従わない
    別系統の穴も観測されていた
- #36681 のクローズは 2026-10-11 だが、理由は「V1 起点の issue を
  V2 開発集中のため一括クローズする」トリアイジ（v1-issue-triage）であり、
  修正確認ではない。V2 現行で同構造がどこまで残っているかは
  #53365 が open の時点では楽観できない。
- なお Windows の公式保証は「バックスラッシュはスラッシュに正規化され、
  マッチングは大文字小文字を区別しない」（Permissions ドキュメント）の
  みで、ドライブレターの正規表記や `~`/`$HOME` の Windows での展開規約は
  現行 V2 ドキュメントにも明示されていない（#36681 報告者の指摘と同じ
  状況が続いている）。

### パス正規化パイプラインの実装確認（同日第二追記）

- 「絶対的な互換性を持つパス記載方式はあるか」「連続スラッシュは
  問題にならないか」という問いに対し、v2 ブランチの実装
  （`permission.ts`、`file-access.ts`、`util/wildcard.ts`、
  `config/plugin/agent.ts`、`util/fs-util.ts`）を直接確認した。
- 検査パスの形式は `FileAccess.resolve` が「Location 内か外か」で
  2分化する。Location（または worktree）内は Location 相対形式
  （`slash(path.relative(...))`）、Location 外は絶対形式
  （`slash(absolute)`、ディレクトリは `dir/*`）。単一の書式で両方を
  cover する手段は現行になく、これが #55728/#50241 の構造的根因である。
- 正規化パイプラインの実装事実は次の通り。
  - config 側: `~`/`$HOME` 展開は `external_directory`, `read`, `edit` の
    3 action のみ（`expandHome`）。`~/`, `$HOME/`, `$HOME\` は
    `path.join(home, ...)` に展開される。それ以外の形式
    （`./` 付き、`/c/` 形式、絶対パス直書き）は正規化されず
    リテラルのまま比較される。shell 系 action は生テキストのまま。
  - ツール入力側: `FSUtil.windowsPath`（win32 のみ）が `/c:`、`/c/`、
    `/cygdrive/c/`、`/mnt/c/`（WSL）を `C:/` に変換し、`path.resolve()` が
    連続セパレータを畳む。config 側にはこの変換が適用されない。
  - マッチ（`Wildcard.match`）: 両辺を `replaceAll("\\", "/")` した後、
    `*` を `.*`（`s` フラグで `/` を含む全文字）、`?` を `.` に変換し
    `^...$` 完全一致。win32 のみ `i` フラグで大文字小文字を無視。
- 連続スラッシュ（`//`）の実態は「縮約しない」ことと両辺の非対称性に
  ある。`Wildcard.match` は `//` を `/` に畳まないため、UNC
  （`\\server\share`）は `//server/share` として検査され、設定にも
  `//server/share/*` と2個で書く必要がある。ツール入力側の `//` は
  `path.resolve()` で畳まるが、config 側は畳まれない。
- #36681 のログで `C:\\\\Projects\\\\...` と4重に見えたのは JSON
  エスケープの表示であり、実比較は1回のスラッシュ化のみ。
  「バックスラッシュがひどい」のは主に表示上の問題で、比較上は
  `C:\` と `C:/` が等価（両辺スラッシュ化）である。本当の地雷は
  「config 側は正規化されないまま比較される」点にある。
- なお `permission.ts` では、deny 判定は saved approval（Allow always の
  保存ルール）を含まない先読み評価で行われており、「configured deny が
  saved approval に勝つ」設計は実装されている。deny が効かない場合は
  優先度ではなくマッチ失敗と切り分けられる。

## 得られた気づき

- パーミッション系の静かな失敗（quiet failure）はセキュリティ上
  最悪の失敗モードである。deny が「マッチしなかった」だけで黙って
  allow 相当に落ちる。no match の既定が `ask` のアクションでは
  プロンプトで止まるため気づけるが、`external_directory` は既定が
  allow なので、パス表記を誤った deny は完全に不可視になる。
- `resource` の正規化は設定ロード時に一部しか行われない。公式に
  保証されているのは `external_directory` / `read` / `edit` での
  `~`, `~/`, `$HOME`, `$HOME/` の先頭展開のみで、`./` の除去や
  絶対・相対の相互正規化は現行実装にない（#53365 が対応中）。
- 実務上のワークアラウンドは次の通り。
  - `read`/`edit` の `resource` は `./` なしの Location 相対パスで書く
  - ホーム配下は `~/` または `$HOME/` 形式を使う
  - 絶対パス直書きは相対検査にマッチしない現状リスクを避ける
  - 記述後に実プロンプトで deny が発動するか確認する。スキーマ検証は
    この種のミスマッチを検出しない
- 「後出し」の時間的側面（実行中セッションへの設定反映）については、
  client-server アーキテクチャでバックグラウンドサービスが permissions を
  保持するため、設定変更反映にはサービスの再読み込みが絡む場面がある
  （#43669 の再現手順に `service restart` を含む）。ただし本件を直接的に
  示す open issue は確認できず、未検証事項として区別して記録する。
- 伝聞をそのまま受け取らず一次情報で裏を取る過程で、リポジトリ移転
  （`sst` → `anomalyco`）という前提の変化も捕捉できた。古い参照先で
  検索すると「リソースが存在しない」になり、バグの有無の判断自体が
  できなくなる。

## フォローアップ

- [ ] PR #53365 のマージ状況と、`./` ケースを含む追加修正の追跡
- [ ] ローカル v2.2.1 での実再現テスト（`./` 付き deny の無視、
      `~/` 展開の発動）—— 読み取り専用の本調査では実施せず、
      テスト用プロジェクトで行う
- [ ] 自身の `opencode.jsonc` に書いた deny ルールのパス表記監査
      （`./` 付き、絶対パス直書きの混入確認）
- [ ] 職場 Windows PC での再試行。Location 相対（`./` なし）で deny が
      発動するか、`C:\` バックスラッシュ形式と `C:/` スラッシュ形式の
      差分を確認する。現行 V2 で #36681 系がどこまで残存するかの検証も兼ねる
- [ ] UNC パス（`//server/share/*` 形式）での external_directory の
      発動確認。実装上は `//` 縮約なしのため2個書きが必須のはずだが、
      実ドキュメントに明示がなく未検証である

## 参照

- [OpenCode V2 Permissions ドキュメント](https://opencode.ai/v2/docs/permissions)
- [OpenCode V2 Config ドキュメント](https://opencode.ai/v2/docs/config)
- [anomalyco/opencode#55728: permissions: deny rule for read tool is ignored](https://github.com/anomalyco/opencode/issues/55728)
- [anomalyco/opencode PR #53365: fix(core): match absolute permission rules for relative paths](https://github.com/anomalyco/opencode/pull/53365)
- [anomalyco/opencode#43669: Config permissions/agents overrides cannot override built-in agent policies](https://github.com/anomalyco/opencode/issues/43669)
- [anomalyco/opencode#54044: agents: browser tools exposed despite agent-level "deny browser"](https://github.com/anomalyco/opencode/issues/54044)
- [anomalyco/opencode#59564: permissions: external_directory check misses directory flags](https://github.com/anomalyco/opencode/issues/59564)
- [anomalyco/opencode#50241: plan: plan-mode guard denies writes to configured plan directory](https://github.com/anomalyco/opencode/issues/50241)
- [anomalyco/opencode#36681: Windows path references and permissions on external directory path not working](https://github.com/anomalyco/opencode/issues/36681)
- [file-access.ts（v2）: 検査パスの相対/絶対2分化と `slash` 変換](https://github.com/anomalyco/opencode/blob/v2/packages/core/src/file-access.ts)
- [wildcard.ts（v2）: `Wildcard.match` のスラッシュ化と `*`/`?` 変換](https://github.com/anomalyco/opencode/blob/v2/packages/core/src/util/wildcard.ts)
- [config/plugin/agent.ts（v2）: `expandHome` とパス系 action の限定展開](https://github.com/anomalyco/opencode/blob/v2/packages/core/src/config/plugin/agent.ts)
- [util/fs-util.ts（v2）: `windowsPath` の `/c/`・WSL 形式変換](https://github.com/anomalyco/opencode/blob/v2/packages/util/src/fs-util.ts)
- [permission.ts（v2）: deny 先読み評価と saved approval の連結順](https://github.com/anomalyco/opencode/blob/v2/packages/core/src/permission.ts)
- [同リポジトリ内の関連記録: VS Code の設定ファイルにおけるパスと変数の分かりづらさ](2026-10-30-vscode-path-notation.md)
