---
type: Report
title: "自宅 Kubuntu の IPv6 有効化と外部到達性の評価"
description: "HR02 の RA によるグローバル IPv6 取得、ufw の IPv6 ルール整備、外部到達性テストの結果と運用方針"
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

自宅 Kubuntu PC で IPv6 を有効化し、グローバルアドレスの取得、ufw の IPv6 ルール整備、
外部からの到達性を評価した。HR02 は RA で `240a:61:1221:b391::/64` を配布しており、
PC は SLAAC でグローバル IPv6 を取得できた。

一方、スマホのモバイル回線は IPv6 非対応で、外部到達性の実地テストができなかった。
HR02 の IP アドレスフィルタリングは拒否モードで、外部からの inbound は遮断されている。

## 実施内容

### 1. NetworkManager の設定変更

`ipv6.method` を `dhcp` から `auto` に変更した。
`dhcp` はカーネルの `accept_ra` を 0 にするため、RA を受け取れず、
SLAAC のアドレスもデフォルトルートも設定されなかった。

```bash
sudo nmcli connection modify cable1 ipv6.method auto
sudo nmcli connection down cable1
sudo nmcli connection up cable1
```

### 2. RA の確認

`rdisc6` で HR02 の RA を確認した。

```text
Prefix                   : 240a:61:1221:b391::/64
Autonomous address conf. : Yes
Router lifetime          : 65535 seconds
default via fe80::64b4:a9cf:2f36:8861
```

### 3. 取得したアドレス

| 種類 | アドレス |
| --- | --- |
| temporary dynamic | `240a:61:1221:b391:5f27:4e7e:931f:bc5d/64` |
| mngtmpaddr（stable） | `240a:61:1221:b391:3aac:9861:5bf4:c4de/64` |
| ULA temporary | `fde1:9adb:d4f2:9574:f340:c07b:5c64:4f70/64` |
| ULA stable | `fde1:9adb:d4f2:9574:e6e2:677f:a01e:5f71/64` |

### 4. ufw の IPv6 ルール

`ip6tables -L -n` では `-i tailscale0` が表示されず、全公開に見えたが、
`ip6tables-save` で確認した結果、全公開ルールは存在しなかった。

現在の許可:

| ポート | 許可元 | 用途 |
| --- | --- | --- |
| 22/tcp | `tailscale0` | Tailscale 経由の SSH |
| 22/tcp | `240a:61:1221:b391::/64` | LAN 内 IPv6 からの SSH |
| 8123/tcp | `tailscale0` | Tailscale 経由の Home Assistant |
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

### 6. HR02 のフィルタ

HR02 の「IPアドレスフィルタリング」はフィルターモードが「拒否」で、リストは空。
外部からの inbound は遮断されている。

## 判断

- IPv6 の LAN 内通信は有効化し、Home Assistant、mDNS、KDE Connect で活用する。
- 外部からの IPv6 直接公開は行わない。理由は次のとおり。
  - スマホのモバイル回線が IPv6 非対応で、実地の到達性テストができない。
  - HR02 の許可ルール追加と、動的なプレフィックスへの DDNS 追随が必要。
  - Tailscale の DERP 経由が既に動作しており、追加のポート開放が不要。
- 外部アクセスの主経路は Tailscale とし、IPv6 は補助経路として扱う。

## 運用方針

| 場所 | 経路 |
| --- | --- |
| 自宅 LAN | `ssh koppe@192.168.128.111` |
| 外出先 | Android 版 Tailscale + `ssh koppe@100.117.19.54` |
| IPv6 | LAN 内の通信に限定 |

## 参照

- [NTT ドコモ: IPv6 シングルスタック方式の提供開始](https://www.docomo.ne.jp/info/news_release/2022/01/31_00.html)
- [HR02 取扱説明書](https://www.docomo.ne.jp/binary/pdf/support/manual/HR02_J_syousai.pdf)
- [NetworkManager ipv6 settings](https://networkmanager.dev/docs/api/latest/settings-ipv6.html)
