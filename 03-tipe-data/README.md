# 03 - Tipe Data

- **Tanggal belajar:** 2026-11-11

## 1. Tipe data

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

## 2. Boolean

- Tipe data yang nilainya cuma 2: `true` (benar) atau `false` (salah).
- Nama tipenya `bool`.

```rust
#[test]
fn boolean() {
    let a = true;        // implicit: Rust tahu ini bool
    let b: bool = false; // explicit: tipe ditulis sendiri
    println!("{} {}", a, b);
}
```

- Jalankan: `cargo test boolean -- --exact --nocapture`

Output:
```
true false
```

> `println!` bisa menampilkan lebih dari satu nilai: setiap `{}` diisi berurutan oleh variable setelahnya.

## 3. Char

- Tipe data untuk **satu karakter**.
- Ditulis pakai **petik satu** `'a'`. (Petik dua `"a"` itu teks/string, beda tipe.)

```rust
#[test]
fn char() {
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}
```

- Jalankan: `cargo test char -- --exact --nocapture`

Output:
```
a b
```

## 4. Tuple

- **Compound type**: kumpulan beberapa data, tipenya **boleh beda**.
- Ditulis pakai kurung `( )`. Jumlah datanya tetap, tidak bisa ditambah/dikurangi.

```rust
#[test]
fn tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // mengakses tuple pakai titik (.) + index (mulai dari 0)
    let a = data.0;
    let b = data.1;
    let c = data.2;
    println!("{} {} {}", a, b, c);

    // destructuring: bongkar isi tuple ke beberapa variable sekaligus
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
}
```

- Jalankan: `cargo test tuple -- --exact --nocapture`

Output:
```
(10, 10.5, true)
10 10.5 true
10 10.5 true
```

| Cara | Contoh | Keterangan |
|---|---|---|
| Akses pakai index | `data.0` | ambil data ke-0 (pertama) |
| Destructuring | `let (a, b, c) = data;` | ambil semua sekaligus |

> `{:?}` dipakai untuk menampilkan tuple, karena `{}` biasa tidak bisa menampilkan tuple.

### Tuple mutable

Kalau isi tuple mau diubah, tambahkan `mut`. Ubah datanya lewat index.

```rust
#[test]
fn tuple_mutable() {
    let mut data = (10, 10.6, true);
    println!("{:?}", data);

    data.0 = 15;
    data.1 = 10.2;
    data.2 = false;
    println!("{:?}", data);
}
```

- Jalankan: `cargo test tuple_mutable -- --exact --nocapture`

Output:
```
(10, 10.6, true)
(15, 10.2, false)
```

> Tipe datanya tetap tidak boleh berubah (static typing): `data.0 = "teks"` akan error.
