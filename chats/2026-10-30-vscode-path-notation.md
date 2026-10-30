---
type: Chat
title: "VS Code の設定ファイルにおけるパスと変数の分かりづらさ"
description: "VS Code の変数置換がファイル×キー単位で限定される実態を公式ドキュメントで確認し、opencode.jsonc のパス表記問題と同型の「静かな失敗」構造として整理した"
tags:
  - chat
  - vscode
  - config
  - path-notation
  - quiet-failure
generated:
  by: opencode
  at: "2026-10-30T18:25:46+09:00"
status: "draft"
---

## 文脈

- 2026-10-30 の雑談。同日の
  [opencode.jsonc の deny ルールが黙って無視される](2026-10-30-opencode-permission-deny-ignored.md)
  の調査で「設定ファイルのパス表記はどの層で解釈されるかが外見から
  分からない」問題を扱った流れから、「VS Code の設定ファイルのパス記載
  方式も分かりづらい」という話題が続いた。
- 公式の Variables Reference ドキュメントで変数置換の実態を確認し、
  opencode のパーミッション経路と同じ構造の問題であることを整理した。

## 結論

- VS Code の変数置換（`${...}`）は `launch.json` と `tasks.json` が
  主戦場で、`settings.json` では「一部の設定キーのみ」に限られる。
  ドキュメント冒頭が「and for some select settings」と明記しており、
  同じ `${workspaceFolder}` を書いても、キーによって展開されたり
  リテラル文字列のまま扱われたりする。外見からは判別できない。
- ホームディレクトリの書き方が複数あり、保証が異なる。
  - `~` は VS Code コアでは展開されない（shell に渡れば shell が展開
    するだけ）
  - `${userHome}` が保証された唯一のホーム変数
  - `${env:HOME}` は環境変数構文だが settings.json の大半では未対応
- `${workspaceRoot}` は `${workspaceFolder}` への改名で非推奨。
  古い記事からの引用は動かないまま気づきにくい。
- 変数の実値確認は「echo タスクを作って実行する」以外に公式手段がなく、
  静的検証が存在しない。opencode のパーミッションと同型の
  静かな失敨（quiet failure）構造である。

## やり取り

### 変数置換が機能する範囲の確認

- Variables Reference の該当箇所:
  - `launch.json` / `tasks.json` では `command`, `args`, `options` のみが
    置換対象。tasks.json のそれ以外の値は置換されないことが
    「Why aren't variables in tasks.json being resolved?」で明記されている
  - `settings.json` では terminal の `cwd` / `env` / `shell` /
    `shellArgs` など一部のキーのみ。`window.title` のような
    設定固有変数もある
  - 拡張機能の設定は読み取る側の実装しだいで、コアは関与しない
- 置換は2パス処理（全変数評価→置換）で行われ、コマンド変数は他の変数の
  置換結果にアクセスできない。変数間の依存は不可。

### Windows とパス区切り

- ドキュメントは Windows で JSON エスケープ（`\\`）に注意した上で、
  可搬性のために `${pathSeparator}` または `${/}` を推奨している。
- 実態としては `${workspaceFolder}/sub` のスラッシュ書きでも Windows で
  動作することが多く、「推奨構文」と「実際に動く書き方」の乖離が混乱を
  増幅している。この点は opencode の「バックスラッシュ表示のひどさは
  主に JSON エスケープの見た目の問題」という整理と平行的である。

## 得られた気づき

- VS Code と opencode のパス記載問題は、粒度の違いはあれ同型である:
  - VS Code: ファイル×キー単位で変数置換の可否が変わる
  - opencode: パス系3 action のみ `~`/`$HOME` 展開、config 側は無正規化
    でリテラル比較、検査パスが internal/external で 2 分化
- 両者に共通する核心は「書いた文字列がどの層・どのキーで解釈されるかが
  外見から分からず、解釈されなかった場合に警告が出ない」ことである。
  スキーマ検証は `${...}` やパス表記の解釈可否を拾わない。
- 静的検証がない以上、実務は「保証されている構文に絞る」のが最善:
  - VS Code: `~` を書かず `${userHome}` / `${workspaceFolder}` 系に統一、
    怪しければ echo タスクで実値を吐かせる
  - opencode: Location 相対（`./` なし）、ホームは `~/` か `$HOME/`、
    書いたら実プロンプトで発動確認
- 「非推奨改名の残骸」（`${workspaceRoot}`）は、記事やテンプレートの
  コピー元の鮮度を疑う一般的な教訓になる。設定構文はドキュメントの
  一次情報で都度確認する価値がある。

## フォローアップ

- [ ] 職場 Windows PC での VS Code 設定の監査。`~` 直書きと
      `${workspaceRoot}` の残存確認
- [ ] よく使う拡張機能の設定について、変数置換が実際に機能するかの
      実値確認（echo タスクで `${...}` を吐かせる）

## 参照

- [VS Code Variables Reference](https://code.visualstudio.com/docs/reference/variables-reference)
- [同リポジトリ内の関連記録: opencode.jsonc の deny ルールが黙って無視される](2026-10-30-opencode-permission-deny-ignored.md)
