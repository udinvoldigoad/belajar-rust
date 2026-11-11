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