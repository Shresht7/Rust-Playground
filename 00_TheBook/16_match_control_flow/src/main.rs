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

fn main() {
    matchControlFlow()
}
