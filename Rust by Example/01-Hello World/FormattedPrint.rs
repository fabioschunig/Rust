// 1.2. Formatted print
// https://doc.rust-lang.org/rust-by-example/hello/print.html

// Printing is handled by a series of macros [¹] defined in std::fmt [²] some of which are:
//   > format!: write formatted text to String [³]
//   > print!: same as format! but the text is printed to the console (io::stdout).
//   > println!: same as print! but a newline is appended.
//   > eprint!: same as print! but the text is printed to the standard error (io::stderr).
//   > eprintln!: same as eprint! but a newline is appended.
//
// All parse text in the same fashion.
// As a plus, Rust checks formatting correctness at compile time.
//
// [¹] Macros   https://doc.rust-lang.org/rust-by-example/macros.html
// [²] std::fmt https://doc.rust-lang.org/std/fmt/
// [³] String   https://doc.rust-lang.org/rust-by-example/std/str.html

fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}", 69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
                                          // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1); // 10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // Activity in doc page
    let pi = 3.141592;
    println!("Pi is roughly {0:.3}", pi);
}

// std::fmt contains many traits [¹] which govern the display of text.
// The base form of two important ones are listed below:
//   > fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
//   > fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.

// Here, we used fmt::Display because the std library provides implementations for these types.
// To print text for custom types, more steps are required.

// Implementing the fmt::Display trait automatically implements the ToString [²] trait which
// allows us to convert [³] the type to String.

// In line 43, #[allow(dead_code)] is an attribute [⁴] which only applies to the module after it.

// [¹] traits       https://doc.rust-lang.org/std/fmt/#formatting-traits
// [²] ToString     https://doc.rust-lang.org/std/string/trait.ToString.html
// [³] convert      https://doc.rust-lang.org/rust-by-example/conversion/string.html
// [⁴] attribute    https://doc.rust-lang.org/rust-by-example/attribute.html

// # Activities
// Add a println! macro call that prints: "Pi is roughly 3.142" by controlling the number of decimal
// places shown. For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi.
// (Hint: you may need to check the std::fmt [¹] documentation for setting the number of decimals to display)

// [¹] precison     https://doc.rust-lang.org/std/fmt/#precision
