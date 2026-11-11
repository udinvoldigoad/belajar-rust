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

## 8. Shadowing

- Di Rust kita **boleh** membuat variable baru dengan **nama yang sama** pakai `let` lagi.
- Variable yang lama jadi **tertutup** (tidak bisa dipakai lagi). Ini disebut **shadowing**.
- Karena ini variable **baru**, tipe datanya boleh beda (teks jadi angka).
- Boleh dipakai, tapi jangan terlalu sering karena bisa bikin bingung yang baca kode.

```rust
#[test]
fn shadowing() {
    let name = "Ridho Maulana Saputra"; // teks
    println!("Hello {}", name);

    let name = 10; // variable baru, nama sama, tipe angka
    println!("Hello {}", name);
}
```

- Jalankan: `cargo test shadowing -- --exact --nocapture`

Output:
```
Hello Ridho Maulana Saputra
Hello 10
```

Bedanya dengan `mut`:

| | `let mut` + ubah nilai | Shadowing (`let` lagi) |
|---|---|---|
| Cara | `name = ...` | `let name = ...` |
| Variable | Tetap variable yang sama | Bikin variable **baru** |
| Boleh ganti tipe data? | Tidak (static typing) | Boleh |

## 9. Komentar

- Komentar adalah tulisan di kode yang **dihapus/diabaikan saat compile**, jadi tidak ikut jalan.
- Dipakai untuk menambahkan **catatan/informasi** agar kode mudah dipahami (oleh kita atau programmer lain).

| Jenis | Cara tulis |
|---|---|
| Satu baris | `// komentar` |
| Lebih dari satu baris | diawali `/*` dan ditutup `*/` |

```rust
/*
    ini komentar lebih dari 1 baris
    ini komentar lebih dari 1 baris
 */
#[test]
fn comment() {
    // ini komentar satu baris
    println!("Hello") // komentar juga bisa di akhir baris
}
```

## 10. Tipe data

Setiap nilai di Rust punya tipe data. Garis besarnya dibagi 2:

| Kelompok | Isi | Jenis |
|---|---|---|
| **Scalar** | Satu nilai (nilai tunggal) | integer, float, boolean, char |
| **Compound** | Beberapa nilai dalam satu tipe | tuple, array |

**Scalar type:**
- **Integer**: angka bulat, contoh `10`
- **Float**: angka desimal, contoh `10.5`
- **Boolean**: hanya `true` (benar) atau `false` (salah)
- **Char**: satu karakter, contoh `'A'`

**Compound type:**
- **Tuple**: kumpulan beberapa data, tipenya **boleh beda**
- **Array**: kumpulan beberapa data, tipenya **harus sama**

### Explicit type

Defaultnya Rust otomatis menebak tipe data, jadi tidak wajib ditulis.
Kalau mau ditulis sendiri (explicit), pakai tanda `:` (titik dua):

```rust
let age: i32 = 20;   // explicit: tipe ditulis
let age = 20;        // implicit: Rust yang menentukan
```

### Integer type (angka bulat)

| Panjang | Signed (bisa minus) | Unsigned (tidak bisa minus) |
|---|---|---|
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |

> `i` = signed (boleh negatif), `u` = unsigned (hanya 0 ke atas).
> Makin besar bit-nya, makin besar angka yang bisa ditampung.

### Float type (angka desimal)

| Panjang | Float |
|---|---|
| 32-bit | `f32` |
| 64-bit | `f64` |

### isize & usize

Integer yang panjangnya **mengikuti platform** (sistem operasi): 32-bit atau 64-bit.

| Tipe | Keterangan |
|---|---|
| `isize` | signed, 32-bit / 64-bit |
| `usize` | unsigned, 32-bit / 64-bit |

### Default number

Kalau tipe tidak ditulis (implicit), Rust pakai default:
- Angka bulat -> `i32`
- Angka desimal -> `f64`

```rust
#[test]
fn number() {
    let a: i32 = 10;
    println!("{}", a);

    let b: f64 = 10.5;
    println!("{}", b);
}
```

### Konversi tipe number (`as`)

Pakai kata kunci `as` untuk mengubah tipe number.

```rust
#[test]
fn number_conversion() {
    let a: i8 = 10;
    let b: i16 = a as i16;  // kecil -> besar: aman
    let c: i32 = b as i32;  // kecil -> besar: aman

    let d: i64 = 1000000000;
    let e: i8 = d as i8;    // besar -> kecil: integer overflow!
    println!("{}", e);
}
```

Output `e` adalah `0`, bukan `1000000000`, karena `i8` cuma bisa menampung -128 sampai 127.

> **Integer overflow**: nilai terlalu besar untuk ditampung tipe tujuan, jadi hasilnya jadi angka lain (salah).
> Hati-hati saat konversi dari tipe **besar ke kecil**.

- Jalankan: `cargo test number_conversion -- --exact --nocapture`

> Catatan: folder `target/` ada di root workspace (`belajar-rust/target/`), bukan di dalam `01-hello-world/`.
> Folder ini juga di-ignore git karena isinya hasil compile, bisa dibuat ulang kapan saja.
