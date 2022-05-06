fn main() {
    //  Variables are immutable by default. You can opt out of this by explicitly using the `mut` keyword
    //  When a variable is immutable, once a value is bound to a name, you can't change that value.

    // let x = 5;   <-- This makes x an immutable entity and the compiler will throw an error when x is assigned 6
    let mut x = 5; //  <-- `mut` keyword makes x a mutable variable, allowing it's value to be reassigned
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is {}", x);

    //  constants are declared using the `const` keyword and require type annotations. conventionally named like `I_AM_A_CONSTANT`
    //  You can't use `mut` with  constants. They're always immutable.
    //  Constants can only be assigned to a constant expression, not result of some runtime calculation.
    //  Constants are valid for the entire time a program runs, within the scope they're declared in. They can be declared globally.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "There are {} seconds in three hours",
        THREE_HOURS_IN_SECONDS
    );

    //  You can declare a new variable with the same name as another. This is called shadowing.
    //  Rust will refer to the new variable here on out. The new variable shadows the original.
    //  Re-declaring with `let` allows you to shadow instead of reassigning values
    let x = 7; //  Creates a new variable with the same name x that shadows the previous x and has a value of 7
    println!("The value of x is {}", x);
    //  x will continue to shadow as long as it is in scope.

    //  Shadowed variables are only valid in their scopes.
    {
        let x = 8;
        println!("The value of x is {}", x);
    }
    println!("The value of x is {}", x);
}
