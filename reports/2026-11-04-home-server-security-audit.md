---
type: Report
title: "自宅 Kubuntu サーバのセキュリティ監査（2026-11-04）"
description: "Ubuntu 26.04.1 LTS の常駐サーバ化に伴う外部公開ポート・SSH・権限・更新状況を実測し、優先度付きの対策を整理した監査報告"
tags:
  - security
  - homelab
  - ubuntu
  - audit
generated:
  by: opencode
  at: "2026-11-04T20:26:05+09:00"
status: draft
---

## 概要

自宅 Kubuntu PC を常駐サーバとして運用する前提で、外部公開ポート、認証、権限、更新、監視の状態を
SSH 経由の読み取り調査で確認した。対象は Ubuntu 26.04.1 LTS（kernel 7.0.0-42-generic）。

## 総合評価

基本ハードニングは良好だが、不要なポートの外部公開と SSH のパスワード認証が残る。

| 区分 | 件数 | 代表例 |
| --- | --- | --- |
| High | 3 | SSH パスワード認証、xrdp 全公開、AdGuard 管理画面 |
| Medium | 5 | VS Code ポート、正体不明ポート、docker グループ、ufw 未確認 |
| Low | 3 | X11Forwarding、設定ファイル権限、LUKS なし |
| 良好 | 7 | AppArmor、自動更新、カーネルハードニング、鍵権限 |

## High

### H1: SSH のパスワード認証が有効

`/etc/ssh/sshd_config` に `PasswordAuthentication` の記述がなく、`/etc/ssh/sshd_config.d/` も空である。
OpenSSH のデフォルトは `yes` のため、パスワード認証が有効と判断できる。

- 影響: LAN 内の任意の端末からパスワードブルートフォースが可能。
- 対策: 鍵認証のみに限定する。

### H2: xrdp が全インターフェースに公開

`xrdp.service` が enabled かつ active で、`*:3389` を LISTEN している。
`/etc/xrdp/xrdp.ini` の `port=3389` は全インターフェースを意味する。

- 影響: RDP のブルートフォース、CredSSP 系脆弱性の標的になる。
- 対策: 不要なら停止する。必要なら ufw で LAN と Tailscale だけに許可する。

### H3: AdGuard Home 管理画面が 0.0.0.0:8123

`0.0.0.0:8123` と `[::]:8123` で LISTEN している。
管理画面は認証ありだが、LAN 内の任意の端末から到達できる。

- 影響: 認証情報のブルートフォース、設定改ざんのリスク。
- 対策: `127.0.0.1` バインドに変更し、Tailscale 経由でアクセスする。

## Medium

### M1: VS Code が *:5746 を外部公開

`code` プロセスが `*:5746` を LISTEN している。
ポートフォワードまたは拡張機能の待受と推測される。

- 対策: `ss -tulnp` で用途を確認し、不要なら VS Code のポートフォワードを停止する。

### M2: 正体不明の *:18555

`*:18555` が LISTEN しているが、一般ユーザー権限ではプロセスを特定できない。
`127.0.0.1:18554` と対になっており、Ollama 関連の可能性がある。

- 対策: `sudo ss -tulnp | grep 18555` で特定する。

### M3: docker グループ所属は root 相当

`koppe` は `docker` グループに所属している。
docker ソケットへの書き込みはホストの root 権限に相当する。

- 対策: 常時所属を見直し、必要時だけ `sudo docker` を使う。または rootless Docker へ移行する。

### M4: ufw のルールが未確認

ufw は enabled かつ active だが、許可ルールを一般ユーザーでは読めない。

- 対策: `sudo ufw status numbered` で確認し、デフォルト deny を徹底する。

### M5: fail2ban 未導入

`fail2ban` は導入されていない。
SSH を鍵認証のみにすれば優先度は下がるが、xrdp を残すなら必要。

- 対策: xrdp を残す場合のみ導入する。

## Low

### L1: X11Forwarding が有効

`X11Forwarding yes` が明示されている。使わないなら無効化する。

### L2: opencode 設定ファイルが 664

`~/.config/opencode/opencode.jsonc` と `.bak` が 664 で、他ユーザーから読める。
秘密情報は `.env` と `service.json` に分離されており、それらは 600 で適切。

- 対策: `chmod 600` に統一する。

### L3: ディスク暗号化なし

LUKS は使われておらず、`/dev/nvme0n1p7` を直接マウントしている。
物理アクセス時にディスク全体を読める。

- 対策: 設置場所の物理管理を徹底する。必要なら LUKS + dropbear-initramfs を検討する。

## 良好な点

- AppArmor が有効。
- unattended-upgrades が有効で、セキュリティ更新が自動適用される。
- カーネルハードニングが有効（`kptr_restrict=1`、`dmesg_restrict=1`、`ptrace_scope=1`、
  `rp_filter=2`、`syncookies=1`）。
- SSH 鍵のパーミッションが 700 / 600 で適切。
- `.env` と `service.json` が 600。
- 自動ログインが無効。
- SSH の不正ログインは 30 日間で 4 件のみで、いずれも自分自身の試行ミス。

## 運用上の重大な問題

### Ollama の再起動ループ

`ollama.service` の再起動回数が 65,745 回に達している。
`Restart=always` と `RestartSec=3` により 3 秒間隔で再起動を繰り返している。

- 影響: CPU 消費、ログ肥大化、ディスク圧迫。
- 原因: ポート競合またはモデルロード失敗。`*:18555` が関係している可能性がある。
- 対策: `journalctl -u ollama -n 50` で原因を確認し、設定を修正する。

## 対策手順

### 1. SSH の hardening

```nu
r#'PasswordAuthentication no
KbdInteractiveAuthentication no
PermitRootLogin no
X11Forwarding no
AllowAgentForwarding no
MaxAuthTries 3
ClientAliveInterval 300
ClientAliveCountMax 2
'# | ^sudo tee /etc/ssh/sshd_config.d/10-hardening.conf
^sudo sshd -t
^sudo systemctl reload ssh
```

設定を戻す場合は `10-hardening.conf` を削除し、`sshd -t` の後に reload する。

### 2. xrdp の停止または制限

不要な場合:

```nu
^sudo systemctl disable --now xrdp xrdp-sesman
```

必要な場合:

```nu
^sudo ufw allow from 192.168.128.0/24 to any port 3389 proto tcp
^sudo ufw allow in on tailscale0 to any port 3389 proto tcp
^sudo ufw deny 3389/tcp
```

### 3. AdGuard Home のバインド変更

`/opt/adguardhome/conf/AdGuardHome.yaml` の `http.address` を `127.0.0.1:8123` に変更し、
サービスを再起動する。Tailscale 経由で使う場合は `tailscale serve` を使う。

### 4. ufw の整理

```nu
^sudo ufw status numbered
^sudo ufw default deny incoming
^sudo ufw default allow outgoing
^sudo ufw allow in on tailscale0
^sudo ufw allow from 192.168.128.0/24 to any port 22 proto tcp
```

### 5. fail2ban（xrdp を残す場合）

```nu
^sudo apt install fail2ban
```

`/etc/fail2ban/jail.local`:

```ini
[DEFAULT]
bantime = 1h
findtime = 10m
maxretry = 5

[sshd]
enabled = true

[xrdp]
enabled = true
port = 3389
```

### 6. 設定ファイルの権限

```nu
^chmod 600 ~/.config/opencode/opencode.jsonc ~/.config/opencode/opencode.jsonc.bak
```

### 7. 検証

```nu
^sudo sshd -T | grep -E "passwordauthentication|permitrootlogin|x11forwarding"
^sudo ufw status verbose
^sudo ss -tulnp | grep LISTEN
```

## 要 sudo 確認項目

以下は一般ユーザー権限では取得できなかった。対策の前に手動で確認する。

- `sudo sshd -T`
- `sudo ufw status numbered`
- `sudo ss -tulnp | grep 18555`
- `sudo journalctl -u ollama -n 50`
- `sudo grep -E "address" /opt/adguardhome/conf/AdGuardHome.yaml`

## 参照

- [CIS Ubuntu Linux Benchmark](https://www.cisecurity.org/benchmark/ubuntu_linux)
- [SSH Hardening Guides (ssh-audit.com)](https://www.ssh-audit.com/hardening_guides.html)
- [Tailscale SSH](https://tailscale.com/kb/1193/tailscale-ssh)
- [Ubuntu OpenSSH Server](https://documentation.ubuntu.com/server/how-to/security/openssh-server/)
