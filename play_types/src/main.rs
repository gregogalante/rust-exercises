struct Person {
    name: String,
    age: u8,
}

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let decimal = 65.4321;
    let integer = 88;
    let hex = 0x11;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';

    // NOTE: The println! will print the value of the variable as number
    println!("decimal: {}", decimal); // -> 65.4321
    println!("integer: {}", integer); // -> 88
    println!("hex: {}", hex); // -> 17
    println!("octal: {}", octal); // -> 63
    println!("binary: {}", binary); // -> 240
    println!("byte: {}", byte); // -> 65

    let tuple = (1, 2, 3);
    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c); // -> a: 1, b: 2, c: 3
    println!("tuple.0: {}, tuple.1: {}, tuple.2: {}", tuple.0, tuple.1, tuple.2); // -> tuple.0: 1, tuple.1: 2, tuple.2: 3
    println!("tuple: {:?}", tuple); // -> tuple: (1, 2, 3)

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("name: {}, age: {}", person.name, person.age); // -> name: Alice, age: 30

    let color = Color::Red;
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    } // -> Red
}
