// Tests are Rust functions that verify that the non-test code is functioning in the expected manner.
// The bodies of test functions typically perform these three actions:
// - Setup any needed data or state
// - Run the code you want to test
// - Assert that the results are what you expect

/// The function that we are going to test.
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

fn main() {}

// The tests module ia regular module just with the #[cfg(test)] attribute
#[cfg(test)]
mod tests {
    use super::*;

    #[test] // This attribute indicates that this is a test function
    if it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4); // The assert_eq! macro asserts that the result is actually equal to 4 or it panics
    }

    // We can have non-test functions in this module too; to perform common operations. Unless they have
    // a #[test] attribute, they will not be treated as tests.

    #[test]
    fn another() {
        panic("Make this test fail");
    }

    // Macros
    // assert!: whey ou want to ensure that some condition in a test is true.

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        }
        let smaller = Rectangle {
            width: 5,
            height: 1,
        }
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        }
        let smaller = Rectangle {
            width: 5,
            height: 1,
        }
        assert!(!smaller.can_hold(&larger));
    }

    // Equality can be tested with the `assert_eq` and `assert_ne` macros
    #[test]
    fn equality() {
        assert_eq!(add(2, 3), 5);
    }
    // Under the hood, `assert_eq` and `assert_ne` macros use the operations `==` and `!=`. When the
    // assertion fails, these macros print their arguments using debug formatting, which means the 
    // values being compared must implement the `PartialEq` and `Debug` traits. You may need to implement
    // them for your custom structs

    // You can add a custom message to be printed with the failure message as option arguments to the
    // `assert!`, `assert_eq!` and `assert_ne!` macros.
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"), "Greeting did not contain name, value was `{result}`"
        );
    }

    // In addition to checking return values, it's import to check that our code handles error conditions
    // as we expect.
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    // You can also provide a reasoning for why should this test panics
    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100_with_message() {
        Guess::new(200);
    }

    // We can also write tests that use Result<T, E>.
    #[test]
    fn it_works_result() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // Writing tests like this enables you to use the question mark operator 
    // in the bodies of tests, which can be a convenient way to write tests.
    // Since Err is not panics, we cannot use the should_panic annotation here.
    // instead use `assert!(value.is_err())`
}

// We run all the tests using the `cargo test` command.
