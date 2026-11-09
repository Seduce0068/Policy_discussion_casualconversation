---
type: Report
title: "オープン教材・Markdown・GitHub・AI家庭教師による教育DX構想"
description: "既存標準とクラウドサービスを活用し、構造化教材を制作・配布・再利用してAI家庭教師に接続する構想の調査・設計レポート"
tags:
  - education
  - open-education
  - ai-tutor
  - interoperability
  - markdown
generated:
  by: "GPT-6"
  at: "2026-11-09T19:08:00+09:00"
status: draft
---

## 1. エグゼクティブサマリー

本構想は、教育DXのために学校専用の巨大なシステムを新規開発するのではなく、既存のクラウドサービス、オープン標準、オープン教材、AI技術を組み合わせ、教材の制作・管理・配布・再利用を効率化するものである。

中心となる考え方は次のとおり。

> 教科書・問題集・授業資料を、特定ベンダーに依存しない構造化データとして管理し、Web・PDF・電子書籍・LMS・AI家庭教師などから共通利用できるようにする。

### 1.1 全体像

```text
学校向け基盤
  Google Workspace for Education / Microsoft 365 Education
  認証・ファイル共有・共同編集・授業運営
                 |
                 v
教材コンテンツ基盤
  Markdown / HTML / YAML / 画像 / 数式
  構造化・バージョン管理・出典・ライセンス
                 |
       +---------+----------+
       |         |          |
       v         v          v
      Web       PDF        EPUB
       |
       +---------+----------+
                 |
       +---------+----------+
       |         |          |
       v         v          v
    AI家庭教師  問題演習   教材検索
```

### 1.2 現状評価

| 項目 | 評価 |
| --- | --- |
| Markdownによる教材管理 | 実用可能 |
| HTML・PDF・EPUBへの展開 | 実用可能 |
| Gitによる教材履歴管理 | 実用可能 |
| Google・Microsoftとの連携 | 実用可能 |
| オープン教科書のAI利用 | 条件付きで実用可能 |
| 問題データとの連携 | 実用可能 |
| AI家庭教師 | 限定領域なら実用可能 |
| 日本全国の教科書の構造化 | 未整備 |
| 全教科・全学校への共通展開 | 未実現 |

### 1.3 結論

技術的なボトルネックはMarkdownやLLMそのものではない。教材の構造化、著作権・ライセンス、学習内容の識別子、問題と解説の対応、既存LMSとの連携、教育現場での操作性が主な課題である。

新しい教育用ファイル形式を発明するより、既存のオープン標準を組み合わせる方が合理的である。

---

## 2. 日本の教育DX・標準化の現状

### 2.1 教育データ標準

文部科学省は教育データ標準を整備している。対象には、主体情報（児童生徒・教職員・学校等）、内容情報（教科・教科書等）、活動情報、ユースケースなどが含まれる。

2026年3月31日には教育データ標準5.1（2025年度版）が公開され、2026年3月27日には初等中等教育のシステム間連携に関する相互運用標準モデル version 6.00が公開された。

これは学校やサービスごとにデータの意味や形式が異なる問題を減らし、交換・蓄積・分析できるようにする取り組みである。

公式資料:

- <https://www.mext.go.jp/a_menu/other/data_00001.htm>

ただし、教育データ標準5.1は教科書本文をMarkdownで保存する規格ではない。教育データの意味・項目・連携と、教材本文のファイル形式は分けて考える必要がある。

### 2.2 デジタル教科書

文部科学省ではデジタルな形態を含む教科書の発行・採択等について検討が続いている。デジタル教科書の制度整備と、Markdownを原本とするオープン教材の普及は別の課題である。

**デジタル教科書が普及することと、教材の機械可読性・再利用性が高まることは同義ではない。**

公式資料:

- <https://www.mext.go.jp/b_menu/shingi/chousa/shotou/206/giji_list/index.htm>

### 2.3 教科書LOD

教科書情報をLinked Open Dataとして扱う取り組みも存在する。

- <https://github.com/jp-textbook/jp-textbook.github.io>
- <https://w3id.org/jp-textbook/>

教科書の書誌情報や内容の識別と、教科書本文全体のオープン化は別である。LODが存在しても、出版社の本文・問題・解答まで自由に取得できるわけではない。

### 2.4 学習指導要領コードと教科書単元の対応

デジタル庁の2026年3月31日付報告には、教科書の編修趣意書PDFから単元を抽出し、学習指導要領コードとの対照リストを作成する支援ツールの試行が記載されている。中学校の主要5教科を対象とした取り組みである。

これは教材を学習指導要領に結び付けるうえで重要な先行例だが、教科書本文をMarkdown化するプロジェクトではない。また、PDFの表構造の不統一などによって抽出精度に限界があり、前処理や人手確認が必要になる。

---

## 3. 教材フォーマットと既存標準

本構想では、すべてを一つの形式に統一する必要はない。本文・メタデータ・問題・システム連携・配布形式を分離する。

### 3.1 Open Knowledge Format（OKF）

Google Cloud PlatformのOpen Knowledge Formatは、MarkdownとYAML front matterを用いた、ベンダー中立の知識表現形式を目指す仕様である。2026年にv0.2が公開されている。

- <https://github.com/GoogleCloudPlatform/open-knowledge-format>
- <https://github.com/GoogleCloudPlatform/knowledge-catalog/blob/main/okf/SPEC.md>
- <https://cloud.google.com/blog/products/data-analytics/how-the-open-knowledge-format-can-improve-data-sharing>

基本構造の例:

```text
knowledge/
├── index.md
├── textbook/
│   ├── mathematics/
│   │   ├── chapter-01.md
│   │   └── chapter-02.md
│   └── science/
│       └── chapter-01.md
├── exercises/
│   ├── algebra-001.md
│   └── algebra-002.md
└── glossary/
    └── index.md
```

Markdownファイルのメタデータ例:

```yaml
---
type: "Textbook Section"
title: "二次関数の最大値と最小値"
description: "二次関数の最大値・最小値の求め方を解説する。"
tags:
  - mathematics
  - quadratic-function
  - high-school
generated:
  by: "Education Content Pipeline"
  at: "2026-11-09T19:08:00+09:00"
status: stable
---
```

OKFは教材の基本パッケージ形式として試験採用する価値がある。ただし、教育業界のデファクトスタンダードではなく、問題の採点方式、学習指導要領との対応、学習順序、学習者の理解度などの教育固有の意味論までは定義していない。

したがって、OKFを基本とし、教育固有のメタデータ拡張を別途定義するのが妥当である。

なお、Google CloudのOpen Knowledge Formatと、Open Knowledge FoundationのOpen Definitionは別物である。

### 3.2 各ファイル形式の役割

| 形式 | 適した用途 | 位置付け |
| --- | --- | --- |
| Markdown | 文章、見出し、リスト、コード、基本的な数式 | 編集・管理の原本 |
| HTML | Web教材、検索、アクセシビリティ | 公開・閲覧形式 |
| PDF | 印刷、固定レイアウト、配布 | 出力形式 |
| EPUB 3 | 電子書籍、端末間の可搬性 | 配布形式 |
| SVG | 図、グラフ、模式図 | 図版 |
| MathML / LaTeX | 数式 | 数式表現 |
| JSON / YAML | 問題データ、正答、メタデータ | 機械可読データ |

EPUB 3.3はW3C Recommendationであり、HTMLなどのWeb技術を基礎にした電子出版物の配布形式として利用できる。

- <https://www.w3.org/TR/epub-33/>

MarkdownからHTML・PDF・EPUBを生成できるようにすることが重要であり、複雑な教材を無理にMarkdownだけで表現する必要はない。

### 3.3 CommonMark・GFM・RFC 7764

Markdownには複数の方言が存在する。RFC 7764はMarkdownそのものの文法を標準化したRFCではなく、Markdownが複数の軽量マークアップ構文のファミリーであることや、`text/markdown`の扱い、
variantの指定などを説明するInformational RFCである。

推奨方針:

- 基本構文: CommonMark
- 表・タスクリストなど: 必要に応じてGFM
- 検査: markdownlint
- 文書の明確性: ISO/IEC Directives Part 2を参考
- 記述スタイル: Google Markdown Style Guideを参考

参照:

- <https://spec.commonmark.org/0.31.2/>
- <https://github.github.com/gfm/>
- <https://www.rfc-editor.org/rfc/rfc7764.html>
- <https://google.github.io/styleguide/docguide/style.html>
- <https://www.iso.org/directives-and-policies.html>

ISO/IEC Directives Part 2は規格文書の構造・文章作成の指針であり、教材用Markdownの規格ではない。明確な記述、用語の統一、構造化という原則を参考にする位置付けである。

### 3.4 Diátaxis

Diátaxisは、文書を利用者の目的から整理するフレームワークである。

| 種類 | 主な目的 | 教育での用途 |
| --- | --- | --- |
| Tutorial | 学ぶ | 教師と進める実習 |
| How-to | 目的を達成する | 操作・手順 |
| Reference | 情報を確認する | 用語・公式・仕様 |
| Explanation | 理解する | 理論・背景・原理 |

公式:

- <https://diataxis.fr/>

教科書の全章を4分類に分割する必要はない。教科書の学年・単元・章・節という学習順序を維持したうえで、補助教材や操作手順などにDiátaxisを利用する。

---

## 4. 教育分野の相互運用標準

1EdTech Consortiumは教育テクノロジーの相互運用標準を開発している。

| 標準 | 役割 | 本構想での用途 |
| --- | --- | --- |
| LTI 1.3 | LMSと外部ツールの接続 | AI家庭教師をLMSに組み込む |
| OneRoster 1.2 | クラス・名簿・成績の交換 | 生徒・授業情報の連携 |
| QTI 3.0 | 問題・テスト・採点結果 | 問題集や小テストの交換 |
| CASE 1.1 | 学習基準・能力指標の識別 | 学習目標との対応 |
| Common Cartridge | 教材・学習コンテンツのパッケージ交換 | LMS間の教材移行 |

公式資料:

- <https://www.1edtech.org/standards/lti>
- <https://www.1edtech.org/standards/oneroster>
- <https://www.1edtech.org/standards/qti>
- <https://www.1edtech.org/standards/case>
- <https://www.1edtech.org/standards/cc>

役割を整理すると次のようになる。

```text
教材本文              Markdown / HTML / EPUB
教材パッケージ        Common Cartridge
問題・テスト          QTI
学習基準・能力        MEXTコード / CASE
クラス・名簿・成績    OneRoster
外部ツール接続        LTI 1.3
```

LTIはAI家庭教師などの外部ツールをLMSに接続する際に有用である。独自のログイン機能や生徒情報連携を一から開発する必要を減らせる。

---

## 5. 既存サービスの活用

### 5.1 Google Workspace for Education

既にGoogle環境がある学校では、次のサービスを活用できる。

| サービス | 用途 |
| --- | --- |
| Google Classroom | 授業・課題管理 |
| Google Docs | 教材の共同編集 |
| Google Drive | 教材ファイル |
| Google Sites | 教材ポータル |
| Google Meet | オンライン授業 |
| Gemini for Education | AI学習支援・教員支援 |

Education Fundamentalsには、適格な教育機関向けに無料で利用できるエディションがある。実際の機能範囲は契約エディション、管理者設定、地域、対象年齢などによって異なる。

公式:

- <https://edu.google.com/intl/ALL_jp/workspace-for-education/>

### 5.2 Microsoft 365 Education

Microsoft環境を既に使っている学校では、次のサービスを活用できる。

| サービス | 用途 |
| --- | --- |
| Teams | 授業・連絡 |
| SharePoint | 教材ポータル |
| OneDrive | 教材ファイル |
| Word | 教材の編集 |
| OneNote Class Notebook | 授業ノート |
| Excel | データ管理 |
| Copilot | AI支援 |

公式:

- <https://www.microsoft.com/education>

Microsoft 365 LTIなどの統合方式・提供状況は更新されるため、導入時に現行の公式ドキュメントを確認する必要がある。旧LTI連携製品の一部は2026年9月に終了しているため、古い導入記事をそのまま採用してはいけない。

### 5.3 GoogleとMicrosoftを二重導入しない

学校が既にGoogle Workspaceを使っているならGoogleを中心にする。Microsoft 365を使っているならMicrosoftを中心にする。両方を同時に導入すると、
アカウント管理・ファイルの二重管理・教職員研修・契約費用が増える可能性がある。

重要なのは、どちらを使っていても教材データを持ち出せることである。

### 5.4 GitHub

GitHubは教材のソースと履歴を管理する用途に向いている。

- Markdown教材の保存
- 変更履歴・差分比較
- 教員間のレビュー
- Pull Requestによる教材改善
- GitHub Actionsによる自動検査
- Web教材のビルド・公開

ただし、GitHub Classroomは2026年8月28日に終了している。新規登録の受付も2026年5月26日に終了しており、現行サービスとして前提にしてはいけない。

GitHubは教材管理基盤として利用できるが、一般教科の生徒にGitを操作させる必要はない。情報系の授業で必要に応じてGitを学習内容にすればよい。

また、学生の氏名・成績・個人情報を公開リポジトリに保存してはいけない。

公式:

- <https://github.com/>
- <https://github.blog/changelog/2026-05-26-github-classroom-sunset/>
- <https://docs.github.com/>

### 5.5 Quarto

QuartoはMarkdownを中心にした複数ファイルから、書籍・教材・技術文書を生成できる。HTML、PDF、Word、EPUBなどへの出力に対応する。

公式:

- <https://quarto.org/>
- <https://quarto.org/docs/books/>

教材ビルド基盤として有力だが、教師が簡単に操作できる統合CMSではない。教師向けGUIとQuartoを組み合わせる構成が考えられる。

---

## 6. オープン教科書とAI家庭教師

### 6.1 現状でも実現可能か

限定した教科・教材であれば、現在でも実用的なAI家庭教師を構築できる。

```text
構造化された教科書
  ├── 章・節
  ├── 用語
  ├── 例題
  └── 図表・数式
          |
          v
     教材検索 / RAG
          |
          v
         LLM
          |
  +-------+-------+-------+
  |       |       |       |
  v       v       v       v
質問応答  ヒント  類題生成  理解度確認
```

ただし、RAGを接続しただけで家庭教師として完成するわけではない。学習者の理解状態を把握し、答えを出しすぎず、必要な知識を段階的に説明する仕組みが必要である。

### 6.2 OpenStaxの実例

OpenStaxはオープン教科書を提供しており、AIを使ったGuided Dialoguesという機能も展開している。特定の教科書の章・節に結び付いた対話型学習機能であり、教科の専門家が設計した学習進行に沿って対話を進める。

- <https://openstax.org/>
- <https://help.openstax.org/s/article/Guided-Dialogues-Overview-Assignment-and-Scoring>

対象教材や利用条件は限定されており、すべての教材・教科で使える汎用AI家庭教師ではない。それでも、教科書とAI対話を結び付ける方向性が実サービスとして実現している例である。

### 6.3 問題データとの連携

問題と教材の対応関係を持たせると、AIは問題を解くために必要な知識を逆引きできる。

```yaml
id: MATH-ALG-001
title: "二次関数の頂点"
unit_id: MATH-QUADRATIC
prerequisites:
  - MATH-LINEAR-FUNCTION
  - MATH-QUADRATIC-EQUATION
textbook_refs:
  - chapter: 5
    section: 2
difficulty: 3
answer_ref: restricted/MATH-ALG-001-answer.md
explanation_ref: explanations/MATH-ALG-001.md
```

正答・解説は問題本文とアクセス権を分ける。QTIなどの既存標準を利用すれば、問題データの移行性も高められる。

### 6.4 家庭教師として必要な機能

| 機能 | 目的 | 実現方法 |
| --- | --- | --- |
| 教材検索 | 授業範囲の確認 | 全文検索・RAG |
| 問題検索 | 適切な問題の出題 | 問題データベース |
| ヒント | 自力で解く支援 | LLM＋解答段階管理 |
| 採点 | 回答の確認 | ルールベース＋LLM |
| 理解度推定 | 次に学ぶ内容を決める | 学習履歴 |
| 弱点分析 | 苦手な単元の発見 | 単元・問題IDの集計 |
| 復習提案 | 忘却を抑える | 学習間隔の管理 |
| 教材引用 | 説明の根拠を明示 | 教材ID・章節ID |

AIが答えを生成するだけではなく、教材・問題・学習履歴を使って学習プロセスそのものを支援することが重要である。

---

## 7. 著作権・ライセンス

### 7.1 無料公開とオープンライセンスは違う

無料で閲覧できる教材でも、自由にコピー・改変・再配布できるとは限らない。

| ライセンス | 主な条件 |
| --- | --- |
| CC0 | 権利を可能な範囲で放棄 |
| CC BY | クレジット表示 |
| CC BY-SA | クレジット表示・同一ライセンスで共有 |
| CC BY-NC | 非営利利用に限定 |
| CC BY-NC-SA | 非営利利用・同一ライセンスで共有 |

Open Definition 2.1では、オープンな知識は誰もが利用・改変・再配布でき、商用を含む目的を不当に制限しないことを求める。NC付きライセンスはこの意味での完全なオープンには該当しない。

- <https://opendefinition.org/od/2.1/en/>

### 7.2 OpenStaxのライセンス

OpenStaxの多くの教材はCC BY-NC-SA 4.0で提供されている。教育目的での非営利利用・改変には有用だが、非営利条件があるため、あらゆる商用サービスで自由に使用できるわけではない。

- <https://openstax.org/subjects>

### 7.3 日本の著作権法

文化庁は生成AIと著作権に関する考え方を公開している。著作権法30条の4は、一定の情報解析等について権利制限を認めるが、利用目的・態様・権利者の利益を不当に害するかなどによって判断が変わる。

検索・解析のための利用と、著作物の創作的表現をそのまま出力させる目的の利用は区別しなければならない。

- <https://www.bunka.go.jp/seisaku/chosakuken/aiandcopyright.html>

著作権法35条には学校その他の教育機関における授業目的の利用に関する権利制限がある。しかし、購入した教科書を自由にWeb公開したり、誰でも利用できるAI家庭教師に投入したりできるという意味ではない。授業の過程で必要な範囲などの条件があり、
公衆送信では補償金制度も関係する。

### 7.4 推奨する教材調達方針

| 教材 | 推奨 |
| --- | --- |
| CC0・パブリックドメイン | 優先採用 |
| CC BY | 優先採用 |
| CC BY-SA | 条件を確認して採用 |
| CC BY-NC系 | 非営利利用の範囲を確認 |
| 出版社の商業教科書 | 明示的な許諾を取得 |
| 自作教材 | 権利者・ライセンスを明確化 |
| 出典不明の教材PDF | 公開・AI利用しない |

文章だけでなく、写真・イラスト・図・問題・解説・解答についても権利関係を個別に確認する必要がある。

**技術的に変換できることと、変換したものを配布・AI利用できることは別問題である。**

---

## 8. 推奨システムアーキテクチャ

### 8.1 全体構成

```text
             学校アカウント
                    |
          +---------+---------+
          |                   |
    Google Workspace    Microsoft 365
          |                   |
          +---------+---------+
                    |
                   SSO
                    |
                    v
              既存LMS / 授業
                    |
          +---------+---------+
          |         |         |
          v         v         v
         教材       課題       AI
          |         |         |
          v         v         v
      教材基盤   問題基盤   AI家庭教師
          |         |         |
          +---------+---------+
                    |
                    v
             構造化コンテンツ
                    |
             Markdown / OKF
                    |
                   Git
                    |
          +---------+---------+
          |         |         |
          v         v         v
         HTML      PDF       EPUB
                    |
                    v
                検索・RAG
```

### 8.2 データの分離

| データ | 保存場所の候補 | 理由 |
| --- | --- | --- |
| 教科書本文 | OKF / Git | 再利用・履歴管理 |
| 授業資料 | OKF / Git | 教師間共有 |
| 問題・正答 | QTI対応DB・教材リポジトリ | 出題・採点 |
| 教材画像・動画 | Drive / オブジェクトストレージ | 大容量データ |
| 生徒名簿 | 学校のSIS / LMS | 個人情報保護 |
| 成績 | LMS / 校務システム | 権限と監査 |
| 学習履歴 | LMS / 学習分析基盤 | 学習支援 |
| AI会話 | 管理されたAIサービス | 保存期間・権限制御 |

生徒の個人情報や成績を教材のGitリポジトリに混在させない。

### 8.3 教師の操作画面

教師にGitやYAMLを直接操作させる必要はない。

```text
新しい授業を作成
        |
        v
教材を書く
        |
        v
プレビュー
        |
        v
AIによる教材チェック
        |
        v
公開
        |
        v
Web / PDF / LMS / AI
```

裏側でMarkdown・OKF・Git・Quarto・検索インデックスが動く。教師向けのGUIを用意し、既存の編集環境を活用して不足する部分だけを薄い連携ソフトウェアで補う。

---

## 9. 教材リポジトリの設計

### 9.1 推奨ディレクトリ構造

```text
education-content/
├── index.md
├── courses/
│   ├── mathematics/
│   │   ├── metadata.yaml
│   │   ├── textbook/
│   │   │   ├── chapter-01/
│   │   │   │   ├── index.md
│   │   │   │   ├── section-01.md
│   │   │   │   └── section-02.md
│   │   │   └── chapter-02/
│   │   ├── examples/
│   │   ├── exercises/
│   │   ├── glossary/
│   │   └── images/
│   └── science/
│       ├── metadata.yaml
│       ├── textbook/
│       └── exercises/
├── teachers/
│   └── lesson-plans/
├── schemas/
│   └── education-metadata.yaml
└── build/
    └── quarto/
```

章・節単位に分割し、教材の差分確認やAI検索を容易にする。

### 9.2 教育メタデータの例

```yaml
---
type: "Textbook Section"
title: "二次関数の最大値と最小値"
description: "二次関数の最大値・最小値を求める方法を学ぶ。"
tags:
  - mathematics
  - quadratic-function

education:
  country: JP
  grade: 10
  subject: mathematics
  curriculum_ids:
    - "MEXT-EXAMPLE-CODE"
  textbook_id: "MATH-2026-001"
  chapter_id: "CH-05"
  section_id: "SEC-02"

source:
  publisher: "Example Publisher"
  edition: "2026"
  page_start: 84
  page_end: 89
  license: "CC-BY-4.0"

generated:
  by: "Content Conversion Pipeline"
  at: "2026-11-09T19:08:00+09:00"

verified:
  status: true
  method: "human-review"
status: stable
---
```

`education`、`curriculum_ids`、`textbook_id`などはOKFの必須フィールドではなく、教育用途の拡張フィールドである。拡張フィールドの意味・型・使用可能値を別途定義しなければ、機関間の互換性を保てない。

### 9.3 問題・解答の分離

問題本文と正答・解説はアクセス権を分ける。フォルダ名だけでなく、リポジトリやストレージのアクセス制御で保護する。

---

## 10. PDFからMarkdownへの変換工程

```text
PDF
 |
 v
テキスト・画像・表・数式の抽出
 |
 v
Markdown / HTML化
 |
 v
章・節・問題IDの付与
 |
 v
学習指導要領コードとの対応
 |
 v
原文との照合
 |
 v
OKFパッケージ化
 |
 v
Web / PDF / AI
```

### 10.1 保持すべき情報

- 元PDFのファイル名・版数
- PDFページ番号
- 元の章・節番号
- 図表の番号とキャプション
- 数式
- 問題番号
- 正答・解説の対応
- 変換方法
- 人間による検証状況
- 著作権・ライセンス情報

### 10.2 OCRだけでは不十分

教科書や問題集には数式・図形・グラフ・段組み・表が多く含まれる。OCRの誤りがそのまま教材データに残ると、AIが誤った知識を根拠に回答する可能性がある。

文章だけでなく、数式・図版・表・問題番号・正答の対応も検証する必要がある。

原PDFと変換後Markdownの双方を保存し、Markdownから元PDFのページへ戻れるようにする。

---

## 11. 導入リスクと対策

| リスク | 問題 | 対策 |
| --- | --- | --- |
| 教師の負担 | Markdown・Gitの学習が必要 | GUIと自動変換 |
| 変換ミス | 数式・図表の誤認識 | 人間による検証 |
| 著作権 | 教材を自由に再利用できない | ライセンス管理 |
| AIの誤回答 | 教材にない内容を断定 | 出典提示・回答制御 |
| 解答の漏洩 | AIが答えを先に提示 | 段階的な解答開示 |
| ベンダーロック | 独自形式に依存 | OKF・HTML・Git |
| 個人情報 | 学習履歴の漏洩 | LMS/SISと分離 |
| アクセシビリティ | 図や数式を読み上げられない | 代替テキスト・MathML等 |
| 教材の陳腐化 | 古い内容が残る | バージョン・失効管理 |
| 標準の乱立 | 拡張メタデータが不統一 | 共通スキーマ管理 |

### 11.1 AIの安全性

最低限、次の制御が必要である。

1. 使用する教材を明示する。
2. 回答に教材ID・章・節を付ける。
3. 教材に根拠がない質問には、そのことを明示する。
4. 解答を段階的に提示する。
5. 教師が教材と回答を確認できる。
6. 学習履歴と個人情報の保存を制御する。

RAGは根拠資料を検索する仕組みであり、正答保証装置ではない。

### 11.2 アクセシビリティ

Markdown化によって意味構造を取り出しやすくなるが、自動的にアクセシブルになるわけではない。図の代替テキスト、数式の読み上げ、字幕、キーボード操作、色だけに依存しない表現なども必要になる。

---

## 12. 実行ロードマップ

### Phase 0: 既存環境の確認

- 学校が使っているGoogle/Microsoft環境を確認
- LMSと認証方式を確認
- 教材の著作権・ライセンスを確認
- 変換対象を一つに絞る

### Phase 1: 教材の構造化

- 1教科・1学年・1章を対象にする
- 原本PDFを保持する
- OKF形式でMarkdown化する
- 章・節・問題に安定したIDを付ける
- 原文との対応情報を保存する
- Quarto等でHTML/PDFを生成する

### Phase 2: 教材データの標準化

- MEXTの学習指導要領コードと対応付ける
- 教材IDと問題IDを整備する
- ライセンス情報を付ける
- 必要ならCASEとの対応付けを行う
- 自動検査を導入する

### Phase 3: AI家庭教師

- 教材を検索インデックス化する
- 教材に基づく質問応答を実装する
- 問題を出題できるようにする
- ヒントを段階的に出す
- 回答に出典を付ける
- 教師向けレビュー機能を付ける

### Phase 4: LMS統合

- LTI 1.3でAI家庭教師を接続する
- 学習者の授業情報を連携する
- 必要な場合のみ成績連携を行う
- 学習履歴・データ保持期間を決める

### Phase 5: 評価と拡張

- 教師の教材制作時間
- 学生の理解度
- 問題の正答率
- AIの引用正確性
- 教材変換の精度
- サービス間の移行可能性
- 総運用コスト

を測定し、他教科・他学年に広げる。

### 12.1 実証実験の評価指標

| 評価項目 | 確認すること |
| --- | --- |
| 変換精度 | 原PDFの文章・数式・図表を正しく復元できたか |
| 検索精度 | 質問に対応する教材を取得できたか |
| 引用精度 | 出典と回答が一致しているか |
| 学習効果 | 導入前後で理解度に差があるか |
| 教師負担 | 教材作成・修正時間が減ったか |
| 可搬性 | 別サービスに教材を移行できるか |
| 費用 | 導入・運用・保守の総費用が妥当か |
| 安全性 | 個人情報・解答・権利情報を適切に保護できるか |

AIの利用回数や質問数だけを成果指標にせず、生徒の理解と教師の作業時間を測定する。

---

## 13. 費用対効果と調達方針

### 13.1 新規開発を最小限にする

| 領域 | 方針 |
| --- | --- |
| 学校アカウント | Google / Microsoftを利用 |
| ファイル共有 | Drive / SharePoint |
| 授業管理 | Classroom / Teams / 既存LMS |
| 教材管理 | OKF / Markdown / Git |
| 文書生成 | Quarto等 |
| 問題データ | QTI対応 |
| AI家庭教師 | 既存LLM/APIを利用 |
| システム連携 | LTI / OneRoster等 |
| 教育データ | MEXT標準に対応 |

独自開発の対象は、教師向け編集UI、教材と学習指導要領コードのマッピング、教材専用検索・AI接続などに絞る。

### 13.2 避けたい投資

- 独自Markdown方言
- 独自教科書ファイル形式
- 独自クラウドストレージ
- 独自チャットシステム
- 独自Git相当システム
- 汎用PDF変換エンジンの新規開発
- 特定LLMに固定された教材データ基盤
- 教材を移行できない専用CMS

### 13.3 調達条件

1. 教材をMarkdown・HTML・PDFなどで一括出力できる。
2. メタデータもJSON/YAML等で取得できる。
3. APIが公開されている。
4. LMSとLTI等で接続できる。
5. 教材の履歴と版数を取得できる。
6. 特定LLMに依存しない。
7. 個人情報・学習履歴の管理方法が明示される。
8. 契約終了時のデータ持ち出し方法が明示される。

ライセンス料だけでなく、教材変換、教職員研修、移行、運用保守、アクセス制御、AI利用料、データ保管、セキュリティ監査まで含めた総保有コスト（TCO）で比較する。

---

## 14. 最終提言

### 14.1 標準・形式

| 領域 | 推奨 |
| --- | --- |
| 基本コンテンツ形式 | Markdown |
| 基本パッケージ | OKF v0.2を試験採用 |
| 文書の出力 | HTML / PDF / EPUB 3 |
| Markdown文法 | CommonMark + 必要なGFM |
| 文書設計 | ISO/IEC Part 2の原則を参考 |
| 文書分類 | Diátaxisを補助的に利用 |
| スタイル | Google Markdown Style Guide |
| 機械検査 | markdownlint |
| 国内の学習目標 | MEXT教育データ標準・学習指導要領コード |
| 問題交換 | QTI 3.0 |
| LMS連携 | LTI 1.3 |
| クラス・成績連携 | OneRoster 1.2 |
| 教材パッケージ | Common Cartridge |

### 14.2 サービス

| 領域 | 推奨 |
| --- | --- |
| 学校の基盤 | 既存のGoogle Workspace / Microsoft 365 |
| 教材の管理 | Git |
| Gitホスティング | GitHub等を選択可能にする |
| 教材ビルド | Quarto等 |
| 授業管理 | 既存LMS |
| AI家庭教師 | 教材を切り替え可能なLLM連携 |
| データ保存 | 学校が管理できる領域 |

Gitはバージョン管理の仕組みであり、GitHubはそのホスティングサービスの一つにすぎない。GitHubへの依存は避ける。

### 14.3 日本で優先すべきこと

1. 教材を機械可読にする。
2. 教材に安定したIDを付ける。
3. 学習指導要領コードと関連付ける。
4. 問題・正答・解説を構造化する。
5. オープンライセンスの教材を増やす。
6. AIが教材を根拠に回答できるようにする。
7. 既存LMSと連携する。

LLMやクラウドサービスが将来変わっても、教育コンテンツを使い続けられることが重要である。

---

## 15. 総合評価

**構想の実現可能性は高い。ただし、教材のライセンス・構造化・教育的評価が条件となる。**

特に有望な部分:

- 既存のGoogle/Microsoft環境を活用する。
- OKF・Markdown・Gitで教材を可搬化する。
- MEXTの学習指導要領コードと教材を対応させる。
- QTI・LTIなど既存の教育標準を利用する。
- オープン教材をAI家庭教師に接続する。

この構想の核心は、AI家庭教師を新しく作ることだけではない。AI家庭教師が使える教材を、最初から構造化された形で整備することにある。

日本では教育データ標準、教科書単元と学習指導要領コードの対応付け、デジタル教科書の制度整備などが進んでいる。一方、教科書本文をMarkdown等のオープンな形式で管理し、Git・LMS・AIへ展開する共通ワークフローは、
まだ一般化していない。

したがって、次に必要なのは新しい国家独自フォーマットではない。

**既存標準を接続し、教師が意識せずとも構造化教材を作成・公開・再利用できる仕組み**である。

---

## 16. 参考資料

### 日本の教育政策・データ標準

- 文部科学省・教育データ標準: <https://www.mext.go.jp/a_menu/other/data_00001.htm>
- 文部科学省・デジタル教科書検討会議: <https://www.mext.go.jp/b_menu/shingi/chousa/shotou/206/giji_list/index.htm>
- デジタル庁: <https://www.digital.go.jp/>
- 教科書LOD: <https://github.com/jp-textbook/jp-textbook.github.io>

### 文書・コンテンツ形式

- Open Knowledge Format: <https://github.com/GoogleCloudPlatform/open-knowledge-format>
- CommonMark: <https://spec.commonmark.org/0.31.2/>
- GitHub Flavored Markdown: <https://github.github.com/gfm/>
- RFC 7764: <https://www.rfc-editor.org/rfc/rfc7764.html>
- Google Markdown Style Guide: <https://google.github.io/styleguide/docguide/style.html>
- ISO/IEC Directives: <https://www.iso.org/directives-and-policies.html>
- Diátaxis: <https://diataxis.fr/>
- EPUB 3.3: <https://www.w3.org/TR/epub-33/>
- Quarto: <https://quarto.org/>

### 教育相互運用標準

- 1EdTech Standards: <https://www.1edtech.org/standards>
- LTI: <https://www.1edtech.org/standards/lti>
- OneRoster: <https://www.1edtech.org/standards/oneroster>
- QTI: <https://www.1edtech.org/standards/qti>
- CASE: <https://www.1edtech.org/standards/case>
- Common Cartridge: <https://www.1edtech.org/standards/cc>

### サービス・実例

- Google Workspace for Education: <https://edu.google.com/intl/ALL_jp/workspace-for-education/>
- Microsoft Education: <https://www.microsoft.com/education>
- GitHub: <https://github.com/>
- OpenStax: <https://openstax.org/>
- OpenStax Guided Dialogues: <https://help.openstax.org/s/article/Guided-Dialogues-Overview-Assignment-and-Scoring>

### 著作権・オープンライセンス

- 文化庁・AIと著作権: <https://www.bunka.go.jp/seisaku/chosakuken/aiandcopyright.html>
- Open Definition 2.1: <https://opendefinition.org/od/2.1/en/>

## 参照

- 原本: `education_dx_open_textbook_ai_tutor_report.md`（GPT-6、2026-11-09 19:08 JST、
  SHA-256: `ce921358b1bea92b43fb83fe550ff789c908da3bdc849891af7d8d83103c4c1e`）
- 本リポジトリへの取り込みに際し、frontmatter・見出し階層・表記（表の整形、URL のリンク化、行長調整）のみをリポジトリ規約に合わせて調整した。本文の内容は変更していない。
