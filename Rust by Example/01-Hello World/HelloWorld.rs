// 1. Hello World
// https://doc.rust-lang.org/rust-by-example/hello.html

// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
    println!("I'm a Rustacean!");
}

// println! is a macro that prints text to the console.
// https://doc.rust-lang.org/rust-by-example/macros.html

// A binary can be generated using the Rust compiler: rustc.
// $ rustc HelloWorld.rs

// rustc will produce a HelloWorld binary that can be executed.
// $ ./HelloWorld
// Hello World!
