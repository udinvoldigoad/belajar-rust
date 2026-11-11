# 04 - Operator

- **Tanggal belajar:** 2026-11-11

## 1. Operasi numerik

Rust mendukung semua operasi numerik, operatornya sama seperti bahasa pemrograman lain.

| Operator | Keterangan | Contoh (`a = 10`, `b = 5`) |
|---|---|---|
| `+` | tambah | `a + b` = 15 |
| `-` | kurang | `a - b` = 5 |
| `*` | kali | `a * b` = 50 |
| `/` | bagi | `a / b` = 2 |
| `%` | sisa bagi (modulo) | `a % b` = 0 |

```rust
#[test]
fn numeric_operator() {
    let a = 10;
    let b = 5;
    let c = a * b;
    let d = a + b;
    println!("{}", c);
    println!("{}", d);
}
```

- Jalankan: `cargo test numeric_operator -- --exact --nocapture`

Output:
```
50
15
```

> Pembagian `/` pada integer hasilnya **tetap bilangan bulat**, bagian desimalnya dibuang: `10 / 3` = 3.
> Kalau mau hasil desimal, pakai float: `10.0 / 3.0` = 3.333...

## 2. Augmented assignment

Cara singkat untuk mengubah nilai variable dengan operasi numerik.

| Cara panjang | Cara singkat |
|---|---|
| `a = a + 10` | `a += 10` |
| `a = a - 10` | `a -= 10` |
| `a = a * 10` | `a *= 10` |
| `a = a / 10` | `a /= 10` |
| `a = a % 10` | `a %= 10` |

> Variable-nya harus `mut`, karena nilainya diubah.

```rust
#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a); // 10

    a += 5;            // a = 10 + 5
    println!("{}", a); // 15

    a -= 2;            // a = 15 - 2
    println!("{}", a); // 13
}
```

- Jalankan: `cargo test augmented_assignment -- --exact --nocapture`

Output:
```
10
15
13
```

## 3. Comparison operator (operator perbandingan)

Operator untuk **membandingkan** dua nilai. Hasilnya selalu **boolean** (`true` / `false`).

| Operator | Keterangan | Contoh (`a = 10`, `b = 20`) |
|---|---|---|
| `>` | lebih dari | `a > b` = false |
| `<` | kurang dari | `a < b` = true |
| `>=` | lebih dari sama dengan | `a >= b` = false |
| `<=` | kurang dari sama dengan | `a <= b` = true |
| `==` | sama dengan | `a == b` = false |
| `!=` | tidak sama dengan | `a != b` = true |

> `=` (satu) untuk **mengisi** variable, `==` (dua) untuk **membandingkan**.
> `!=` tidak ada di slide, tapi juga sering dipakai.

```rust
#[test]
fn comparison() {
    let a = 10;
    let b = 20;

    let result = a > b;
    println!("{}", result); // false

    let result = a >= b;
    println!("{}", result); // false

    let result = a < b;
    println!("{}", result); // true

    let result = a <= b;
    println!("{}", result); // true

    let result = a == b;
    println!("{}", result); // false
}
```

- Jalankan: `cargo test comparison -- --exact --nocapture`

> Di contoh ini `let result` ditulis berulang kali. Itu **shadowing** (lihat [02-variable](../02-variable/README.md)).
