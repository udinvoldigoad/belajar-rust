# 00 - Instalasi

- **Video:** [
Tutorial Rust Dasar (Bahasa Indonesia)](https://www.youtube.com/watch?v=FkASrE05VY4&list=PL-CtdCApEFH-eZEVNvpvKULzy460jbynw)
- **Tanggal belajar:** 2026-11-11

## 1. Install Rust lewat `rustup`

`rustup` itu installer + version manager resmi Rust. Sekali install, dapat 3 tool:
- `rustc`: compiler
- `cargo`: build tool + package manager (yang paling sering dipakai)
- `rustup`: buat update / ganti versi toolchain

### Windows
1. Download `rustup-init.exe` dari https://rustup.rs
2. Jalankan, pilih opsi **1 (default)**.
3. Kalau diminta install **Visual Studio C++ Build Tools**, ikuti saja. Rust di Windows butuh
   linker MSVC. Centang workload *"Desktop development with C++"*.
4. Tutup lalu buka lagi terminal biar PATH ke-update.

## 2. Verifikasi

```bash
rustc --version
cargo --version
rustup --version
```

Hasil di laptopku:
```
C:\Users\ASUS>rustc --version
rustc 1.99.1 (58cf8f9c3 2026-10-13)

C:\Users\ASUS>cargo --version
cargo 1.99.1 (5f94df478 2026-08-27)

C:\Users\ASUS>rustup --version
rustup 1.29.3 (edbc831ce 2026-11-02)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: the currently active `rustc` version is `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
```

## 3. Setup Editor (VS Code)

- Install extension **rust-analyzer** (autocomplete, error inline, go to definition)
- Opsional: **CodeLLDB** buat debugging, **Even Better TOML** buat file `Cargo.toml`
- Aktifkan *format on save* supaya `rustfmt` jalan otomatis

## 4. Perintah Cargo Dasar

| Perintah | Fungsi |
|----------|--------|
| `cargo new nama` | Buat project baru |
| `cargo build` | Compile (mode debug) |
| `cargo run` | Compile + jalankan |
| `cargo test` | Jalankan unit test |
| `cargo check` | Cek error tanpa build binary (lebih cepat) |
| `cargo fmt` | Rapikan format kode |
| `cargo clippy` | Lint, kasih saran kode yang lebih idiomatik |
| `rustup update` | Update Rust ke versi terbaru |
| `rustup doc --book` | Buka The Rust Book secara offline |
