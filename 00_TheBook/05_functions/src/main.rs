//  The `fn` keyword allows you to declare new functions.
//  Convention in rust is to use snake_case
//  The function could have been defined after main as well.
fn say_hello() {
    println!("Hello!");
}

//  Functions can also be declared to accept parameters
//  Note that both parameters have type annotations.
//  -> marks the return type of the function
fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

//  The main function is the entry point of your rust program.
fn main() {
    say_hello(); //  Calling a function

    let sum = add(2, 5);
    println!("The sum is {}", sum);
    //  Statements vs Expressions
    //  -------------------------

    // Function bodies are made up of a series of statements optionally ending in an expression.
    // Statements are instructions that perform some action and do not return a value.
    // Expressions on the other hand evaluate to a resulting value

    // let x = 5; //  <-- This is a statement

    // let y = (let z = 6)  will return an error (let z = 6) does not return anything
    //  In some languages you can write x = y = 6 and have both x and y be 6. This is not the case in Rust.

    //  5 + 6 is an expression that evaluates to 11
    //  Calling a function is an expression. Calling a macro is also an expression.
    //  Creating a block scope is also an expression
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is {}", y)
    //  This block evaluates to 4. The value of x + 1 gets bound to y. Note that x + 1 does not end with a semicolon.
    //  Expressions do not include semicolons. If you add the semicolon, this will become a statement (which will not return a value)
}
