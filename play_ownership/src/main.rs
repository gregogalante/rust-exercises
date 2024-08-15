fn main() {
    // IMPORTANT: Ownership is a unique feature of Rust.
    // Ownership is a way to manage memory in Rust. Thanks to ownership, Rust can guarantee memory safety without a garbage collector.
    // In Rust memory is managed through a set of rules that the compiler checks at compile time. If the rules are not followed, the code will not compile.

    // STACK AND HEAP
    // In Rust, memory is divided into two parts: the stack and the heap.
    // The stack stores values in the order it gets them and removes the values in the opposite order.
    // The stack is fast because of the way it stores and retrieves data, but it is limited in size.
    // Each data stored in the stack must have a known, fixed size.
    // The heap is less organized: when you put data on the heap, you request a certain amount of space.
    // The operating system finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location in memory.
    // This process is called allocating on the heap and is less fast than storing data on the stack.
    // The pointer is a reference to a location in memory and can be stored on the stack.
    // The pointer itself is stored on the stack, but when we want to access the data it refers to, we must follow the pointer.

    // When we call a function, the values passed to the function are pushed onto the stack.
    // When the function is over, the values are popped off the stack.
    // Functions can also receive pointers to data on the heap.

    // OWNERSHIP PURPOSES
    // 1. Keep track of what parts of code are using what data on the heap.
    // 2. Minimize the amount of duplicate data on the heap.
    // 3. Clean up unused data on the heap to avoid memory leaks.

    // OWNERSHIP RULES
    // 1. Each value in Rust has a variable that is its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let s = "Hello"; // s is a string literal, which is immutable and stored on the stack.
    println!("{}", s);

    let s = String::from("Hello"); // s is a String, which is mutable and stored on the heap.
    println!("{}", s);

    let s1 = 5;
    let s2 = s1;
    println!("{}", s1); // s1 and s2 are integers, which are stored on the stack.
    println!("{}", s2); // s1 and s2 are integers, which are stored on the stack.

    let s1 = "Hello";
    let s2 = s1;
    println!("{}", s1); // s1 and s2 are string literals, which are stored on the stack.
    println!("{}", s2); // s1 and s2 are string literals, which are stored on the stack.

    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}", s1); // This will not compile because s1 and s2 are Strings, which are stored on the heap.
    println!("{}", s2); // s1 and s2 are Strings, which are stored on the heap.
    // NOTE: When we assign s1 to s2, we are copying the pointer, length, and capacity of the String to s2.
    // In the same time s1 is invalidated, so we cannot use it anymore.
    // NOTE: For integers, floats, booleans, characters, tuples (if they contain types that are also Copy), and other simple types, the value is stored on the stack and a copy is made when assigning the value to another variable.
    
}
