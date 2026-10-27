# wabot-rust

Bot WhatsApp modular pake Rust. Pairing code, sistem plugin, session SQLite.

## Konfigurasi

Edit file config.json di root folder:

```json
{
  "phone_number": "628123456789",
  "prefixes": ["!", ".", "#", "/"],
  "mode": "self",
  "owner_numbers": ["628123456789"],
  "db_path": "sessions/bot.db"
}
```

- phone_number: nomor bot (awalan kode negara tanpa tanda +). Kalo dikosongin nanti bakal nanya di terminal pas start.
- mode: isi "self" biar cuma lu doang yg bisa pake, atau "public" buat semua orang.
- owner_numbers: nomor lu buat akses exec $ sama bypass mode self.
- db_path: tempat nyimpen session database.

## Cara Install & Jalanin

### Windows

Pastikan udah install Rust (rustup) dan Git.

Buka terminal di folder bot, langsung jalanin:

```powershell
cargo run --release
```

Tunggu kode pairing 8 digit muncul, buka WhatsApp > Perangkat Tertaut > Tautkan dengan nomor telepon, lalu masukin kodenya.

### Termux (Android)

Install paket yang dibutuhin dulu:

```bash
pkg update && pkg upgrade -y
pkg install rust git clang sqlite binutils -y
```

Masuk ke folder bot dan jalanin:

```bash
cargo run --release
```

Tinggal masukin kode pairing yang muncul ke WA.

## Command Bawaan

- !ping : cek latency, clock cpu, pemakaian ram rss, ram sistem, sama uptime.
- !menu : liat daftar plugin aktif.
- $ <cmd> : eksekusi command shell langsung di terminal host (khusus owner).
  Contoh Windows: $ dir, $ whoami
  Contoh Termux: $ ls, $ uname -a
