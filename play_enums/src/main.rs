fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:?}", four);
    println!("six: {:?}", six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);
    home.call();
    loopback.call();

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);
    println!("absent_number: {:?}", absent_number);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = None;
    println!("x: {:?}", x);
    println!("y: {:?}", y);
    println!("z: {:?}", z);
    let sum_x_y = x + y.unwrap();
    let sum_x_z = x + z.unwrap_or(0); // NOTE: unwrap_or() returns the value if Some, otherwise the default value. unwrap() panics if None.
    println!("sum_x_y: {:?}", sum_x_y);
    println!("sum_x_z: {:?}", sum_x_z);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

impl IpAddr2 {
    fn call(&self) {
        println!("call: {:?}", self);
    }
}

// enum Option<T> {
//     Some(T),
//     None,
// }