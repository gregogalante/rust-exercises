fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    let coin = Coin::Penny;
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    println!("The value of coin is: {}", value);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("The value of counter is: {}", counter);
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of result is: {}", result);

    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("The value of element is: {}", element);
    }
    for number in (1..4).rev() { // NOTE: 1..4 will not include 4
        println!("The value of number is: {}", number);
    }
    for number in (1..=4).rev() { // NOTE: 1..=4 will include 4
        println!("The value of number is: {}", number);
    }
}
