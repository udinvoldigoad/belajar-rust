# Belajar Rust

Catatan belajar Rust, satu folder per topik.

- **Sumber:** [Tutorial Rust Dasar (Bahasa Indonesia)](https://www.youtube.com/watch?v=FkASrE05VY4&list=PL-CtdCApEFH-eZEVNvpvKULzy460jbynw)

## Daftar materi

| No | Folder | Isi |
|---|---|---|
| 00 | [00-instalasi](00-instalasi/NOTES.md) | Install Rust, rustup, setup VS Code |
| 01 | [01-hello-world](01-hello-world/README.md) | `cargo run`, `cargo build --release`, `cargo test`, komentar |
| 02 | [02-variable](02-variable/README.md) | `let`, `let mut`, static typing, shadowing |
| 03 | [03-tipe-data](03-tipe-data/README.md) | Scalar & compound, integer, float, konversi `as`, boolean |
| 04 | [04-operator](04-operator/README.md) | Operasi numerik, augmented assignment, perbandingan |

## Cara menjalankan

Semua perintah dijalankan dari folder root (`belajar-rust/`):

```bash
cargo run -p hello-world        # jalankan program di satu folder
cargo test                      # jalankan semua test di semua folder
cargo test -p variable          # jalankan test di satu folder saja
cargo test shadowing -- --exact --nocapture   # satu test + tampilkan output
```

> `-p` diisi **nama package** (lihat `name` di `Cargo.toml` tiap folder), bukan nama folder.

## Menambah materi baru

```bash
cargo new 05-nama-topik --name nama-topik
```

- Nama folder diawali nomor supaya urut, dan otomatis masuk workspace
  (`Cargo.toml` root memakai `members = ["[0-9][0-9]-*"]`).
- `--name` wajib diisi karena nama package Rust tidak boleh diawali angka.
- Jangan lupa tambahkan barisnya ke tabel **Daftar materi** di atas.
