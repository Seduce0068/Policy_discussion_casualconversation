---
type: Report
title: "PC と Bluetooth イヤホンの接続不安定・ユーザビリティ改善調査"
description: "途切れ・片耳のみ接続・端末の取り合い・スマホ再生中の PC 接続不能について、ローカル実測と一次情報で原因を特定し優先度付きの改善手順を整理した報告"
tags:
  - bluetooth
  - linux
  - pipewire
  - usability
generated:
  by: lead
  at: "2026-11-28T19:10:21+09:00"
status: "draft"
---

## 概要

結論を先に述べる。

- 最大の原因は PC 側の設定ではなく、**これまで「途切れ対策」として入れられた modprobe.d 設定が
  すべて無効なパラメータ名で、一度も効いていなかった**ことである。正しくは `enable_autosuspend` で、
  実測値は `Y`（有効）のままだった。
- アダプタは RTL8761BU ファミリ推定だが upstream の汎用バインドで動作しており、
  カーネルやファームウェアの更新より先に USB 電源管理の修正を試すのが合理的である。
- 途切れの直接証拠は PipeWire の `Missing completion reports for packet` である。
  TX 完了報告が 1 秒以上欠落したときの警告で、抑制カウンタが 1432 に達した回もある。
  末尾の `Bluetooth adapter firmware bug?` は**仮説であり診断ではない**。
- 片耳だけ接続される症状は、片側だけをケースから取り出した場合は**仕様**である。
  ただしスマホで時間経過しても解消しない場合は、Classic Audio から LE Audio への
  切り替え後に片側だけ接続される公式の既知問題が該当し得る。
  スマホ側の削除（forget）と再ペアリングが第一候補である。
- 端末の取り合いとスマホ再生中の PC 接続不能は、WF-1000XM6 のマルチポイント設定で解決できる。
  Sony 公式は「2 台の機器と同時に接続」「再生機器の切り替え」「接続中のリストから機器を固定」を
  用意している。ただし Linux PC は公式の対応対象外（Windows 11 / macOS のみ）である。
- 3 台目の機器が接続されると、最後に再生していた機器以外が切断される仕様である。
  ペアリング済みの他デバイスが接続に関与していないかを確認する必要がある。
- 本調査は読み取りのみで行い、**設定変更は一切実行していない**。適用は利用者の判断で行う。

調査は `swarm-research` の R1〜R6 を完全無制限プロファイルで実施し、R6 で親が出典・妥当性・
敵対的検証の結果を集約した。同様のローカル改善調査として
[自宅常駐サーバのセキュリティ監査](2026-11-04-home-server-security-audit.md) がある。

## 調査範囲と方法

### 対象と症状

- 対象ホスト: Ubuntu 26.04.1 LTS、kernel 7.0.0-44-generic、BlueZ 5.85、
  PipeWire / WirePlumber 1.6.2。
- 対象デバイス: Sony WF-1000XM6（ファームウェア名 `WF-1000XM6`）、
  Shokz OpenRun Pro、Android スマートフォン、MX Master 4。
- 症状: (1) 再生が途切れる、(2) 接続直後に左右のどちらかしか繋がらない、
  (3) スマホと同時接続すると取り合いになる、
  (4) スマホ再生中に PC へ接続できず PC 側 Bluetooth の無効化・有効化が必要。

### 情報源と並列調査

- 情報源: ローカルの `journalctl` / sysfs / `bluetoothctl` / `wpctl` / `/etc`、
  Sony 公式ヘルプガイドとサポート記事、BlueZ 公式 issue とソース、
  PipeWire / WirePlumber 公式ドキュメントとソース、USB-IF と kernel.org の白書。
- 並列調査: R2 一次調査 5 体、R3 深掘り 1 体、R4 敵対的検証 2 体、R5 補足 1 体。
  R1 の論点分解と R6 の集約は親が実施した。
- 使用モデル: 子エージェントは opencode-go の登録済み無料モデル
  `hy4-flash-free`、`beam-free`、`hungerg-free`、`iceman-contributor-free` を併用した。
  選択根拠は大容量・構造化出力・推論特性と調査役割の適合である。
  従量の `deepseek-v4.1-flash` は使わず、追加課金は発生していない。
- 上限: 完全無制限プロファイル（同時起動上限 80、深度上限 80 を環境能力値として設定）。
  実際の同時起動数は最大 5 体、深度は 1、累計起動数は 11 体。停止理由はタスク枯渇である。

### 除外事項

- 実機へのペアリング操作、イヤホンのリセット・初期化、設定ファイルの書き換え、
  サービス再起動、ファームウェア更新は行っていない。
- 音質の主観評価、RF スペクトラム実測、`btmon` トレースの取得は行っていない。
  したがって「どの対策が効くか」の実測比較は未実施である。

## 確認済みの現状（Runtime Proof）

### 環境

| 項目 | 確認結果 | 根拠 |
| --- | --- | --- |
| OS | Ubuntu 26.04.1 LTS | `lsb_release -a` |
| カーネル | 7.0.0-44-generic（Ubuntu 7.0.0-44.44, 7.0.14） | `uname -r` / `/proc/version_signature` |
| BlueZ | 5.85 | `bluetoothctl --version` |
| オーディオ | PipeWire 1.6.2 + WirePlumber 1.6.2 | `wpctl status` |
| アダプタ | VID 0411 / PID 0374、Realtek Bluetooth Radio（RTL8761BU ファミリ推定）、USB 2.0 | sysfs / `lsusb -t` / `hciconfig -a` |
| HCI | 5.1 (0xa)、ACL MTU 1021:6、SCO MTU 255:12 | `hciconfig -a` |
| 現在の出力 | UAB-80（USB DAC）が既定シンク。Bluetooth デバイスは未接続 | `wpctl status` / `pactl info` |

### ログ証拠

直近 3 日のログから、以下を確認した。

1. `src/profile.c:record_cb() Unable to get Hands-Free Voice gateway SDP record: Host is down`
   が WF-1000XM6 に対して反復（11/26、11/27、11/28）。
2. `src/profile.c:ext_io_disconnected() ... getpeername: Transport endpoint is not connected (107)`
   が 11/27 01:32、22:05、11/28 17:58 に発生。
3. 11/28 17:58:48 に HFP 切断と A2DP の `sep3/fd0 terminated unexpectedly` が同一秒で発生。
4. `spa.bluez5.sink.media: Missing completion reports for packet (N suppressed)`
   が 11/27 22:03〜23:20 に多数。抑制カウンタは 598 / 1340 / 1432 に達した。
5. 直後に `pw.node: (bluez_output...) running -> error (Received error event)` と
   `spa.bluez5: Failure in Bluetooth audio transport .../fd0`。
6. `btd_adv_monitor_power_down() Unexpected NULL ...` と `Failed to set mode: Busy (0x0a)`
   が Bluetooth の電源投入・切断時に反復。
7. WirePlumber の `bluetooth.autoswitch-to-headset-profile` は `false` で保存済み
   （既定は `true`）。つまり HFP 自動切替は PC 側で既に無効化されている。

### 既存設定の不具合

ブートログに次が記録されている。

```text
btusb: unknown parameter 'autosuspend' ignored
btusb: unknown parameter 'disable_autosuspend' ignored
```

出所を確認すると、次の 2 ファイルが存在した。

```text
/etc/modprobe.d/btusb_stability.conf:1:options btusb disable_autosuspend=1
/etc/modprobe.d/btusb_disable_autosuspend.conf:1:options btusb autosuspend=0
```

このカーネルの `btusb` が公開するパラメータは `disable_scofix`、`enable_autosuspend`、
`force_scofix`、`reset` の 4 つだけで、`enable_autosuspend` の実測値は `Y` である。
したがって上記 2 ファイルは**一度も効果を発揮していない**。
USB アダプタの `power/control=auto`、`autosuspend_delay_ms=2000` もこの事実と整合する。

## 症状別の原因分析

### 途切れ

PipeWire の `bt-latency.h` は、送信済みパケットの完了報告が 1 秒以上欠落すると
`Missing completion reports for packet` を警告する。主に LE Audio の ISO（CIS/BIS）
パケットを対象とするが、A2DP の media sink ともコードを共有するため A2DP でも発生する。
抑制カウンタは 5 分間隔・バースト 8 のレート制限による未出力回数で、大きい値ほど
継続的な完了報告欠落を示す。

PipeWire のソースは「この 1 秒タイムアウトは ISO キューのフラッシュ待ちで固まることを防ぐだけで、
カーネル 6.18 以降が未完了バッファを溜めた結果、送信を停止することは防げない」と述べる。
したがって根本原因は PipeWire ではなく、アダプタ・USB・カーネル側にある。
PipeWire 自身も Intel AX210 と Realtek 8761CU を既知の事例として挙げるが、
末尾の `firmware bug?` は仮説であり、この個体の原因を確定するものではない。

BlueZ 側の HFP SDP 失敗は A2DP を直接切断しない（A2DP は固定 PSM の L2CAP を使い
SDP ルックアップを要さない）。HFP 失敗がデバイス接続全体のキャンセルに波及する事例は
報告されているが、ローカルで観測された同一秒の発生は相関にすぎず因果は未確認である。

補足として、このアダプタは upstream の `btusb.c` に明示エントリがなく、
汎用 Bluetooth USB クラスでバインドされる。そのため Realtek 固有の
`btrtl_download_firmware()` が呼ばれず、`rtl_bt/rtl8761bu_fw.bin` の読み込みログが出ない。
これはファームウェア未搭載を意味するのではなく、ROM ファームウェアで動作していることを
示唆する。`BROKEN_EXT_SCAN` quirk は commit `ca0583c24661` で VID/PID `0bda:a728` に
限定されており、この `0411:0374` には適用されない。
したがってカーネルや `linux-firmware` の更新だけでは改善しにくく、
まず runtime PM 設定を試すのが合理的である。

### 片耳のみ接続

Sony 公式によれば、充電ケースから片側だけを取り出した場合はその側だけに電源が入り、
もう片側を装着すると自動的に左右が接続される。両方を取り出した場合は両側がオンになり
左右接続が確立する。片耳だけ音が出る状態はこの仕様であり、
Classic Audio 接続時は片耳でも左右チャンネルを合成した音声が聞こえる。

利用者から追加された観察（2026-11-28 19:45 頃）:

- PC では一定時間待てば両側から音が出るようになるが、スマホでは時間が経っても解消しない。
- ケースから 1 個目を取り出して装着している間に 2 個目を取り出して装着すると、
  「片方だけ装着」と判定されているのではないかという仮説。
- 実際スマホの Bluetooth 接続表示に片方しか表示されることがある。

この観察について次が分かった。

1. 公式の片耳ページには装着順序や時間差の但し書きがない。したがって
   「装着途中の片側判定が永続する」という仮説は**未確認**である。
   TWS の実装には「最初に取り出した側が固定 primary になりロールを切り替えない」
   方式があり、技術的にあり得ないわけではない。ただし WF-1000XM6 がその方式を
   採用しているかは公式資料から断定できない。
2. Sony は「Classic Audio から LE Audio へ切り替えた後に片側だけ接続される」症状を
   公式に認識しており、スマホの Bluetooth 設定からイヤホンを削除（forget）して
   再ペアリングするよう案内している。スマホ側で時間が経っても解消しない場合は、
   この既知問題が最も有力である。
3. スマホに `LE_ WF-1000XM6` だけが表示される場合は、すぐ選択せず数秒待つと
   プレフィックスなしの `WF-1000XM6` が表示される。両方表示された場合は
   `WF-1000XM6` を選ぶ。これは Classic / LE Audio の別エントリであり、
   左右分割を示す公式記述はない。
4. PC では解消／スマホでは不解消という差そのものは公式に説明されていない。
   BlueZ の再接続は 7 回・1,2,4,8,16,32,64 秒の指数バックオフで
   link loss と host suspend 時に限られる一方、Android の PhonePolicy は
   Bluetooth ON 時に直近 A2DP 端末へ autoConnect し、6 秒後に未接続プロファイルを試す。
   この差が一因になり得るが、WF-1000XM6 での因果は未確認である。
   WF-1000XM5 で Linux のみ片側再接続するコミュニティ報告はあるが、
   XM6 への一般化はできない。

スマホ側の回復手順は次を順に試す。

1. Android の Bluetooth を OFF/ON して `WF-1000XM6` をタップする。
2. Sound Connect の「接続中のデバイス」で再生機器を切り替え／固定する。
3. Classic／LE Audio を切り替えた直後なら、Bluetooth 設定でイヤホンを削除し再ペアリングする。
4. 両側をケースに戻して蓋を閉じ、開け直して取り出す。
5. リセット（ペアリング情報を保持、約 20 秒押し）。
6. 最後の手段として初期化（全ペアリング情報削除、ホスト側も削除して再ペアリング）。

各段階で接続通知音が鳴り、Sound Connect の左右バッテリー表示が両方出るかを確認する。

### 端末の取り合い

WF-1000XM6 は 2 台の同時 Bluetooth 接続に対応し、公式は
「2 台の機器と同時に接続」を Sound Connect の［接続］から有効化する手順を示している。
再生機器の自動切替は「再生機器の切り替え」で制御でき、オフにすると後から再生した機器への
自動切替を止められる。優先端末は「接続中のリストから機器を固定」で固定できる。
ただし、オフになるのは 2 台接続中の再生切替であり、マルチポイント全体や
3 台目接続時の入れ替えまで止まるとは公式から読めない。

3 台目の機器が接続されると、最後に音楽再生していた機器との接続が維持され、
もう一方が切断される。ペアリング済みの Shokz やスマホが意図せず接続に関与すると
「取り合い」として知覚される可能性がある。
また、1 台目が再生停止後も無音信号を送り続ける機種では、ヘッドホンがその機器に留まり
2 台目へ音が切り替わらない。公式対処は「マルチポイントがオンか確認 → 1 台目で再生停止 →
1 台目のアプリを終了」の順である。スマホの通知音・操作音をオフまたは小さくすることも
意図しない切替の抑制策として示されている。

### スマホ再生中の PC 接続

公式の切替手順は「マルチポイントを有効化 → 接続中のデバイスから PC を選択 →
両方が接続中であることを確認 → PC で再生開始 → スマホ側を一時停止」であり、
Bluetooth のトグルは不要である。ヘッドホン操作で再生した場合は、
最後に再生していた機器から再生される。

切替ができない場合の公式対処は、最後に接続していた再生機器との Bluetooth 接続を
切断してから、接続したい側で再度接続を試すことである。
このとき「登録を解除する」「接続を解除」を選ぶと機器登録から削除されるため選択してはならない。
ただし Sony の PC 接続手順は Windows 11 と macOS が対象で、Linux は公式保証外である。

## 改善提案

### P0: 無効な btusb 設定を修正する

最優先で実施する。退避してから正しいパラメータへ置き換える。

```bash
# 1. 現状を退避する（上書きではなくコピー）
mkdir -p ~/.config/bluetooth-backup
sudo cp /etc/modprobe.d/btusb_stability.conf \
       /etc/modprobe.d/btusb_disable_autosuspend.conf \
       ~/.config/bluetooth-backup/

# 2. 無効な 2 ファイルを無効化し、正しいパラメータを 1 ファイルに書く
sudo mv /etc/modprobe.d/btusb_stability.conf \
        /etc/modprobe.d/btusb_stability.conf.invalid
sudo mv /etc/modprobe.d/btusb_disable_autosuspend.conf \
        /etc/modprobe.d/btusb_disable_autosuspend.conf.invalid
printf '%s\n' 'options btusb enable_autosuspend=0' | \
  sudo tee /etc/modprobe.d/btusb-enable-autosuspend.conf

# 3. 再起動して確認する（0 = 無効）
cat /sys/module/btusb/parameters/enable_autosuspend
```

副作用は待機電力の増加のみで、system suspend は継続する。
再起動後も `Y` に戻る場合は別の PM 設定が競合している。
より狭い範囲で試すなら、sysfs の `power/control=on` をアダプタの USB デバイス
（実機では `1-4`）に対して一時的に設定し、効果を確認してから VID/PID 限定の udev rule で
恒久化する。

### P1: Sony 側のマルチポイントを設定する

Android の Sony | Sound Connect で次を設定する。

1. ［接続］→「2 台の機器と同時に接続」をオンにする。
2. 「接続中のリストから機器を固定」で優先端末を固定する。
3. 用途に応じて「再生機器の切り替え」をオン／オフする。
   自動で後から再生した機器へ移したいならオン、手動で固定したいならオフ。
4. 接続切替ができないときは、最後に接続していた再生機器側で Bluetooth を切断し、
   PC 側から再接続する。登録解除は選ばない。

あわせて、ホスト側に `WF-1000XM6` と `LE_ WF-1000XM6` が両方表示された場合は
`WF-1000XM6` を選ぶ。ペアリング済みデバイスが 4 台あるため、
3 台目接続時の入れ替えを避ける目的でも不要用の接続を整理する。
片耳状態がスマホで残る場合は、前節「片耳のみ接続」の回復手順
（forget → 再ペアリング）を先に試す。

### P1: コーデックとプロファイルを絞る

LDAC はビットレートが高く、完了報告欠落を抱えるアダプタでは途切れを悪化させやすい。
Sony 公式は LDAC 再生に Sound Connect の「Bluetooth Connection Quality」を
「Priority on Sound Quality」に設定する必要があると説明し、出荷時の
「High Reliability」では LDAC を再生できない。安定性優先なら次を試す。

- Sound Connect 側を「High Reliability」に設定する。
- WirePlumber の `monitor.bluez.properties` で
  `bluez5.codecs = [ "aac" ]` または `[ "sbc" ]` に制限する。
- LE Audio（BAP/CIS）経路で不具合が出る場合は
  `bluez5.enable-le-audio = false` を設定し Classic A2DP へ戻す。
  その際は `bluez5.enable-bap = false` も併せる。

設定断片は `/etc/wireplumber/wireplumber.conf.d/` に置き、
`systemctl --user restart wireplumber` で反映する。
現在このディレクトリは存在しない（配布既定のみ）。
`bluez5.profile` は「セッションマネージャが上書きするため通常効果がない」と
公式に明記されており、A2DP 固定の手段として使ってはならない。

注: `bluetooth.autoswitch-to-headset-profile` は実測で既に `false` である。
WirePlumber 側の HFP 自動切替は追加で変更しても効果が出ない可能性が高い。

### P2: RF と USB の干渉を切り分ける

USB-IF の白書は、USB 3.x のコネクタ・ケーブル・周辺機器が 2.4〜2.5 GHz 帯へ
広帯域ノイズを放射し、Bluetooth の SNR を下げることを示している。
データ転送中でなく接続中でも影響する。USB 2.0 ポートでも USB 3.x 機器に
隣接していれば改善しない。次を 1 変数ずつ A/B テストする。

1. 近接する USB 3.x ストレージ・ドック・ハブを一時的に外す。
2. Bluetooth アダプタを別ポートへ移す。効果がなければ USB 2.0 延長ケーブルで
   PC の反対側へ配置する。
3. Wi-Fi を 5 GHz 帯へ切り、不可なら 2.4 GHz の 1 / 6 / 11・20 MHz を比較する。
4. イヤホンを PC から 1 m 程度へ近づける。
5. 他の Bluetooth デバイスを一時的に切断する。

kernel.org の Bluetooth/Wi-Fi coexistence 資料によれば、1 つの 802.11 チャンネルは
約 20 個の Bluetooth チャンネルと重なる。AFH は固定干渉を避けられるが、
ノイズが多い環境では通信自体が困難になることがある。

### P2: HFP の自動再接続を抑制する（任意）

BlueZ の `[Policy]` セクションで `ReconnectUUIDs` を A2DP のみに限定すると、
リンクロス時の HFP/HSP 自動再接続を抑えられる。

```text
[Policy]
ReconnectUUIDs=0000110a-0000-1000-8000-00805f9b34fb,0000110b-0000-1000-8000-00805f9b34fb
```

これはリンクロス後の再接続だけを制御し、HFP/HSP の初回自動接続や
policy の HFP↔A2DP 切り替えトリガーまでは止まらない。
`--noplugin=hfp` は BlueZ 5.85 には存在しない（5.86 以降のオプション）ため、
この環境では使えない。HFP を完全に使わないなら WirePlumber 側の
roles から `hfp_hf` / `hfp_ag` を外す方法があるが、マイクが使えなくなる。

## 検証手順

### ログ同時取得

再現試験では 3 系統のログを同時に取得する。

```bash
# ルート権限で HCI トレースを保存
sudo btmon -i hci0 -w /tmp/hci0.snoop

# 別ターミナルでカーネルログを追跡
sudo journalctl -k -b -f

# 別ターミナルでユーザーサービスを追跡
journalctl --user -b -u pipewire.service -u wireplumber.service -u pipewire-pulse.service -f
```

`hcidump` は非推奨のため使わない。`btmon -a` は CIS/BIS のパケットロス統計を出力するが、
Classic A2DP の ACL ドロップを直接証明するものではない。
A2DP では AVDTP の Set Configuration / Suspend / Close と ACL Disconnect を
同時刻の `journalctl` と突き合わせる。

### A/B テストの順序

1. 対策前に 10 分間再生し、`Missing completion reports` の抑制カウンタ最大値を記録する。
2. P0 の `enable_autosuspend=0` を適用し、同一条件で 10 分間再生して比較する。
3. 改善しなければ P1 のコーデックを SBC / AAC へ制限して比較する。
4. さらに改善しなければ USB 3.x 機器の分離と配置変更を試す。
5. 最後に Sony 側のマルチポイント設定を見直す。

### 成功基準

- `Missing completion reports` の抑制カウンタが 0 のまま、または明確に減少する。
- `sep3/fd0 terminated unexpectedly` と `running -> error` が 30 分以上発生しない。
- スマホ再生中に PC 側で再生開始すると、Bluetooth トグルなしで音声が PC へ移る。
- 両側装着時に左右両方から音が出る状態が 30 分以上維持される。

## ロールバック

- modprobe.d: 退避した2ファイルを戻し、`btusb-enable-autosuspend.conf` を削除して
  `sudo modprobe -r btusb && sudo modprobe btusb`、または再起動する。
- WirePlumber: 追加した設定断片を削除し、`systemctl --user restart wireplumber`。
  設定値だけなら `wpctl settings --reset <キー>` で戻せる。
  状態を完全に初期化する場合は `wpctl reset --dry-run` で影響を確認してから実行する。
- Sony 側: 「2 台の機器と同時に接続」をオフに戻す。初期化した場合は再ペアリングが必要。

## 未確認事項

- VID/PID `0411:0374` が RTL8761BU であることは LMP subversion `0x8761` から強く
  示唆されるが、Ubuntu 配布カーネルが独自パッチで Realtek 固有バインドを追加して
  いないかの確認は未実施。
- `btmon` トレースによる AVDTP / L2CAP / ACL の時系列確認。
  17:58:48 の HFP と A2DP の同一秒切断は因果証明ではない。
- 現在の再生コーデック（SBC / AAC / LDAC / LC3）と LE Audio の使用有無。
- 3 台目の機器が実際に接続に関与したか。
- 「装着順序で片側判定され、それがスマホ側に永続する」という仮説。Sony 公式に
  装着順序・時間差の記述はなく、WF-1000XM6 の TWS 実装方式も未確認。
- PC では解消／スマホでは不解消という差の直接原因。BlueZ と Android で
  再接続ポリシーが異なることは確認できるが、WF-1000XM6 での因果は未確認。

## 参照

- [Sony: 2 台の機器と同時に接続](https://helpguide.sony.net/mdr/2985/v1/ja/contents/F034_Connecting_the_headset_to_2_devices_simultaneously.html)
- [Sony サポート: 再生機器の切り替え](https://support.sony.jp/electronics/support/articles/00376855)
- [Sony サポート: 接続の切り替えができない場合](https://support.sony.jp/electronics/support/articles/00352009)
- [Sony ヘルプガイド: 片側だけの使用](https://helpguide.sony.net/mdr/2985/v1/ja/contents/F011_Using_only_one_headset_unit.html)
- [Sony サポート: LE Audio 切替後の片側接続](https://www.sony.com/electronics/support/wireless-headphones-bluetooth-headphones/wf-1000xm6/articles/00366813)
- [Sony サポート: LE_ プレフィックスのみ表示される場合](https://www.sony.com/electronics/support/wireless-headphones-bluetooth-headphones/wf-1000xm6/articles/00204611)
- [Sony ヘルプガイド: Android スマホへの接続](https://helpguide.sony.net/mdr/2985/v1/ja/contents/F024_Connecting_to_a_paired_Android_smartphone.html)
- [Sony サポート: リセットと初期化](https://support.sony.jp/electronics/support/articles/00377954)
- [Qualcomm: True Wireless ホワイトペーパー](https://assets.qualcomm.com/rs/385-TWS-803/images/whitepaper-the-evolution-of-true-wireless-technology-2020.pdf)
- [BlueZ issue #2267](https://github.com/bluez/bluez/issues/2267)
- [AOSP Bluetooth: PhonePolicy.java](https://android.googlesource.com/platform/packages/modules/Bluetooth/+/refs/heads/main/android/app/src/com/android/bluetooth/btservice/PhonePolicy.java)
- [Sony ヘルプガイド: トラブルシューティング](https://helpguide.sony.net/mdr/2985/v1/ja/contents/F063_Troubleshooting.html)
- [Sony ヘルプガイド: 対応コーデック](https://helpguide.sony.net/mdr/2985/v1/en/contents/F055_Supported_codecs.html)
- [WirePlumber: Bluetooth 設定](https://pipewire.pages.freedesktop.org/wireplumber/daemon/configuration/bluetooth.html)
- [WirePlumber: Settings](https://pipewire.pages.freedesktop.org/wireplumber/daemon/configuration/settings.html)
- [PipeWire: bt-latency.h](https://github.com/PipeWire/pipewire/blob/master/spa/plugins/bluez5/bt-latency.h)
- [PipeWire commit d0309b4e](https://github.com/PipeWire/pipewire/commit/d0309b4e1e5a01884d51633af3951c66442cc1c1)
- [BlueZ issue #1992](https://github.com/bluez/bluez/issues/1992)
- [BlueZ issue #1567](https://github.com/bluez/bluez/issues/1567)
- [BlueZ 5.85 main.conf](https://github.com/bluez/bluez/blob/5.85/src/main.conf)
- [kernel: USB power management](https://docs.kernel.org/driver-api/usb/power-management.html)
- [USB-IF: USB 3.0 radio interference](https://usb.org/sites/default/files/327216.pdf)
- [kernel.org: Bluetooth coexistence](https://wireless.docs.kernel.org/en/latest/en/users/documentation/bluetooth-coexistence.html)
- [kernel: btusb_main.c](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/drivers/bluetooth/btusb_main.c?h=master)
- [kernel: btrtl.c](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/plain/drivers/bluetooth/btrtl.c?h=master)
- [kernel commit ca0583c24661](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=ca0583c24661)
