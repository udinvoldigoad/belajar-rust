fn main() {
    println!("Hello, World");

    println!("Helllo, Udin");

    println!("hello, Ridho");
}

#[test]
fn hello_test() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Ridho Maulana Saputra";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Ridho Maulana Saputra";
    println!("Hello {}", name);

    name = "Udin Voldigoad";
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Ridho Maulana Saputra";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);

    println!("Hello {}", name);
}


/*
    ini komentar lebih dari 1 baris
    ini komentar lebih dari 1 baris
    ini komentar lebih dari 1 baris
 */
#[test]
fn comment() {
    // ini komentar
    println!("Hello") // ini komentar
}

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

// Numeric Operator
#[test]
fn numeric_operator() {
    let a = 10;
    let b = 5;
    let c = a * b;
    let d = a + b;
    println!("{}", c);
    println!("{}", d);
}

// Augmented Assignment
#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 5;
    println!("{}", a);

    a -= 2;
    println!("{}", a);
}