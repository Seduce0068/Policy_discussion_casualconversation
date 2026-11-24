---
type: Report
title: "自宅常駐サーバのセキュリティ監査（2026-11-04）"
description: "Linux 常駐サーバの外部公開ポート・認証・権限・更新状況を実測し、優先度付きの対策を整理した監査報告"
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

自宅 PC を常駐サーバとして運用する前提で、外部公開ポート、認証、権限、更新、監視の状態を
SSH 経由の読み取り調査で確認した。対象は Linux ディストリビューションの常駐構成で、
OS 版・カーネル版は `<os-version>` / `<kernel-version>` として汎化している。

## 総合評価

基本ハードニングは良好だが、不要なポートの外部公開と SSH のパスワード認証が残る。

| 区分 | 件数 | 代表例 |
| --- | --- | --- |
| High | 3 | SSH パスワード認証、RDP 全公開、ホームオートメーション公開 |
| Medium | 5 | エディタポート、正体不明ポート、コンテナ特権グループ、ufw 未確認 |
| Low | 3 | X11Forwarding、設定ファイル権限、LUKS なし |
| 良好 | 7 | AppArmor、自動更新、カーネルハードニング、鍵権限 |

## High

### H1: SSH でパスワード認証が残っている

SSH サーバ設定に `PasswordAuthentication` の明示的な無効化がなく、
Drop-in 設定も空だった。OpenSSH の既定に従いパスワード認証が有効と判断できる。

- 影響: 同一ネットワーク内の端末からパスワードブルートフォースが可能。
- 対策: 鍵認証のみに限定する。

### H2: リモートデスクトップサービスが全インターフェースに公開

RDP サービスが enabled かつ active で、全アドレス（`<rdp-port>`）を LISTEN している。
設定の `port=` にアドレス指定がなく、全インターフェースを意味する。

- 影響: RDP のブルートフォース、CredSSP 系脆弱性の標的になる。
- 対策: 不要なら停止する。必要ならファイアウォールで LAN と VPN だけに許可する。

### H3: ホームオートメーションサービスが全アドレスで待ち受け

スマートホーム系サービスが全 IPv4/IPv6 アドレス（`<service-port>`）で LISTEN し、
特権ユーザーで稼働している。LAN 内の任意の端末から到達できる。

- 影響: 認証情報のブルートフォース、設定改ざんのリスク。
- 対策: ファイアウォールで LAN と VPN だけに許可し、外部公開しない。

## Medium

### M1: エディタが予備ポートを外部公開

エディタプロセスが `<editor-port>` を LISTEN している。
ポートフォワードまたは拡張機能の待受と推測される。

- 対策: `ss -tulnp` で用途を確認し、不要ならポートフォワードを停止する。

### M2: カメラストリーミングサービスが外部公開

`<camera-port>` はカメラストリーミングサービスが LISTEN している。
ホームオートメーションのカメラ用で、特権ユーザーで稼働している。

- 対策: ファイアウォールで遮断したままにし、ホームオートメーションサービス経由で利用する。

### M3: コンテナ用特権グループ所属は root 相当

運用ユーザー（`<user>`）はコンテナ用特権グループに所属している。
ソケットへの書き込みはホストの root 権限に相当する。

- 対策: 常時所属を見直し、必要時だけ `sudo` 経由で使う。または rootless 構成へ移行する。

### M4: ufw のルールが未確認

ufw は enabled かつ active だが、許可ルールを一般ユーザーでは読めない。

- 対策: `sudo ufw status numbered` で確認し、デフォルト deny を徹底する。

### M5: fail2ban 未導入

`fail2ban` は導入されていない。
SSH を鍵認証のみにすれば優先度は下がるが、RDP サービスを残すなら必要。

- 対策: RDP サービスを残す場合のみ導入する。

## Low

### L1: X11Forwarding が有効

`X11Forwarding yes` が明示されている。使わないなら無効化する。

### L2: エージェント設定ファイルが他ユーザーから読める状態

ユーザー設定ディレクトリ内のエージェント設定ファイルとバックアップが 664 で、
他ユーザーから読める。秘密情報は個別ファイルへ分離され、そちらは 600 で適切。

- 対策: `chmod 600` に統一する。

### L3: ディスク暗号化なし

ディスク暗号化（LUKS）は使われておらず、`<disk-device>` を直接マウントしている。
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

### 2. RDP サービスの停止または制限

不要な場合:

```bash
sudo systemctl disable --now <rdp-service>
```

必要な場合:

```bash
sudo ufw allow from <private-ip>/24 to any port <rdp-port> proto tcp
sudo ufw allow in on <vpn-iface> to any port <rdp-port> proto tcp
sudo ufw deny <rdp-port>/tcp
```

### 3. ホームオートメーションサービスの公開範囲

ホームオートメーションサービスは `<service-port>/tcp` で待ち受ける。ufw で LAN と VPN だけに許可し、
ルーターのポート開放は行わない。

```bash
sudo ufw allow from <private-ip>/24 to any port <service-port> proto tcp
sudo ufw allow in on <vpn-iface> to any port <service-port> proto tcp
```

### 4. ufw の整理

```bash
sudo ufw status numbered
sudo ufw default deny incoming
sudo ufw default allow outgoing
sudo ufw allow from <private-ip>/24 to any port 22 proto tcp
sudo ufw allow in on <vpn-iface> to any port 22 proto tcp
```

### 5. fail2ban（RDP サービスを残す場合）

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

[rdp]
enabled = true
port = <rdp-port>
```

### 6. 設定ファイルの権限

```bash
chmod 600 "$HOME/.config/<agent>/config.jsonc" "$HOME/.config/<agent>/config.jsonc.bak"
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
- `sudo ss -tulnp | grep LISTEN`（待受ポートの洗い出し）
- `sudo journalctl -u ollama -n 50`

## 訂正

2026-11-05 の実行確認で、初版の記述に次の誤りが見つかったため訂正した。

- H3 の待受ポートのサービスは、当初想定していたものではなくホームオートメーションサービスだった。
- M2 の待受ポートのサービスは、正体不明ではなくカメラストリーミングサービスだった。
- Ollama の再起動ループはポート競合ではなく、`/usr/share/ollama` の権限不足だった。
- 対策手順のコード例を Nushell から Bash に統一した。

## 参照

- [CIS Ubuntu Linux Benchmark](https://www.cisecurity.org/benchmark/ubuntu_linux)
- [SSH Hardening Guides (ssh-audit.com)](https://www.ssh-audit.com/hardening_guides.html)
- [Tailscale SSH](https://tailscale.com/kb/1193/tailscale-ssh)
- [Ubuntu OpenSSH Server](https://documentation.ubuntu.com/server/how-to/security/openssh-server/)
