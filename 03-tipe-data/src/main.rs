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

// Char
#[test]
fn char() {
    let char1 = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}

// Tuple
#[test]
fn tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    // mengakses tuple pakai titik (.)
    let a = data.0;
    let b = data.1;
    let c= data.2;
    println!("{} {} {}", a, b, c);

    // destructuring tuple
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);
}

// Tuple Mutable
#[test]
fn tuple_mutable() {
    let mut data = (10, 10.6, true);
    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 15;
    data.1 = 10.2;
    data.2 = false;
    println!("{:?}", data);
}