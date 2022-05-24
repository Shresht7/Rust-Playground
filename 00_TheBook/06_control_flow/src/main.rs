fn main() {
    //  CONDITIONAL EXPRESSIONS
    //  -----------------------

    //  If expressions allow you to branch your code depending on conditions
    let is_ready = false;

    //  If expressions evaluate a condition (In Rust, the condition must evaluate to a boolean)
    if is_ready {
        //  This branch (or arm) is executed if the condition evaluates to true
        println!("Let's Go!");
    } else {
        //  Otherwise, this branch (or arm) is executed
        println!("Oh noooo!");
    }

    //  You can have multiple conditions by combining if, else and elseif
    //  Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //  If is an expression! There is not ternary operator in Rust
    let condition = true;
    let value = if condition { 10 } else { 0 };
    println!("The value is {}", value);

    //  All arms of an if expression must have the same data type as Rust needs to know the type ahead-of-time (at compile time)

    //  LOOPS
    //  -----

    //  The `loop` keyword executes a block of code forever
    let mut count = 0;
    loop {
        if count > 5 {
            break; //  The break keyword tells program to break out of the loop

        // continue; //    The continue keyword tells the program to skip everything to the next iteration
        } else {
            println!("Here we go again!");
        }
        count += 1;
    }

    //  You can nest loops. break and continue apply to the innermost loop. You can label loops and use labels with break and continue to apply to specific loops.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    //  You can return values from loops!
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    //  While Loops
    //  -----------

    //  You probably don't want to loop indefinitely. So you can use while loops to loop until a condition is valid
    let mut number = 3;

    while number != 0 {
        println!("{}", number);
        number -= 1;
    }

    println!("Liftoff!!!");

    //  For Loops
    //  ---------

    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("Liftoff!!!");
}
