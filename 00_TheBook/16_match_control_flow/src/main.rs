//  Rust has an extremely powerful control flow operator called match that allows you to compare a value against a series of patterns and execute the code based on which pattern matches
//  Patterns can be made up of literal values, names, wildcards, and many other things.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
    //  First we list the match keyword followed by an expression, which in this case is the value coin.
    //  This seems very similar to an expression used with if, but there's a big difference. With if, the expression
    //  needs to return a boolean, but here it can be any type.
    //  Next are the match arms, An arm has two parts: a pattern and some code. The first arm here has a pattern that is the
    //  value Coin::Penny and then the => operator that separates the pattern and the code to run.
    //  The code in this case is just the value 1.
    //  Each arm is separated from the next with a comma.
    //  When the match expression executes, it compares the resulting value against the pattern of each arm, in order.
    //  if the pattern doesn't matches the value, execution continues to the next arm.
}

fn matchControlFlow() {}

// Another useful feature of match arms is that they can bind values. This is how we can extract values out from an enum variant.

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

// Matching with Option<T>

// This is how we handle NULL values in Rust.

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_option() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// Combined, match and enums are extremely useful in many situations. You'll see them come up a lot in Rust code.

// One other thing about match; They are exhaustive. i.e. they must cover all possible cases.
// We can use catch all patterns, when we want an expression to apply for any kind of match.

fn catch_all() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    // The first two arms, the patterns are the literal values of 3 and 7. The last arm covers all other possible
    // values. We've chosen to name it `other`. If we don't want to capture the pattern in a variable we can use _ to
    // indicate that we are not interested in storing the value as a variable.
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn main() {
    matchControlFlow();
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
