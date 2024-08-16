fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"), // _ is a catchall value and should always be the last value in the match expression
    }

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        _ => (), // the () is the unit value, so nothing will happen
    }
}
