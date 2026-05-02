#![allow(dead_code)]

// # Testing

// Rust is a programming language that cares a lot about correctness and it includes support for writing software tests within the language itself.

// Testing comes in three styles:
// - Unit testing
// - Doc testing
// - Integration testing

// Also, Rust has support for specifying additional dependencies for tests: https://doc.rust-lang.org/rust-by-example/testing/dev_dependencies.html

fn main() {}

//## 21.1 Unit Testing

// Tests are Rust function that verify that the non-test code is functioning in the expected manner.
// The bodies of test functions typically perform some setup, run the code we want to test, then assert whether the results are what we expect.

// Most unit tests go into a `tests` mod with the `#[cfg(test)]` attribute. Test functions are marked with the `#[test]` attribute

// Tests fail when something in the test function panics. There are some helper macros:
// - assert!(expression) - panics if expression evaluates to false
// - assert_eq!(left, right) and `assert_ne!(left, right)` - testing left and right expressions for equality and inequality respectively.

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this example
#[allow(dead_code)]
pub fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idion: importing names from outer (for mod tests) scope
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail. Please note, that private functions can be tested too!
        assert_eq!(bad_add(2, 3), 5);
    }
}

// Tests can be run with `cargo test`

// Tests and ?

// None of the previous unit test examples had a return type. But in Rust 2018, your unit tests can return `Result<()>`, which lets you use ? in them.
// This can make them much more concise.

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}

// Testing panics

// To check functions that should panic under certain circumstances, use attribute `#[should_panic]`.
// This attribute accepts optional parameter `expected = ` with the text of the panic message. if your function can panic in multiple ways
// it helps make sure your test is testing the correct panic.

// Note: Rust also allows a shorthand form `#[should_panic = "message"]` which works exactly like `#[should_panic(expected = "message")]`.
// Both are valid, the latter is more commonly used and considered more explicit.

pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "Divide-by-zero error")]
    fn test_divide_by_zero() {
        divide_non_zero_result(10, 0);
    }

    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_divide_result_is_zero() {
        divide_non_zero_result(1, 2);
    }
}

// Running specific tests

// To run specific tests, one may specify the test name to `cargo test test_any_panic`

// Tests can be marked with #[ignore] attribute to exclude some tests. Or to run them, cargo test -- --ignored

// ## 21.2 Documentation Testing

// The primary way of documenting a Rust project is through annotating the source code.
// Documentation comments are written in CommonMark Markdown Specification and support code blocks in them.
// Rust takes care about correctness, so these code blocks are compiled and used as documentation tests.

/// First line is a short summary describing the function
///
/// The next lines present detailed documentation. Code blocks starts with triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing a `playground` library crate or using the Playground's Test action:
///
/// ```
/// let result = playground::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn addd(a: i32, b: i32) -> i32 {
    a + b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures"
///
/// # Examples
///
/// ```
/// let result = playground::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero
///
/// ```should_panic
/// // panics on division by zero
/// playground::div(10, 0);
/// ```
pub fn divide(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

// Code blocks in documentation are automatically tested when running the regular cargo test command

// ### Motivation behind the documentation tests

// The main purpose of documentation tets is to serve as examples that exercise the functionality which is one of the most important guidelines.
// It allows using examples from docs as complete code snippets. But using ? makes compilation fail since main returns unit.
// The ability to hide some source lines from the documentation comes to the rescue: one may write fn try_main() -> Result<(), ErrorType>, hide it
// unwrap it in hidden main.

/// Using hidden `try_main` in doc tests
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compilable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in the doc
/// let result = playground::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # fn main() { // starting main that'll unwrap()
/// #    try_main().unwrap(); // calling the try_main and unwrapping
/// #    // so that test will panic in case of errors
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}

// ## 21.3 Integration Testing

// Unit tests are testing one module in isolation at a time: they're small and can test private code.
// Integration tests are external to your crate and use only it's public interface in the same way any other code would.
// Thier purpose is to test that many parts of your library work together correctly.

// Cargo looks for integration tests in `tests` directory next to `src`

// Each Rust source file in the tests directory is compiled as a separate crate. In order to share code between integration tests, we can make a module
// with public functions, importing and using it within tests.

// ## 21.4 Development Dependencies

// Sometimes there is a need to have dependencies for tests (for examples, or benchmarks) only.
// Such dependencies are added to Cargo.toml in the [dev-dependencies] section. These dependencies are not propagated to other packages which depend on this package.

// One such example is pretty_assertions, which extends standard `assert_eq!` and `assert_ne!` macros, to provide colorful diff.

// ```
// [dev-dependencies]
// pretty_assertions = "1.0"
//```
