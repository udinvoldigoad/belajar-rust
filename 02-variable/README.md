# 02 - Variable

- **Tanggal belajar:** 2026-11-11

## 1. Variable

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

## 2. Mutable variable (`let mut`)

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

## 3. Static typing

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

## 4. Shadowing

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
