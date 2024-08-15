fn main() {
    let i = 5;
    call_int(i);
    println!("value after call_int is: {}", i);

    let s = String::from("hello");
    call_string(s); // This will move s ownership to call_string so we can't use s after this
    // println!("value after call_string is: {}", s); // This will not compile because s has been moved to call_string

    let s = String::from("hello");
    call_string(s.clone()); // This will clone s and pass the clone to call_string so we can still use s after this
    println!("value after call_string is: {}", s);

    let s = String::from("hello");
    call_string_ref(&s); // This will pass a reference to s so we can still use s after this
    println!("value after call_string_ref is: {}", s);

    // NOTE: The final example is the most efficient way to pass a string to a function without losing ownership.
    // This is because we are passing a reference to the string instead of cloning it.
    // Cloning a heap allocated value can be expensive in terms of performance.
    // What is happening here is that we are passing a reference to the string to the function.
    // A reference is a pointer to the other pointer (in stack) that points to the actual value (in heap).
    // With this way we can not require the ownership of the value to be passed to the function.

    let mut s = String::from("hello");
    complete_string(&mut s); // This will pass a mutable reference to s so we can still use s after this
    println!("value after complete_string is: {}", s);
    
    let mut s = String::from("hello");
    let s1 = &mut s;
    // let s2 = &mut s; // This will not compile because we can't have multiple mutable references to the same value on the same scope
    println!("value after s1 is: {}", s1);

    let mut s = String::from("hello");
    {
        let s2 = &mut s;
        s2.push_str(" world");
    }
    let s1 = &mut s; // This will compile because s2 is out of scope
    println!("value after s1 is: {}", s1); // -> hello world

    let mut s = String::from("hello");
    let s1 = &s;
    let s2 = &s;
    // let s3 = &mut s; // This will not compile because we can't have a mutable reference to the same value if we have an immutable reference to it
    println!("value after s1 and s2 is: {} {}", s1, s2);
    let s3 = &mut s; // This will compile because s1 and s2 are out of scope (they are not used after this line)
    println!("value after s3 is: {}", s3);
}

fn call_int(i: i32) {
    println!("value inside call_int is: {}", i);
}

fn call_string(s: String) {
    println!("value inside call_string is: {}", s);
}

fn call_string_ref(s: &String) {
    println!("value inside call_string_ref is: {}", s);
}

fn complete_string(s: &mut String) {
    s.push_str(" world");
}