fn main() {}

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

// Comparison Operators
#[test]
fn comparison() {
    let a = 10;
    let b = 20;

    let result = a > b;
    println!("{}", result);

    let result = a >= b;
    println!("{}", result);

    let result = a < b;
    println!("{}", result);

    let result = a <= b;
    println!("{}", result);

    let result = a == b;
    println!("{}", result);
}