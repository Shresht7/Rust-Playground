#![allow(dead_code)]

// # 6. Conversions

// Primitive types can be converted to each other using type-casting with the `as` keyword.

// Rust addresses conversion between custom types (i.e. `struct` and `enum`) by the use of `traits`.
// The generic conversions will use the `From` and `Into` traits.

fn main() {
    showcase_from();
    showcase_into();
    showcase_from_into();
    showcase_try_from_into();
    showcase_to_string();
    showcase_parse();
    showcase_from_str();
}

// ## 6.1 From and Into

//### 6.1.1 From

// The `From` trait allows for a type to define how to create itself from another type, hence providing a very simple mechanism for converting between several types.
// There are numerous implementations of this trait within the standard library for conversion of primitive and common types.

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn showcase_from() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}

//### 6.1.2 Into

// The `Into` trait is simply the reciprocal of the `From` trait. It defines how to convert a type into another type.

// Calling `into()` typically requires us to specify the result type as the compiler is unable to determine this most of the time.

use std::convert::Into;

#[derive(Debug)]
struct Number2 {
    value: i32,
}

impl Into<Number2> for i32 {
    fn into(self) -> Number2 {
        Number2 { value: self }
    }
}

fn showcase_into() {
    let num: Number2 = 30.into();
    println!("My number is {:?}", num);
}

// ### 6.1.3 From and Into are reciprocal traits

// `From` and `Into` are designed to be complementary. We do not need to provide an implementation for both traits.
// If you have implemented the `From` trait for your type, `Into` will call it when neccessary.
// Note, however, that the converse is not true: implementing `Into` for your type will not automatically provide it with an implementation of `From`

#[derive(Debug)]
struct Number3 {
    value: i32,
}

impl From<i32> for Number3 {
    fn from(item: i32) -> Self {
        Number3 { value: item }
    }
}

fn showcase_from_into() {
    let num_from = Number3::from(30);
    println!("My number from is {:?}", num_from);

    let num_into: Number3 = 30.into();
    println!("My number into is {:?}", num_into);
}

// ## 6.2 `TryFrom` and `TryInto`

// Similar to `From` and `Into`, `TryFrom` and `TryInto` are generic traits for converting between types,
// but, unlike `From` and `Into` traits, they are used for falliable conversions, and as such, return `Result`s.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(format!("{} is not an even number", value))
        }
    }
}

fn showcase_try_from_into() {
    let even_number = EvenNumber::try_from(8);
    println!("Even number from: {:?}", even_number); // Even number from: Ok(EvenNumber(8))

    let even_number: Result<EvenNumber, _> = 8.try_into();
    println!("Even number into: {:?}", even_number); // Even number into: Ok(EvenNumber(8))

    let odd_number = EvenNumber::try_from(5);
    println!("Odd number from: {:?}", odd_number); // Odd number from: Err("5 is not an even number")

    let odd_number: Result<EvenNumber, _> = 5.try_into();
    println!("Odd number into: {:?}", odd_number); // Odd number into: Err("5 is not an even number")
}

// ## 6.3 To and From Strings

// ### 6.3.1 Converting to a String

// To convert any type to a `String` is as simple as implementing the `ToString` trait for the type.
// Rather than doing so directly, you should implement the `std::fmt::Display` trait which automatically provides an implementation of `ToString`.

struct Circle {
    radius: f64,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Circle with radius: {}", self.radius)
    }
}

fn showcase_to_string() {
    let circle = Circle { radius: 5.0 };
    let circle_string = circle.to_string();
    println!("{}", circle_string); // Circle with radius: 5
}

// ### 6.3.2 Parsing a String

// It's useful to be able to parse a string into another type. The `FromStr` trait is used for this purpose, and is implemented by many types in the standard library.

// The idiomatic approach to this is to use the `parse` function and either to arrange for type inference or to specify the type to parse using the `turbofish` syntax.

fn showcase_parse() {
    let parsed: i32 = "5".parse().expect("Not a number!");
    println!("Parsed number: {}", parsed); // Parsed number: 5

    let turbo_parsed = "10".parse::<i32>().expect("Not a number!");
    println!("Turbo parsed number: {}", turbo_parsed); // Turbo parsed number: 10

    let sum = parsed + turbo_parsed;
    println!("Sum: {}", sum); // Sum: 15
}

// To obtain this functionality on a user defined type simply implement the `FromStr` trait

impl std::str::FromStr for Circle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(num) => Ok(Circle { radius: num }),
            Err(_) => Err(format!("'{}' is not a valid number", s)),
        }
    }
}

fn showcase_from_str() {
    let circle_str = "5.0";
    match circle_str.parse::<Circle>() {
        Ok(circle) => println!("Parsed circle: {}", circle), // Parsed circle: Circle with radius: 5
        Err(e) => println!("Error parsing circle: {}", e),
    }
}
