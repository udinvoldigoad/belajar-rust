fn main() {}

// Belajar Tipe data
#[test]
fn explicit() {
    /*  
        tipe data disebutkan secara ekplisit disebelah variable(defaultnya rust otomatis
        dalam menentukan tipe data, jadi tidak perlu di tulis  secara eksplisit)
    */
    let age: i32 = 20;
    println!("{}", age);
}

// Number
#[test]
fn number() {
    let a: i32 = 10;
    println!("{}", a);

    let b: f64 = 10.5;
    println!("{}", b);
}

// Number Conversion
#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    /*
        ini bakal terjadi integer overflow (perlu di perhatikan 
        ketika konversi dari tipe data yang nilainya lebih tinggi ke rendah)
     */
    let d: i64 = 1000000000;
    let e: i8 = d as i8;
    println!("{}", e);
}

// Boolean
#[test]
fn boolean() {
    let a = true; // implicit
    let b: bool = false; // explicit
    println!("{} {}", a, b);
}
