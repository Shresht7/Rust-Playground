#![allow(unused)]

// # 7. Expressions

// A Rust program is (mostly) made up of a series of statements and expressions.

// A statement is an instruction that performs some action and does not return a value. It ends with a semicolon (`;`).
// An expression evaluates to a resulting value. Expressions do not include ending semicolons.

fn main() {
    showcase_expressions();
}

fn showcase_expressions() {
    let x = 5; // statement

    let y = {
        let x_squared = x * x; // statement
        let x_cubed = x_squared * x; // statement

        // The final expression in the block is returned and assigned to `y`
        x_cubed + x_squared + x // expression
    };

    let z = {
        2 * x; // The semicolon suppresses the return value of this expression, so the block evaluates to `()`
    };

    println!("The value of z is: {:?}", x);
    println!("The value of y is: {:?}", y);
    println!("The value of z is: {:?}", z);
}
