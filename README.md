<div align="center">

# WaBot-Rust

WhatsApp bot modular ditulis dengan Rust, pairing code login, dan sistem plugin.

![Rust](https://img.shields.io/badge/rust-2024_edition-dea584?style=flat-square&logo=rust&logoColor=white)
![WhatsApp](https://img.shields.io/badge/whatsapp-rust-25D366?style=flat-square&logo=whatsapp&logoColor=white)
![Tokio](https://img.shields.io/badge/runtime-tokio-blue?style=flat-square&logo=tokio&logoColor=white)
![SQLite](https://img.shields.io/badge/storage-sqlite3-003B57?style=flat-square&logo=sqlite&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-black?style=flat-square)

</div>

---

## Fitur

- Pairing code 8-digit tanpa scan QR.
- Samaran identitas browser Chrome di Windows.
- Penyimpanan session SQLite di folder `sessions/`.
- Sistem plugin modular di `plugins/tools/`.
- Multi-prefix (`!`, `.`, `#`, `/`).
- Mode bot `self` (hanya owner) atau `public` (semua user).
- Eksekusi perintah terminal host (`$ <cmd>`) khusus owner.
- Resolusi otomatis ID internal WhatsApp (LID) ke nomor telepon asli.

## Konfigurasi

Edit file `config.json` di root direktori:

```json
{
  "phone_number": "",
  "prefixes": ["!", ".", "#", "/"],
  "mode": "self",
  "owner_numbers": ["628123456789"],
  "db_path": "sessions/bot.db"
}
```

| Opsi | Keterangan |
|---|---|
| `phone_number` | Nomor WhatsApp bot (format 62xxx tanpa tanda `+`). Kosongkan jika ingin input manual di terminal saat start. |
| `prefixes` | Daftar awalan command bot. |
| `mode` | `self` (cuma owner yang bisa pakai) atau `public` (semua orang bisa pakai). |
| `owner_numbers` | Nomor yang punya akses command exec `$` dan bypass mode self. |
| `db_path` | Lokasi file database session SQLite. |

## Cara Pasang & Menjalankan

### Windows

Pastikan sudah install Rust (via rustup) dan Git. Buka PowerShell di folder project:

```powershell
cargo run --release
```

Tunggu 8 karakter kode pairing muncul di terminal, lalu buka:
**WhatsApp > Perangkat Tertaut > Tautkan Perangkat > Tautkan dengan nomor telepon saja** dan masukkan kodenya.

### Termux (Android)

Install paket yang dibutuhkan:

```bash
pkg update && pkg upgrade -y
pkg install rust git clang sqlite binutils -y
```

Jalankan di folder bot:

```bash
cargo run --release
```

## Command Bawaan

| Command | Akses | Keterangan |
|---|---|---|
| `!ping` | Sesuai Mode | Menampilkan latency, CPU clock, pemakaian RAM proses (RSS), total RAM, dan uptime. |
| `!menu` | Sesuai Mode | Menampilkan daftar plugin yang terpasang. |
| `$ <cmd>` | Khusus Owner | Menjalankan perintah terminal host (contoh: `$ ls`, `$ whoami`, `$ git status`). |
