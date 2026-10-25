---
type: Chat
title: "Meta のパーソナルエージェント Muse を入れてみた"
description: "日本で提供開始された Muse の機能・課金・隔離アーキテクチャと、既知のセキュリティ事象から初期設定の確認点を整理した"
tags:
  - chat
  - ai-agent
  - meta
  - privacy
  - security
generated:
  by: opencode
  at: "2026-10-25T19:15:03+09:00"
status: "draft"
---

## 文脈

- 2026-10-25 の雑談。Meta のパーソナルエージェント「Muse」を入れたという報告から、
  何ができるのか、どこまで任せてよいのか、入れた直後に何を確認すべきかを整理した。
- 以下は Meta の公式ヘルプ・発表と、報道・セキュリティ研究の公開情報を突き合わせた
  技術的な所見であり、個別の利用を推奨・保証するものではない。

## 結論

- Muse は単なるチャットアプリではなく、クラウド上の専用 VM でブラウザ操作・
  フォーム入力・予約・購入まで実行する汎用エージェントである。権限の総量が大きい。
- 統制の設計は妥当である。認証情報はモデルから隔離され、重要操作の承認は
  モデルの判断の外側で強制される。プロンプトインジェクション対策も多層化されている。
- ただし「Meta もデータを見られない」わけではない。Confidential VM はまだ提供前で、
  モデル学習への利用は既定で有効になっている。オプトアウトが必要である。
- 初期運用は「読み取り系コネクタのみ」「送信・支払いは都度承認」「学習オプトアウト」
  で様子を見るのが妥当。無料枠でトークン消費を把握してから課金を判断する。

## できること

| 領域 | 内容 |
| --- | --- |
| 仕事効率化 | 予定調整、予約、リマインダー、連絡のフォローアップ |
| 家計 | 支出記録、サブスク整理、請求リマインド、解約前の確認 |
| 健康 | 運動・食事プラン、カロリー記録、睡眠モニタリング |
| 買い物 | 価格比較、値下がり追跡、承認を前提とした購入代行 |
| コネクタ | Gmail、Google カレンダー、Instagram、決済・旅行系サービス |

- 日本では Gmail、Google カレンダー、Instagram に加え、Stripe、Shopify、
  Trip.com、Agoda、Klook などとの連携が案内されている。マネーフォワード ME、
  チケットぴあなど国内サービスの連携も進められている。
- アプリを閉じてもタスクを継続し、承認が必要な場面で通知する。
- Meta Connect 発表の高度な音声モード、Realtime Avatar、Muse 専用メールアドレスは、
  日本での提供状況が別途案内される段階である。

## 課金

| プラン | 月額 | 週間トークン |
| --- | --- | --- |
| 無料 | 0 円 | 上限あり（具体的な量は非公開） |
| Power | 2,799 円（$20） | 5 億 |
| Maximum | 13,990 円（$100） | 30 億 |

- 無料枠の上限に達した場合、サブスクリプション、追加トークンの単発購入、
  または翌週の枠更新まで待つ、の 3 択になる。
- 追加購入したトークンに有効期限はない。一方、サブスクリプションの週次枠の
  未使用分は翌週に繰り越されない。
- サブスクリプションは購入時と同じプラットフォーム・同じアカウントでしか
  管理できない。iOS と Android、Web をまたいだ変更はできない。

## アーキテクチャと統制

- Muse Secure VM: 利用者ごとに隔離されたクラウド上の仮想マシン。会話、ファイル、
  エージェントの記憶、接続サービスの認証情報をここに保持する。継続的にバックアップされる。
- Secure Credentials Store: 資格情報は VM 内の別コンテナに置かれ、AI モデルは
  パスワードを参照できない。認可された操作の実行時に、Sentinel がネットワーク境界で
  実トークンに差し替える。
- Sentinel: Muse とは独立したエージェントで、外部への通信を管理する。
  Muse 側から上書きできない。重要操作の承認可否はモデルの判断だけに依存せず、
  決定論的なチェックで強制される。
- プロンプトインジェクション対策: モデル学習、複数の検出分類器、外部入力への
  untrusted ラベル付与、人間による承認、ランタイムセルの権限制限を重ねる。
  Meta は Muse の bug bounty を公開し、プロンプトインジェクションに最大 13 万ドル、
  全体で最大 30 万ドルを提示している。
- 前提の限界: VM 内データは、サービス提供・保守・セキュリティのために Meta が
  アクセスできる。Confidential VM は設計・外部監査中で、2026 年内の提供予定とされる。
  会話のモデル学習利用は既定で有効であり、オプトアウトが必要である。

## 既知のセキュリティ事象

- macOS の 0-day（2026-09、Patrick Wardle が発見）: 同一端末上の任意のプロセスが
  未公開設定を書き換え、文字起こしの送信先を攻撃者のサーバーに変更できた。
  認証トークンが奪われ、Muse アカウント全体を操作できる。Meta は 1 日で修正し、
  ローカル権限昇格であってリモート攻撃ではないと説明した。発見者は ClickFix 型の
  誘導でリモートから成立すると指摘している。
- 共有リンクの列挙（2026-10、Claroc）: 共有 URL の設計が推測可能で、6,800 件の
  リンクを列挙できた。業務秘密、ローン申請、旅程、勤務表などが閲覧可能だった。
  Meta は共有機能を一時停止して見直しに入った。
- 企業側の可視性: Sentinel がトークンを差し替える設計のため、OAuth の付与履歴だけを
  見る統制では、利用者が持ち込んだ API キーによる接続を検出できない。監査には
  コネクタの活動ログと接続先サービス側の監査ログが必要になる。
- いずれも修正または機能停止済みである。ただし、権限の大きさに対して統制と
  運用が成熟途上であることを示す事象として扱う。

## 入れた直後に確認すること

1. 設定で「モデルの学習に利用させない」オプトアウトを有効にする。既定は学習許容である。
2. コネクタは読み取り中心から始める。Gmail の送信、カレンダーの書き込み、
   Instagram の DM・投稿は、動作を確認してから許可する。
3. メール送信、購入、予約、データ共有などの重要操作は、毎回承認する設定にする。
4. 共有リンク機能は使わない。再開後も、機密情報を含む会話では使わない。
5. Muse に渡す資格情報は専用のものにする。既存のパスワードを使い回さない。
6. アクティビティログを定期的に確認する。心当たりのない操作があれば接続を解除する。
7. macOS 版を使う場合、ClickFix 型の「ターミナルに貼り付け」を誘導する手順に注意する。
8. Android 版はバックグラウンドで位置情報を使う場合があり、電池消費に影響し得る。

## 評価

- 「人に頼む感覚でタスクを任せられる」という説明は誇張ではない。ただし、
  その便利さは個人アカウントの権限を広く渡すことと表裏一体である。
- 最初から全コネクタを接続せず、1〜2 週間は限定した権限で挙動を観察するのが現実的である。
- 課金は、無料枠でどの操作がどれだけトークンを消費するかを把握してから判断する。
  週次枠は繰り越されないため、月末ではなく週単位で使用量を見る。
- 他のエージェント（ChatGPT のエージェント機能、Gemini、Claude など）との比較は、
  権限モデル、監査可能性、データの扱いを同じ軸に揃えて別途行う。

## 参照

- [Meta、パーソナル AI エージェント「Muse」を日本で提供開始](https://about.fb.com/ja/news/2026/10/muse-personal-ai-agent-launches-in-japan/)
- [Muse: Meta's personal AI agent](https://ai.meta.com/muse/)
- [Muse ヘルプセンター](https://www.meta.com/ja-jp/help/artificial-intelligence/1303670544995562/)
- [How Muse handles your privacy, safety and security](https://www.meta.com/help/superintelligence/1047255454427887/)
- [How We Built Safety Into Muse（Meta AI Research）](https://research.meta.ai/blog/security-and-safety-for-ai-agents-our-approach-with-muse)
- [Muse サブスクリプションについて](https://www.meta.com/ja-jp/help/subscriptions/1021145227643680/)
- [Muse from Meta（App Store 日本版）](https://apps.apple.com/jp/app/muse-from-meta/id6760173601)
- [Muse from Meta（Google Play 日本版）](https://play.google.com/store/apps/details?hl=ja&id=com.facebook.aura)
- [PC Watch: 人に頼む感覚でメールも予約も丸投げ](https://pc.watch.impress.co.jp/docs/news/2150743.html)
- [Ars Technica: Muse の macOS 0-day](https://arstechnica.com/security/2026/09/muse-metas-extraordinarily-privileged-ai-assistant-has-a-serious-0-day/)
- [Ars Technica: 共有リンクから個人情報が露出](https://arstechnica.com/security/2026/10/meta-muse-exposes-users-personal-and-business-secrets-when-they-share-conversations/)
- [The Verge: AI agent makers are promising privacy](https://www.theverge.com/ai-artificial-intelligence/1009051/privacy-ai-agent-promises-openai-meta-muse-dots)
- [VentureBeat: Muse の 0-day 修正後も監査の可視性が課題](https://venturebeat.com/security/meta-patched-muses-zero-day-but-security-teams-still-lack-visibility-into-what-the-agent-can-access)
