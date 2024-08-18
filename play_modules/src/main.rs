pub mod restaurant {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }

        pub fn seat_at_table() {
            println!("seat_at_table");
        }

        pub mod server {
            use super::super::kitchen;
            use super::super::serving;

            pub fn take_order() {
                serving::take_order();
            }

            pub fn serve_order() {
                kitchen::cook_order();
                serving::serve_order();
            }

            pub fn take_payment() {
                serving::take_payment();
            }
        }
    }

    mod serving {
        pub fn take_order() {
            println!("take_order");
        }

        pub fn serve_order() {
            println!("serve_order");
        }

        pub fn take_payment() {
            println!("take_payment");
        }
    }

    mod kitchen {
        pub fn cook_order() {
            println!("cook_order");
        }
    }
}

fn main() {
    // RUST MODULE SYSTEM
    // - Packages: A bundle of one or more crates. It contains a Cargo.toml file that describes how to build those crates.
    // - Crates: A tree of modules that produces a library or executable.
    // - Modules and use: Let you control the organization, scope, and privacy of paths.
    // - Paths: A way of naming an item, such as a struct, function, or module.

    // The src/main.rs file is an entry point for a binary crate, and src/lib.rs is an entry point for a library crate.

    // RULES
    // 1. The compiler starts from the crate root (from main.rs or lib.rs).
    // 2. In the crate root (main.rs or lib.rs) we can define modules using the mod keyword.
    // - Modules can be inline, in a file (named with the module name), or in a directory (named with the module name and containing a mod.rs file).
    // 3. Modules can contain other submodules. They are files or directories nested inside the module directory (directory named with the module name).
    // 4. By default a module is private. It can be made public using the pub keyword.

    // USE KEYWORD
    // The use keyword is used to bring a path into scope.
    // It is a shorcut to avoid writing the full path every time.

    restaurant::hosting::add_to_waitlist();
    restaurant::hosting::seat_at_table();
    restaurant::hosting::server::take_order();
    restaurant::hosting::server::serve_order();
    restaurant::hosting::server::take_payment();
}
