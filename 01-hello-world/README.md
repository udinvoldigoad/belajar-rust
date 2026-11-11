# 01 - Hello World & Cargo

- **Tanggal belajar:** 2026-11-11

## 1. `cargo run`: compile + jalankan

```bash
cargo run -p hello-world
```

Yang terjadi di belakang layar:
1. Cargo **compile** kode di `src/main.rs` jadi file program (`.exe` di Windows).
2. Hasilnya disimpan di folder `target/debug/`.
3. Cargo langsung **menjalankan** file `target/debug/hello-world.exe` itu.

> `-p hello-world` dipakai karena project ini workspace (banyak project dalam satu folder).
> Kalau terminal sudah di dalam folder `01-hello-world`, cukup `cargo run`.

Program hasil compile bisa juga dijalankan langsung tanpa cargo:
```bash
./target/debug/hello-world.exe
```

## 2. `cargo build --release`: bikin file distribusi

```bash
cargo build --release -p hello-world
```

- Cuma **compile**, tidak menjalankan.
- Hasilnya ada di `target/release/hello-world.exe`.
- File inilah yang dibagikan ke orang lain / dipakai di production.

Jalankan hasilnya:
```bash
./target/release/hello-world.exe
```

## 3. Bedanya debug vs release

| | Debug (`cargo run` / `cargo build`) | Release (`cargo build --release`) |
|---|---|---|
| Folder hasil | `target/debug/` | `target/release/` |
| Kecepatan compile | Cepat | Lebih lama |
| Kecepatan program | Lebih lambat | Lebih cepat (dioptimasi) |
| Dipakai untuk | Saat ngoding & testing | File final untuk dibagikan |

**Intinya:** pakai `cargo run` waktu belajar/ngoding, pakai `cargo build --release` kalau mau bikin program jadi.

## 4. Unit test dengan `cargo test`

Fungsi test ditandai dengan `#[test]` di atasnya:

```rust
#[test]
fn hello_test() {
    println!("Hello Test");
}
```

Jalankan test:

| Perintah | Fungsi |
|---|---|
| `cargo test` | Jalankan semua test |
| `cargo test hello_test` | Jalankan test yang namanya **mengandung** `hello_test` |
| `cargo test hello_test -- --exact` | Jalankan test yang namanya **persis** `hello_test` |
| `cargo test hello_test -- --exact --nocapture` | Sama, tapi output `println!` ikut **ditampilkan** |

Kenapa perlu `--nocapture`? Secara default cargo **menyembunyikan** output `println!` di dalam test
(biar hasil test rapi). Tambahkan `--nocapture` kalau mau lihat output-nya.

```
running 1 test
Hello Test              <- muncul karena --nocapture
test hello_test ... ok
```

> Tanda `--` memisahkan opsi untuk cargo (sebelah kiri) dan opsi untuk program test-nya (sebelah kanan).
> `--exact` dan `--nocapture` itu opsi untuk program test, jadi ditaruh setelah `--`.

## 5. Variable

- Variable adalah **tempat untuk menyimpan data**.
- Dibuat pakai kata kunci `let`.
- Secara default variable di Rust **tidak bisa diubah** (immutable) setelah diisi data.

```rust
#[test]
fn test_variable() {
    let name = "Ridho Maulana Saputra";
    println!("Hello {}", name);
}
```

- `{}` di dalam `println!` adalah tempat nilai variable dimasukkan.
- Jalankan: `cargo test test_variable -- --exact --nocapture`

Output:
```
Hello Ridho Maulana Saputra
```

Kalau dicoba diubah, hasilnya **error** saat compile:
```rust
let name = "Ridho";
name = "Udin"; // error: cannot assign twice to immutable variable
```

## 6. Mutable variable (`let mut`)

Kalau variable perlu **diubah nilainya**, tambahkan kata kunci `mut` setelah `let`.

```rust
#[test]
fn test_mutable() {
    let mut name = "Ridho Maulana Saputra";
    println!("Hello {}", name);

    name = "Udin Voldigoad"; // boleh, karena pakai mut
    println!("Hello {}", name);
}
```

- Jalankan: `cargo test test_mutable -- --exact --nocapture`

Output:
```
Hello Ridho Maulana Saputra
Hello Udin Voldigoad
```

| | `let` | `let mut` |
|---|---|---|
| Nilai bisa diubah? | Tidak (immutable) | Bisa (mutable) |
| Kalau diubah | Error saat compile | Aman |

> Tips: pakai `let` biasa dulu. Tambahkan `mut` hanya kalau memang perlu diubah.

## 7. Static typing

- Rust itu bahasa **static typing**: sekali variable dibuat dengan tipe data tertentu,
  tipenya **tidak bisa berubah** jadi tipe lain.
- Contoh: variable berisi teks (string) tidak bisa diisi angka (number), walaupun pakai `mut`.

```rust
let mut name = "Ridho"; // tipe: teks (&str)
name = 10;              // error! ini angka, bukan teks
```

Error saat compile:
```
error[E0308]: mismatched types
  |     name = 10;
  |            ^^ expected `&str`, found integer
```

> `mut` cuma membolehkan **nilai** diubah, bukan **tipe datanya**.
> `"Udin"` -> boleh (sama-sama teks), `10` -> tidak boleh (angka).

> Catatan: folder `target/` ada di root workspace (`belajar-rust/target/`), bukan di dalam `01-hello-world/`.
> Folder ini juga di-ignore git karena isinya hasil compile, bisa dibuat ulang kapan saja.
