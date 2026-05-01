#![allow(dead_code, unused_variables)]

// # 17 macro_rules!

// Rust provides a powerful macro system that allows metaprogramming.
// Macros look like functions with a `!` after their name, but they operate on the code itself rather than on values.

// Instead of generating a function call, macros are expanded into source code that gets compiled with the rest of the program.

// However, unlike macros in C and other languages, Rust macros are expanded into abstract syntax trees, rather than string preprocessing, so you don't get unexpected precedence bugs.

// Macros are created using the `macro_rules!` syntax.

fn main() {
    showcase_macro_rules_syntax();
    showcase_macro_rules_designators();
    showcase_macro_rules_overload();
    showcase_macro_rules_repeat();
    showcase_dsl();
    showcase_variadic_interface();
}

// This is a simple macro named `say_hello`
macro_rules! say_hello {
    // `()` indicates that the macro takes no arguments
    () => {
        // The macro will expand into the contents of this block
        println!("Hello!");
    };
}

fn showcase_macro_rules_syntax() {
    // We can call the macro like this:
    say_hello!();
    // This will expand into:
    // println!("Hello!");
}

// Macros are useful becuase:
// 1. They allow you to "Don't Repeat Yourself". There are many cases where you may want similar functionality in multiple places with different types.
// 2. Domain-specifc languages. Macros allow you to define special syntax for special purposes
// 3. Variadic interfaces. Sometimes you want to define an interface that takes an varaible number of arguments. An example of this is the `println!` macro, which can take any number of arguments, dependeing on the format string.

// ## 17.1 Syntax

// ### 17.1.1 Patterns and Designators

// The arguments of a macro are prefixed by a dollar sign `$` and type annotated with a desginator.

macro_rules! create_function {
    // This macro takes an argument of desginator `ident` and creates a function name `$func_name`
    // The `ident` designator is used for variable/function names
    ($func_name:ident) => {
        fn $func_name() {
            // The stringify! macro converts an `ident` into a string literal
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

// Create functions named `foo` and `bar` with the above macro
create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints it as a string along with its reult
    // The `expr` designator is used for expressions
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

fn showcase_macro_rules_designators() {
    // Call the functions created by the `create_function` macro
    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;
        x * x + 2 * x - 1
    });
}

// These are some of the avialable designators:
// - block
// - expr: expressions
// - ident: variable/function names
// - item
// - literal: literal constants
// - pat: patterns
// - path
// - stmt: statements
// - tt: token trees (any sequence of tokens)
// - ty: types
// - vis: visibility qualifiers (e.g., `pub`, `pub(crate)`, etc.)

// ### 17.1.2 Overload

// Macros can be overloaded to accept different combinations of arguments. In that regard, `macro_rules!` can work similar to a match block:

// `test!` will compare `$left` and `$right` in different ways depending on how you invoke it:
macro_rules! test {
    // Arguments don't need to be separated by a comma, any template can be used!
    ($left:expr; and $right:expr) => {
        println!(
            "{:?} AND {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left && $right
        );
    }; // each arm must end with an semicolon
    ($left:expr; or $right:expr) => {
        println!(
            "{:?} OR {:?} is {:?}",
            stringify!($left),
            stringify!($right),
            $left || $right
        );
    };
}

fn showcase_macro_rules_overload() {
    test!(1i32 + 1 == 2i32; and 2i32 * 2 == 4i32);
    test!(true; or false);
}

// ### 17.1.3 Repeat

// Macros can use `+` in the argument list to indicate that an argument may repeat at least once, or `*`, to indicate that the argument may repeat zero or more times.

// In the following example, surrounding matcher with `$(...),+` will match one or more expression, separated by commas. Also note that semicolon is optional on the last case

// `find_min!` will calculate the minimum of any number of arguments:
macro_rules! find_min {
    // Base case
    ($x:expr) => ($x);
    // `$x` followed by atleast one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn showcase_macro_rules_repeat() {
    println!("Minimum of 1, 2, 3 is {}", find_min!(1u32, 2u32, 3u32));
    println!(
        "Minimum of 8, 4, 7, 5, 6 is {}",
        find_min!(8u32, 4u32, 7u32, 5u32, 6u32)
    );
}

// ## 17.2 DRY (Don't Repeat Yourself)

// Macros allow writing DRY code by factoring out the common parts of functions and/or test suites.
// Here's an example that implements and tests the `+=`, `*=nd `-=` operators on `Vec<T>`:

use std::ops::{Add, Mul, Sub};

macro_rules! assert_equal_len {
    // The `tt` (token tree) designator is used for operators and tokens
    ($a:expr, $b:expr, $func:ident, $op:tt) => {
        assert!(
            $a.len() == $b.len(),
            "{:?}: dimension mismatch: {:?} {:?} {:?}",
            stringify!($func),
            ($a.len(),),
            stringify!($op),
            ($b.len(),)
        );
    };
}

macro_rules! op {
    ($func:ident, $bound:ident, $op:tt, $method:ident) => {
        fn $func<T: $bound<Output = T> + Copy>(xs: &mut Vec<T>, ys: &Vec<T>) {
            assert_equal_len!(xs, ys, $func, $op);
            for (x, y) in xs.iter_mut().zip(ys.iter()) {
                *x = $bound::$method(*x, *y);
                // *x = x.$method(*y);
            }
        }
    };
}

// Implement `add_assign`, `mul_assign` and `sub_assign` functions
op!(add_assign, Add, +=, add);
op!(mul_assign, Mul, *=, mul);
op!(sub_assign, Sub, -=, sub);

mod test {
    macro_rules! test {
        ($func:ident, $x:expr, $y:expr, $z:expr) => {
            #[test]
            fn $func() {
                for size in 0usize..10 {
                    let mut x: Vec<_> = iter::repeat($x).take(size).collect();
                    let y: Vec<_> = iter::repeat($y).take(size).collect();
                    let z: Vec<_> = iter::repeat($z).take(size).collect();

                    super::$func(&mut x, &y);

                    assert_eq!(x, z);
                }
            }
        };
    }

    // Test `add_assign`, `mul_assign` and `sub_assign` functions
    test!(add_assign, 1i32, 2i32, 3i32);
    test!(mul_assign, 2i32, 3i32, 6i32);
    test!(sub_assign, 5i32, 3i32, 2i32);
}

// ## 17.3 DSL (Domain Specific Languages)

// A DSL is a mini "language" embedded in a Rust macro. It is completely valid Rust because the macro system expands into normal Rust constructs,
// but it looks like a small language. This allows you to define concise or intuitive syntax for some special functionality(within bounds)

// Suppose that I want to define a little calculator API. I would like to supply an expression and have the output printed to console.

macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be unsigned integers
            println!("{} = {}", stringify!($e), val);
        }
    }
}

fn showcase_dsl() {
    calculate! {
        eval 1 + 2  // Note: eval is not a Rust keyword, it's just part of the DSL syntax
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}

// ## 17.4 Variadic Interfaces

// A variadic interfaace takes an arbitrary number of arguments. For eaxmple, println! can take an arbitrary number of arguments, as determined by the format string.

macro_rules! more_calculate {
    // The pattern for a single `eval`
    (eval $e:expr) => {
        {
            let val: usize = $e; // Forces types to be integers
            println!("{} = {}", stringify!($e), val);
        }
    };

    // Decompose multiple `eval`'s recursively
    (eval $e:expr, $(eval $es:expr),+) => {{
        more_calculate! { eval $e }
        more_calculate! { $(eval $es),+ }
    }}
}

fn showcase_variadic_interface() {
    more_calculate! {
        eval 1 + 2,
        eval (1 + 2) * (3 / 4),
        eval 5 * 6 - 7
    }
}
