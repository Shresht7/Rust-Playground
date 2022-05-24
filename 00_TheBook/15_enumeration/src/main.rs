//  Enumerations allow you to define a type by enumerating all its possible variants.
//  enumerations are declared using the enum keyword

//  The IpAddrKind enum lists the two kind of IP Addresses
enum IpAddrKind {
    V4,
    V6,
}

#[allow(unused_variables)]
fn instantiate_ip() {
    //  We can create instances of each of the two variants like so
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;
    //  Note that the variants of the enum are namespaced under its identifier.

    //  A function can take any IpAddrKind, and we can call it with any variant
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

#[allow(unused_variables)]
fn route(ip_kind: IpAddrKind) {}

//  We can also store data inside each enum variant
enum IpAddr {
    V4(String),
    V6(String),
}

#[allow(unused_variables)]
fn enums_with_data() {
    //  Here we attach data to each variant of the enum directly.
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

//  Each variant can have different types and amounts of associated data.
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//  Quit has no data associated with it at all
//  Move has named fields like a struct
//  Write includes a single String
//  ChangeColor includes three i32 values.

//You can also define methods for enums!
#[allow(dead_code)]
impl Message {
    fn call(&self) {
        println!("Hello!")
    }
}
fn main() {
    instantiate_ip();
    enums_with_data();
    matchControlFlow();
}

//  The Option enum is a very special enum defined by the standard library
//  it encodes a very common scenario in which a value could be something or it could be nothing.
//  Expressing this concept in terms of the type system instead of a null value allows the compiler to check if you've handled all cases
//  Option is defined as
//  ```rs
//  enum Option<T> {
//      None,
//      Some<T>
//  }
//  ```
//  When we have a `Some` value we know that a value is present and the value is held within the `Some`.
//  When we have a `None` value we know that the same thing is null. We don't have a valid value
//  The compiler will force us to handle the None case!

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
