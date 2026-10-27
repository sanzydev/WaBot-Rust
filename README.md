<div align="center">

# WaBot-Rust

A lightweight, modular WhatsApp bot built with Rust, pairing code authentication, and dynamic plugin support.

![Rust](https://img.shields.io/badge/rust-2024_edition-dea584?style=flat-square&logo=rust&logoColor=white)
![WhatsApp](https://img.shields.io/badge/whatsapp-rust-25D366?style=flat-square&logo=whatsapp&logoColor=white)
![Tokio](https://img.shields.io/badge/runtime-tokio-blue?style=flat-square&logo=tokio&logoColor=white)
![SQLite](https://img.shields.io/badge/storage-sqlite3-003B57?style=flat-square&logo=sqlite&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-black?style=flat-square)

[English](#features) | [Bahasa Indonesia](#bahasa-indonesia)

</div>

---

## Features

- 8-digit pairing code authentication (no QR scanning needed).
- Windows Chrome browser identity override.
- Isolated SQLite session storage in `sessions/`.
- Modular plugin architecture under `plugins/tools/`.
- Dynamic multi-prefix routing (`!`, `.`, `#`, `/`).
- Dual access modes (`self` or `public`).
- Host shell execution (`$ <cmd>`) restricted to owner.
- Automatic internal WhatsApp ID (LID) resolution to actual phone number.

## Configuration

Edit `config.json` in the root directory:

```json
{
  "phone_number": "",
  "prefixes": ["!", ".", "#", "/"],
  "mode": "self",
  "owner_numbers": ["628123456789"],
  "db_path": "sessions/bot.db"
}
```

| Key | Description |
|---|---|
| `phone_number` | Bot WhatsApp phone number (international format without `+`). Leave empty for terminal prompt. |
| `prefixes` | Accepted command prefixes. |
| `mode` | `self` (owner-only access) or `public` (accessible by all users). |
| `owner_numbers` | Authorized numbers for owner-level commands and shell execution (`$`). |
| `db_path` | SQLite session database path. |

## Installation & Usage

### Windows

Requirements: Rust toolchain (rustup) and Git.

```powershell
git clone https://github.com/sanzydev/WaBot-Rust.git
cd WaBot-Rust
cargo run --release
```

An 8-digit pairing code will display in the terminal. Open WhatsApp on your phone:
**Settings > Linked Devices > Link a Device > Link with phone number instead** and enter the code.

### Mobile (Termux)

Install build dependencies:

```bash
pkg update && pkg upgrade -y
pkg install rust git clang sqlite binutils -y
```

Clone and run:

```bash
git clone https://github.com/sanzydev/WaBot-Rust.git
cd WaBot-Rust
cargo run --release
```

## Built-in Commands

| Command | Access | Description |
|---|---|---|
| `!ping` | By Mode | Displays latency, CPU clock, process memory (RSS), system RAM, and uptime. |
| `!menu` | By Mode | Lists all registered plugins. |
| `$ <cmd>` | Owner Only | Executes terminal commands on host (e.g., `$ ls`, `$ whoami`, `$ git status`). |

---

## Bahasa Indonesia

<details>
<summary>Klik untuk membuka panduan versi Bahasa Indonesia</summary>

### Fitur

- Pairing code 8-digit tanpa scan QR.
- Samaran identitas browser Chrome di Windows.
- Penyimpanan session SQLite di folder `sessions/`.
- Sistem plugin modular di `plugins/tools/`.
- Multi-prefix (`!`, `.`, `#`, `/`).
- Mode bot `self` (hanya owner) atau `public` (semua user).
- Eksekusi perintah terminal host (`$ <cmd>`) khusus owner.
- Resolusi otomatis ID internal WhatsApp (LID) ke nomor telepon asli.

### Panduan Instalasi Singkat

**Windows**:
```powershell
cargo run --release
```

**Termux**:
```bash
pkg update && pkg upgrade -y
pkg install rust git clang sqlite binutils -y
cargo run --release
```

Masukkan kode pairing yang muncul ke **WhatsApp > Perangkat Tertaut > Tautkan dengan nomor telepon**.

### Command Bawaan

- `!ping` : Info latency, clock CPU, RAM RSS proses, total RAM sistem, dan uptime.
- `!menu` : Daftar plugin yang aktif.
- `$ <cmd>` : Jalankan command terminal host langsung dari chat WhatsApp (khusus nomor owner).

</details>
