fn main() {
    println!("Hello, world!");

    another_function(12);

    let x = {
        let y = 3;
        y + 1
    };
    println!("The value of x is: {}", x); // 4
}

fn another_function(x: i32) {
    println!("Another function with x = {}", x);
}