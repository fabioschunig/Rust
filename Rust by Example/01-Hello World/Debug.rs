// 1.2.1. Debug
// https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html

// All types which want to use std::fmt formatting traits require an implementation to be printable.
// All types can derive (automatically create) the fmt::Debug implementation.
// This is not true for fmt::Display which must be manually implemented.

// This structure cannot be printed either with `fmt::Display` or with `fmt::Debug`.
// > struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
// > #[derive(Debug)]
// > struct DebugPrintable(i32);

// Derive the `fmt::Debug` implementation for `Structure`.
// `Structure` is a structure which contains a single `i32`.
#[derive(Debug)]
#[allow(dead_code)]
struct Structure(i32);

// Put a `Structure` inside of the structure `Deep`. Make it printable also.
#[derive(Debug)]
#[allow(dead_code)]
struct Deep(Structure);

// So fmt::Debug definitely makes this printable but sacrifices some elegance.
// Rust also provides "pretty printing" with {:#?}.
#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}
