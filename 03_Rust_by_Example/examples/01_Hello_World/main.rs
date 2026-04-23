#![allow(dead_code)]

// # 1. Hello World

// This is a comment, and is ignored by the compiler.

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
    println!("I'm a Rustacean!");

    // println! is a macro that prints to the console

    formatted_print();
    debug_formatting();
    display_formatting();
}

// This can be run using `cargo run`
// or, by `cargo build` and then manually running the executable

// or by compiling the code with `rustc` and then running the binary
// `rustc main.rs` will produce an executable named `main` (or `main.exe` on Windows)
// `./main` (or `main.exe`) will run the compiled program

// ## 1.1 Comments

// ### Regular Comments

// Ignored by the compiler.
// - Line Comments: Start with `//` and continue to the end of the line.
// - Block Comments: Start with `/*` and end with `*/`. Can span multiple lines.

// ### Documentation Comments

// Used to generate documentation for the code. Parsed into HTML library documentation. https://doc.rust-lang.org/rust-by-example/meta/doc.html

// - /// Generates docs for the item that follows. Can be used on functions, structs, enums, etc.
// - //! Generates docs for the enclosing item, such as a module or crate. Placed at the beginning of a file or module.

// ## 1.2 Formatted Print

// https://doc.rust-lang.org/std/fmt/

// Printing is handled by a series of macros defined in `std::fmt`:
// - `format!`: Write formatted text to a `String`
// - `print!`: Same as `format!` but text is printed to the console (`io::stdout`)
// - `println!`: Same as `print!` but a newline is appended
// - `eprint!`: Same as `print!` but the text is printed to the standard error (`io::stderr`)
// - `eprintln!`: Same as `eprintln!` but a newline is appended
// All parse text in the same fashion; and as a bonus, Rust checks formatting correctness at compile time!

fn formatted_print() {
    // In general, the `{}` will automatically be replaced with any arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used as well by specifying an index in the curly braces. Arguments start at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments can also be used by specifying an identifier in the curly braces.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting options are available. For example to display a number in binary, octal, or hexadecimal.
    let number = 1234;
    println!("Base 10:               {}", number); // 1234
    println!("Base 2 (binary):       {:b}", number); // 10011010010
    println!("Base 8 (octal):        {:o}", number); // 2322
    println!("Base 16 (hexadecimal): {:x}", number); // 4d2

    // You can right-justify text with a specified width. This will output "     1" (4 spaces and a "1" for a total width of 5).
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:0>5}", number = 1);

    // You can also left-adjust by flipping the sign (<). This will output "10000" (a "1" followed by 4 zeroes for a total width of 5).
    println!("{number:0<5}", number = 1);

    // You can use named arguments in the format specfier by appending a `$`
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure that the correct number of arguments are used.
    // println!("My name is {0}, {1} {0}", "Bond"); // error: 1 positional argument in format string, but 2 were supplied
    println!("My name is {0}, {1} {0}", "Bond", "James"); // My name is Bond, James Bond

    // Only types that implement the `std::fmt::Display` trait can be formatted with `{}`.
    // This includes most primitive types, and some standard library types like `String` and `Vec<T>`, but not tuples.
    // User-defined types do not implement `Display` by default, but you can implement it yourself to enable formatting with `{}`.

    #[allow(dead_code)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement `Display`.
    // println!("This struct `{}` won't print...", Structure(3)); // error: `Structure` doesn't implement `std::fmt::Display`

    // Starting from Rust 1.58, you can directly capture the argument from a surrounding variable.
    let number = 1;
    let width = 5;
    println!("{number:0>width$}");
}

// `std::fmt` contains many traits which govern the display of text.
// - `std::fmt::Debug`: Uses the `{:?}` marker. Formats text for debugging purposes.
// - `std::fmt::Display`: Uses the `{}` marker. Formats text in a more elegant, user friendly fashion.

// Implementing the `std::fmt::Display` trait automatically implements the `ToString` trait which allows us to convert the type to a `String`.

// ### 1.2.1 Debug

// All types which want to use `std::fmt` formatting traits require an implementation to be printable.
// Automatic implementations are only provided for types such as in the standard library. All others must be manually implemented somehow.

// The `std::fmt::Debug` trait makes this very straightforward. All types can "derive" (automatically create) the `std::fmt::Debug` implementation.
// This is not true for `std::fmt::Display` which requires a manual implementation.

fn debug_formatting() {
    // This struct cannot be printed either with `std::fmt::Display` (`{}`) or with `std::fmt::Debug` (`{:?}`) because it does not implement either trait.
    struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation required to make this struct printable with `std::fmt::Debug`
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // All standard library types are automatically printable with `{:?}` too.

    // Putting a struct inside of another struct...
    #[derive(Debug)]
    struct Deep(DebugPrintable);

    println!("{:?}", DebugPrintable(7)); // DebugPrintable(7)
    println!("{:?}", Deep(DebugPrintable(7))); // Deep(DebugPrintable(7))

    // Printing with `{:?}` is similar to `{}`
    println!("{:?} months in a year.", 12); // 12 months in a year.
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    ); // "Christian" "Slater" is the "actor's" name.

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    // Rust also provides "pretty printing" with {:#?} which is more verbose but easier to read.
    println!(
        "{:#?}",
        Person {
            name: "Alice",
            age: 30
        }
    ); // Person { name: "Alice", age: 30 }
}

// ## 1.3 Display

// Define a structure for which we want to implement `Display`
struct Structure(i32);

// To use the `{}` marker, the `std::fmt::Display` trait must be implemented for the type
impl std::fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output stream: `f`.
        // Returns `std::fmt::Result` which indicates whether the operation was successful or not.
        // Note that `write!` uses syntax which is very similar to `println!`
        write!(f, "{}", self.0)
    }
}

// Because there is no ideal style for all types, the `std` lib doesn't presume to dicate one and does not implement `Display` for every type (e.g. `Vec<T>`)

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x:{}, y:{}", self.x, self.y)
    }
}

struct ComplexNumber {
    real: f64,
    imag: f64,
}

impl std::fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        } else {
            write!(f, "{} - {}i", self.real, -self.imag)
        }
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl std::fmt::Display for City {
    // `f` is a buffernd this method must write the formatted string into it.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!` is like `format!` but it will write the formatted string into a buffer (the first argument)
        write!(
            f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(),
            lat_c,
            self.lon.abs(),
            lon_c
        )
    }
}

fn display_formatting() {
    println!("{}", Structure(3)); // 3
    println!("{}", Point2D { x: 1.0, y: 2.0 }); // x:1, y:2

    println!(
        "{}",
        ComplexNumber {
            real: 3.0,
            imag: 4.0
        }
    ); // 3 + 4i
    println!(
        "{}",
        ComplexNumber {
            real: 3.0,
            imag: -4.0
        }
    ); // 3 - 4i
}
