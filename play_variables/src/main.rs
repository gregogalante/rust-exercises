fn main() {
    let z: i32 = 5; // NOTE: This is an explicit type annotation
    println!("The value of z is: {}", z);

    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // NOTE: This will not compile because x is immutable
    // println!("The value of x is: {}", x);

    let mut y = 5;
    println!("The value of y is: {}", y);
    y = 6; // NOTE: This will compile because y is mutable
    println!("The value of y is: {}", y);

    let a = 5;
    println!("The value of a is: {}", a);
    let a = 6; // NOTE: We can re-declare a new variable with the same name, the previous value is shadowed
    println!("The value of a is: {}", a);
    let a = a + 1; // NOTE: We can also perform operations on the previous value
    println!("The value of a is: {}", a);

    let b = 5;
    println!("The value of b is: {}", b); // -> 5
    { // NOTE: This is a block scope, the code inside the block is a separate scope
        let b = 10; // NOTE: This is a new variable b, the previous value is not shadowed because it is in a separate scope
        println!("The value of b is: {}", b); // -> 10
    }
    let b = b + 1; // NOTE: This is the original b, the value is not shadowed
    println!("The value of b is: {}", b); // -> 6

    let c = 5;
    println!("The value of c is: {}", c);
    let c = "five"; // NOTE: We can re-declare a new variable with a different type
    println!("The value of c is: {}", c);

    let mut d = 5;
    println!("The value of d is: {}", d);
    // d = "five"; // NOTE: This will not compile because d is an integer
    // println!("The value of d is: {}", d);

    const MAX_POINTS: u32 = 100_000; // NOTE: This is a constant, the value cannot be changed, the type must be annotated, the assignment must be on same line
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
