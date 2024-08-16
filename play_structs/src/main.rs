
fn main() {
    // Structs are custom data types with a name and fields.
    // Default fields are immutable, but can be made mutable with the `mut` keyword.
    // We can not have a single field mutable, all fields must be mutable or immutable.

    let c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    // c.red = 200; // ERROR: Cannot assign to `c.red` because it is borrowed.
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut c1 = build_color(255, 0, 0);
    println!("Color c1: {} {} {}", c1.red, c1.green, c1.blue);

    let c2 = build_color(c1.red, c1.green, c1.blue);
    println!("Color c2: {} {} {}", c2.red, c2.green, c2.blue);

    c1.red = 200;
    println!("Color c1 after: {} {} {}", c1.red, c1.green, c1.blue);
    println!("Color c2 after: {} {} {}", c2.red, c2.green, c2.blue); // NOTE: c2 is not changed because it is a copy of c1 (not a reference).

    let p = Point(1.0, 2.0, 3.0);
    println!("Point: {} {} {}", p.0, p.1, p.2);

    let r = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Rectangle: {} {}", r.width, r.height);
    println!("Rectangle: {:?}", r); // NOTE: {:?} is used to print debug information. It is a trait that must be implemented for the type.
    println!("Rectangle: {:#?}", r); // NOTE: {:#?} is used to print debug information with pretty print.

    let r = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Area: {}", r.area());
    println!("Can hold: {}", r.can_hold(&Rectangle { width: 5, height: 10 }));
    println!("Width: {}", r.width());

    let r = Rectangle::square(10);
    println!("Square: {:?}", r);
    println!("Height: {}", r.height());
}

struct Color { // NOTE: Usually name is singular and capitalized.
    red: u8,
    green: u8,
    blue: u8,
}

fn build_color(red: u8, green: u8, blue: u8) -> Color {
    Color { red, green, blue }
}

struct Point(f64, f64, f64); // NOTE: Tuple struct with unnamed fields. You can access fields by index.

#[derive(Debug)] // NOTE: This is a derive attribute that allows us to print debug information.
struct Rectangle {
    width: u32,
    height: u32,
}

// Structs can also have methods.
impl Rectangle {

    // Rectangle methods (like public methods in other languages)
    //

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn width(&self) -> bool { // NOTE: we can have a method with the same name as a field.
        self.width > 0
    }

    // Rectangle associated functions (like static methods in other languages)
    //

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

impl Rectangle {
    // We can have multiple `impl` blocks for the same struct.
    // This is useful when we want to separate methods and associated functions.

    fn height(&self) -> bool {
        self.height > 0
    }
}