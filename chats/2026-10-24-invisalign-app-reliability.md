---
type: Chat
title: "My Invisalign アプリの「順調です」とカメラ認識をどう見るか"
description: "Virtual Care の判定が何を意味するか、公式IFU・アプリレビュー・Redditの事例から整理した"
tags:
  - chat
  - invisalign
  - health-tech
  - app-review
generated:
  by: opencode
  at: "2026-10-24T13:39:44+09:00"
status: "draft"
---

## 文脈

- 2026-10-24 の雑談。インビザラインの My Invisalign アプリについて、
  「毎回『順調です』しか言わないのではないか」「カメラの認識精度が悪い」
  という不信感が出た。
- 論点は 2 つ。Virtual Care の判定をどこまで信用できるかと、撮影機能の
  不出来が判定にどう影響するか。
- 以下は、公式の取扱説明書・患者ガイド・提供者向けページと、App Store /
  Google Play / Reddit の事例を突き合わせた整理。医学的助言ではない。

## 結論

- アプリは矯正の診断装置ではなく、写真提出・装着管理・遠隔モニタリングの
  フロントエンドである。Virtual Care の「順調です」は臨床的な保証ではない。
- 公式の取扱説明書自体が、臨床意思決定支援（CDS）であり、診断・治療を
  行うものではないと明記している。AI 評価が不正確になり得ること、
  過度な依存が治療遅延につながることも警告している。
- カメラ認識は 3D スキャンではなく、9 枚の写真を撮るときの構図合わせと
  奥歯検出が中心である。公式も奥歯が検出できない場合の「検出オフで続行」
  を用意しており、失敗は想定済みといえる。
- したがって「全く信用ならない」ではなく、「補助信号としては使えるが、
  単独で信用してはいけない」が妥当である。品質はクリニックの運用に
  強く依存する。

## 公式の位置づけ

日本の患者向け取扱説明書（バーチャル・ケア 3.0、2025 年 2 月発行）には、
次のように書かれている。

- 「バーチャル・ケアは歯の特徴を特定し説明するものであり、一般歯科医、
  矯正医、またはその他の医療従事者の助言の代わりとなるものではありません。」
- 「本ツールへの過度の依存や転用は、不正確な評価もしくは治療の遅延に
  つながる恐れがあります。」
- 「AI システムが生成するシミュレーションおよび測定値は、最終的な臨床結果と
  異なる場合があります。」

| 項目 | 公式の説明 | 実務上の意味 |
| --- | --- | --- |
| 位置づけ | CDS。診断・治療は行わない | 判定は診断ではない |
| 出力 | `On track`（順調です）または `Instructions`（指示コメント）の 2 種類 | 中間の「要観察」はない |
| 判定 | 担当医が設定したアライナー空隙の閾値と AI 評価 | 主指標はアライナーの浮き |
| 確認 | AI 評価を訓練された技術者が確認 | 担当医本人とは限らない |
| 通知 | 約 1 時間で自動通知 | 人間の診察ではない |
| 責任 | 矯正治療の決定は担当医の責任 | アプリは責任を負わない |

- 提供者向けページでは「AI-assisted assessment clinically measures aligner
  spaces; Align-trained personnel verify AI results」と説明されている。
- 患者向けハンドブックは「Your doctor is letting you know your treatment is
  on track」と表現するが、提供者向けページでは Align の訓練スタッフが確認すると
  説明されている。表現の差が、ユーザーの不信感につながりやすい。
- 国によっては医療機器として登録されているが、少なくとも CDS としての
  位置づけどおり、診断装置ではない。

## 「順調です」しか来ない理由

- 仕様上、出力は `On track` か `Instructions` の 2 択しかない。
  「問題なし」ではなく「設定した閾値を超えなかった」という消極的な判定で、
  臨床的な異常がないことの証明ではない。
- 公式が説明する主指標はアライナー空隙（aligner spaces）である。
  咬合、歯根、骨、歯周組織、虫歯、清掃状態まで評価するとは書かれていない。
- 写真から見えるアライナーの浮きが中心なので、咬合の変化や見た目の違和感は
  判定に乗りにくい。
- 確認は Align の訓練スタッフで、担当医本人とは限らない。クリニックが
  写真を能動的に見る運用かどうかで、返信の質と速度に差が出る。
- コミュニティでも「ほとんど on track」「AI は無視して人間の返信を待て」
  という運用が共有されている。

コミュニティの具体例:

- 2025-07 の Reddit。26 トレイ中、毎週写真を提出。ほとんどが `On track` で、
  4 回は 2 週間同じトレイを継続。うち 2 回は担当医から 7 日延長の指示、
  残り 2 回は AI の指示以外になかったという。
  [AI virtual care - Invisalign app](https://www.reddit.com/r/Invisalign/comments/1mdzcsv/ai_virtual_care_invisalign_app/)
- 2023-08 の Reddit。「AI のフィードバックは信じるな。担当医は数週間後に
  無視するよう言った。人間の返信を待て」という証言。
  [Invisalign App Virtual Care](https://www.reddit.com/r/Invisalign/comments/15juaom/invisalign_app_virtual_care/)
- 2026-03 の Reddit。26 トレイの遠隔モニタリング中、噛み合わせの異常が
  週次の写真に数か月写っていたが、終了まで指摘されなかったという。
  返信が外部委託されている可能性も疑っている。ただし使用アプリは
  My Invisalign と明記されておらず、Dental Monitoring の可能性がある。
  遠隔モニタリング一般の限界として読むのが妥当。
  [Is it normal for Invisalign providers to miss obvious tracking issues until the end?](https://www.reddit.com/r/Invisalign/comments/1s37m1v/is_it_normal_for_invisalign_providers_to_miss/)
- App Store の日本語レビュー。「コメントは AI 自動送信で、担当医がちゃんと
  チェックしているのか疑問」という指摘がある。
  [My Invisalign（App Store）](https://apps.apple.com/jp/app/my-invisalign/id1509376619)

## カメラ認識の実態

- My Invisalign の Virtual Care は口腔内 3D スキャンではなく、公式患者ガイド
  では 9 枚の写真を撮る。カメラの「認識」は、口の位置合わせと奥歯検出が
  中心で、歯の形状を計測するスキャンではない。
- 公式ハンドブックには、奥歯が検出できないときに「再試行」か
  「検出オフで続行」を選ぶ画面が記載されている。検出失敗は例外ではなく、
  公式に想定された経路である。
- 取扱説明書は、画質が低いこと、AI 評価が不正確であること、指示が不明瞭で
  あることを、不快症状や歯牙・歯周組織の損傷につながる要因として挙げている。
  撮影の失敗は単なる UX の問題ではなく、判定の入力そのものを壊す。
- コミュニティでは、フリーズ、カメラが暗くなる、奥歯が検出されない、
  ピントが合わない、撮影後のトリミングがずれる、送信中に写真が消える、
  といった報告が繰り返されている。Android、特に Pixel で悪いという証言もある。
  [The worst part of Invisalign is...](https://www.reddit.com/r/Invisalign/comments/16ixv3a/the_worst_part_of_invisalign_is/)
  [Invisalign App - Problem w/ taking photos](https://www.reddit.com/r/Invisalign/comments/qqx5il/invisalign_app_problem_w_taking_photos/)
- 公式の推奨端末には Samsung Galaxy シリーズが含まれる。端末が非推奨だから
  失敗する、という単純な話ではない。

対策:

- チークリトラクターまたは Invisalign Lens を使う。公式も奥歯の撮影には
  リトラクターを推奨している。
- 照明は正面から当てる。真上からの照明は影を作りやすい。
- 9 枚すべてを撮る。奥歯を写すときは頬を十分に引く。
- 「検出オフ」は最後の手段にする。使った場合はクリニックに伝える。
- アプリを最新版にする。OS の要件を満たしているか確認する。
  フリーズが続くならキャッシュ削除や再インストールを試す。
- それでも改善しないなら、クリニックに撮影方法の代替や、対面チェックへの
  切り替えを相談する。

## コミュニティの評価

- App Store 日本版は 4.2（1,391 件の評価）、Google Play 日本版は 3.5
  （1.52 万件のレビュー）。2026-10-24 時点のスナップショット。
  [App Store](https://apps.apple.com/jp/app/my-invisalign/id1509376619)
  [Google Play](https://play.google.com/store/apps/details?id=com.aligntech.myinvisalign&hl=ja)
- 低評価の中心は、撮影の不安定さ、セッション切れ、アップロード失敗、
  ログインの煩雑さ、アプリの基本機能の少なさである。
- 「ClinCheck を見る以外に価値がない」「TrayMinder で十分」という声が多い。
  一方で、ClinCheck を見るためにアプリを使う層もいる。
  [Useless Invisalign app?](https://www.reddit.com/r/Invisalign/comments/172ulc9/useless_invisalign_app/)
- Reddit は不満を持つ人が書き込みやすいため、母集団に偏りがある。ただし、
  App Store、Google Play、Reddit で同じ不満が繰り返されており、孤立した
  現象ではない。

## 実務的な使い方

- アプリは「記録と連絡のツール」として使う。`On track` を、治療が順調である
  証明として受け取らない。
- クリニックに次を確認する。
  1. 写真を確認するのは担当医か、Align のスタッフか。
  2. 返信までの標準的な所要時間はどれくらいか。
  3. アライナー空隙の閾値はどう設定しているか。
  4. 次回の対面チェックはいつか。
  5. アプリが `On track` でも違和感がある場合、どう連絡すればよいか。
- 次の場合はアプリの判定を待たずに連絡する。
  - トレイが最後まで沈まない、大きな隙間（ハロー）がある。
  - 噛み合わせが変わった、奥歯が当たらない、顎に痛みがある。
  - アタッチメントが取れた、歯茎が腫れた、出血が続く。
  - 強い痛み、発熱、口内炎の悪化など。
- アプリの指示だけでトレイ交換を早めたり遅らせたりしない。変更は担当医の
  指示で行う。
- 遠隔モニタリングだけで完結させず、定期的な対面チェックを求める。
  特に咬合の変化は写真では評価しにくい。

## 参照

- [インビザライン バーチャル・ケアAI 取扱説明書（日本語 PDF）](https://assets.ctfassets.net/y75jnaifzz4q/6AmBzT7gMYZ6fK8kgJU7ot/8634358caef1cda11f0f2c44484319df/A008149_Rev_D_Instructions_for_use_Virtual_Care_AI_Tech_Model_Patient_JP.pdf)
- [患者ガイド（英語 PDF）](https://assets.ctfassets.net/ekxs4f5jni5l/10Db5YAGThq4xJg9KP8T7u/8350a9364c2b156c139438d7fa2ab271/MKT-0004115-Rev-E-Patient-Handbook-Virtual-Care.pdf)
- [Invisalign Virtual Care AI（提供者向け）](https://www.invisalign.com/provider/align-digital-platform/virtual-care-ai)
- [Align Technology の発表（2022-09-28）](https://investor.aligntech.com/news-releases/news-release-details/align-technologys-next-generation-invisalign-virtual-care-ai)
- [My Invisalign（App Store 日本版）](https://apps.apple.com/jp/app/my-invisalign/id1509376619)
- [My Invisalign（Google Play 日本版）](https://play.google.com/store/apps/details?id=com.aligntech.myinvisalign&hl=ja)
