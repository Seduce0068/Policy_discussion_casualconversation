---
type: Report
title: "自宅 Linux の IPv6 有効化と外部到達性の評価"
description: "`<router>` の RA によるグローバル IPv6 取得、ufw の IPv6 ルール整備、外部到達性テストの結果と運用方針"
tags:
  - ipv6
  - homelab
  - ufw
  - tailscale
generated:
  by: opencode
  at: "2026-11-06T00:40:00+09:00"
status: review
---

## 概要

自宅 Linux PC で IPv6 を有効化し、グローバルアドレスの取得、ufw の IPv6 ルール整備、
外部からの到達性を評価した。`<router>` は RA で `2001:db8:1::/64` を配布しており、
PC は SLAAC でグローバル IPv6 を取得できた。

一方、スマホのモバイル回線は IPv6 非対応で、外部到達性の実地テストができなかった。
`<router>` の IP アドレスフィルタリングは拒否モードで、外部からの inbound は遮断されている。

## 実施内容

### 1. NetworkManager の設定変更

`ipv6.method` を `dhcp` から `auto` に変更した。
`dhcp` はカーネルの `accept_ra` を 0 にするため、RA を受け取れず、
SLAAC のアドレスもデフォルトルートも設定されなかった。

```bash
sudo nmcli connection modify <lan-iface> ipv6.method auto
sudo nmcli connection down <lan-iface>
sudo nmcli connection up <lan-iface>
```

### 2. RA の確認

`rdisc6` で `<router>` の RA を確認した。

```text
Prefix                   : 2001:db8:1::/64
Autonomous address conf. : Yes
Router lifetime          : 65535 seconds
default via fe80::1
```

### 3. 取得したアドレス

| 種類 | アドレス |
| --- | --- |
| temporary dynamic | `2001:db8:1:<temporary-host-id>/64` |
| mngtmpaddr（stable） | `2001:db8:1:<stable-host-id>/64` |
| ULA temporary | `fd00:1:<temporary-host-id>/64` |
| ULA stable | `fd00:1:<stable-host-id>/64` |

### 4. ufw の IPv6 ルール

`ip6tables -L -n` では `-i <vpn-iface>` が表示されず、全公開に見えたが、
`ip6tables-save` で確認した結果、全公開ルールは存在しなかった。

現在の許可:

| ポート | 許可元 | 用途 |
| --- | --- | --- |
| 22/tcp | `<vpn-iface>` | Tailscale 経由の SSH |
| 22/tcp | `2001:db8:1::/64` | LAN 内 IPv6 からの SSH |
| 8123/tcp | `<vpn-iface>` | Tailscale 経由の Home Assistant |
| 41641/udp | 全 IPv6 | Tailscale |
| 5353/udp | LAN | mDNS |
| 7000,7001,7100/tcp | LAN | KDE Connect |
| 6000,6001,7011/udp | LAN | KDE Connect |
| 1714:1764/tcp | IPv4 LAN | KDE Connect |
| 1714:1764/udp | IPv4 LAN | KDE Connect |

### 5. 外部到達性テスト

スマホのモバイル回線から `ping6` と `nc -6` を試した。

```text
3 packets transmitted, 0 received, 100% packet loss
nc: connect to ... port 22 (tcp) failed: Network is unreachable
```

`Network is unreachable` はスマホ側に IPv6 の経路がないことを示す。
モバイル回線の APN が IPv4 のみ、または IPv6 のデフォルトルートが未設定。

### 6. `<router>` のフィルタ

`<router>` の「IPアドレスフィルタリング」はフィルターモードが「拒否」で、リストは空。
外部からの inbound は遮断されている。

## 判断

- IPv6 の LAN 内通信は有効化し、Home Assistant、mDNS、KDE Connect で活用する。
- 外部からの IPv6 直接公開は行わない。理由は次のとおり。
  - スマホのモバイル回線が IPv6 非対応で、実地の到達性テストができない。
  - `<router>` の許可ルール追加と、動的なプレフィックスへの DDNS 追随が必要。
  - Tailscale の DERP 経由が既に動作しており、追加のポート開放が不要。
- 外部アクセスの主経路は Tailscale とし、IPv6 は補助経路として扱う。

## 運用方針

| 場所 | 経路 |
| --- | --- |
| 自宅 LAN | `ssh USER@<private-ip>` |
| 外出先 | Android 版 Tailscale + `ssh USER@<tailnet-ip>` |
| IPv6 | LAN 内の通信に限定 |

## 参照

- [NTT ドコモ: IPv6 シングルスタック方式の提供開始](https://www.docomo.ne.jp/info/news_release/2022/01/31_00.html)
- ルーター取扱説明書（メーカー公開 PDF。型番を汎化したため URL は記載しない）
- [NetworkManager ipv6 settings](https://networkmanager.dev/docs/api/latest/settings-ipv6.html)
