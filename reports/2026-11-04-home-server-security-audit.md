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
| High | 3 | SSH パスワード認証、xrdp 全公開、Home Assistant |
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

### H3: Home Assistant が 0.0.0.0:8123

`0.0.0.0:8123` と `[::]:8123` で LISTEN している。
プロセスは `python3 -P -m homeassistant --config /config` で、root で稼働している。
LAN 内の任意の端末から到達できる。

- 影響: 認証情報のブルートフォース、設定改ざんのリスク。
- 対策: ufw で LAN と Tailscale だけに許可し、外部公開しない。

## Medium

### M1: VS Code が *:5746 を外部公開

`code` プロセスが `*:5746` を LISTEN している。
ポートフォワードまたは拡張機能の待受と推測される。

- 対策: `ss -tulnp` で用途を確認し、不要なら VS Code のポートフォワードを停止する。

### M2: go2rtc が *:18555 を外部公開

`*:18555` は `go2rtc` が LISTEN している。
Home Assistant のカメラストリーミング用サービスで、root で稼働している。

- 対策: ufw で遮断したままにし、Home Assistant の 8123 経由で利用する。

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
- 原因: `ollama` ユーザーのホームディレクトリ `/usr/share/ollama` が存在せず、
  `mkdir /usr/share/ollama: permission denied` で起動に失敗していた。
- 対策: `sudo mkdir -p /usr/share/ollama/.ollama` と
  `sudo chown -R ollama:ollama /usr/share/ollama` で解消する。

## 対策手順

### 1. SSH の hardening

```bash
sudo tee /etc/ssh/sshd_config.d/10-hardening.conf > /dev/null <<'EOF'
PasswordAuthentication no
KbdInteractiveAuthentication no
PermitRootLogin no
X11Forwarding no
AllowAgentForwarding no
MaxAuthTries 3
ClientAliveInterval 300
ClientAliveCountMax 2
EOF
sudo sshd -t
sudo systemctl reload ssh
```

設定を戻す場合は `10-hardening.conf` を削除し、`sshd -t` の後に reload する。

### 2. xrdp の停止または制限

不要な場合:

```bash
sudo systemctl disable --now xrdp xrdp-sesman
```

必要な場合:

```bash
sudo ufw allow from 192.168.128.0/24 to any port 3389 proto tcp
sudo ufw allow in on tailscale0 to any port 3389 proto tcp
sudo ufw deny 3389/tcp
```

### 3. Home Assistant の公開範囲

Home Assistant は `8123/tcp` で待ち受ける。ufw で LAN と Tailscale だけに許可し、
ルーターのポート開放は行わない。

```bash
sudo ufw allow from 192.168.128.0/24 to any port 8123 proto tcp
sudo ufw allow in on tailscale0 to any port 8123 proto tcp
```

### 4. ufw の整理

```bash
sudo ufw status numbered
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow from 192.168.128.0/24 to any port 22 proto tcp
sudo ufw allow in on tailscale0 to any port 22 proto tcp
```

### 5. fail2ban（xrdp を残す場合）

```bash
sudo apt install fail2ban
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

```bash
chmod 600 "$HOME/.config/opencode/opencode.jsonc" "$HOME/.config/opencode/opencode.jsonc.bak"
```

### 7. 検証

```bash
sudo sshd -T | grep -E "passwordauthentication|permitrootlogin|x11forwarding"
sudo ufw status verbose
sudo ss -tulnp | grep LISTEN
```

## 要 sudo 確認項目

以下は一般ユーザー権限では取得できなかった。対策の前に手動で確認する。

- `sudo sshd -T`
- `sudo ufw status numbered`
- `sudo ss -tulnp | grep -E "18555|5746|8123|3389"`
- `sudo journalctl -u ollama -n 50`

## 訂正

2026-11-05 の実行確認で、初版の記述に次の誤りが見つかったため訂正した。

- H3 の 8123 は AdGuard Home ではなく Home Assistant だった。
- M2 の 18555 は正体不明ではなく go2rtc だった。
- Ollama の再起動ループはポート競合ではなく、`/usr/share/ollama` の権限不足だった。
- 対策手順のコード例を Nushell から Bash に統一した。

## 参照

- [CIS Ubuntu Linux Benchmark](https://www.cisecurity.org/benchmark/ubuntu_linux)
- [SSH Hardening Guides (ssh-audit.com)](https://www.ssh-audit.com/hardening_guides.html)
- [Tailscale SSH](https://tailscale.com/kb/1193/tailscale-ssh)
- [Ubuntu OpenSSH Server](https://documentation.ubuntu.com/server/how-to/security/openssh-server/)
