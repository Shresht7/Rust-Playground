#![allow(dead_code)]
// # Error Handling

// Error handling is the process of handling the possiblity of failure. For example, failing to read a file and the continuiing to use that _bad_ input would
// clearly be problematic.Noticing and explicitly managing those errors saves the rest of the program from various pitfalls.

// There are various ways to deal with errors in Rust, and they all have more or less subtle differences and different use cases.

fn main() {
    panic::showcase_panic();
    abort::showcase_abort();
    unwind::showcase_unwind();
    option::showcase_option();
    showcase_option_chaining();
    map::showcase_option_combinators();
    and_then::showcase_option_combinators();
    or::showcase_option_or();
    or_else::showcase_option_or_else();
    get_or_insert::showcase_option_get_or_insert();
    get_or_insert_with::showcase_option_get_or_insert_with();
}

// ## `panic`

// The simplest error handling mechanism we will see is `panic`. It prints an error message, starts unwinding the stack, and usually exits the program.

mod panic {
    fn drink(beverage: &str) {
        // You shouldn't drink too many sugary beverages
        if beverage == "lemonade" {
            panic!("Aaagh, too sugary!");
        }
        println!("Some refreshing {} is all I need.", beverage);
    }

    pub fn showcase_panic() {
        drink("water");
        drink("lemonade");
        drink("still water");
        // The first call to drink works. The second panics and thus the third is never called.
    }
}

// ## abort and unwind

// Different code paths can be conditionally compiled based on the panic setting. The current values available are `unwind` and `abort`

mod abort {
    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            if cfg!(panic = "abort") {
                println!("This is not your party. Run!!!");
            } else {
                panic!("Spit it out!!!");
            }
        } else {
            println!("Some refreshing {} is all I need.", beverage);
        }
    }

    pub fn showcase_abort() {
        drink("water");
        drink("lemonade");
        drink("still water");
    }
}

mod unwind {
    #[cfg(panic = "unwind")]
    fn ah() {
        println!("Spit it out!!!");
    }

    #[cfg(not(panic = "unwind"))]
    fn ah() {
        println!("This is not your party. Run!!!");
    }

    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            ah();
        } else {
            println!("Some refreshing {} is all I need.", beverage);
        }
    }

    pub fn showcase_unwind() {
        drink("water");
        drink("lemonade");
        drink("still water");
    }

    // The panic stragegy can be set from the command line by using `abort` or `unwind`
    // rustc lemonade.rs -C panic=abort
}

// ## 18.3 Option & unwrap

// An enum called `Option<T>` in the `std` library is used when absence is a possibility. It manifests itself as one of two "options":
// - `Some(T)`: An element of type `T` was found
// - `None`: No element was found

// These cases can either be explicitly handled via `match` or implictly with `unwrap`. Implicit handling will either return the inner element or `panic`

// Note that it's possible to manually cutomize `panic`, with expect, but `unwrap` otherwise leaves us with a less meaningful output than explicit handling.

mod option {
    // The adult has seen it all, and can handle any drink well.
    // All drinks are handled explicitly using `match`
    fn give_adult(drink: Option<&str>) {
        // Specify a course of action for each case
        match drink {
            Some("lemonade") => println!("Yuck! Too sugary."),
            Some(inner) => println!("{}? How nice.", inner),
            None => println!("No drink? Oh well."),
        }
    }

    // Others will `panic` before drinking sugary drinks. All drinks are handled implicitly usin `unwrap`
    fn drink(drink: Option<&str>) {
        // `unwrap` returns a `panic` when it receives a `None`
        let inside = drink.unwrap();
        if inside == "lemonade" {
            panic!("AAAaaaa!!!!");
        }
        println!("I love {}s!!!", inside);
    }

    pub fn showcase_option() {
        let water = Some("water");
        let lemonade = Some("lemonade");
        let void = None;

        give_adult(water);
        give_adult(lemonade);
        give_adult(void);

        let coffee = Some("coffee");
        let nothing = None;

        drink(coffee);
        drink(nothing);
    }
}

// Unpacking options with ?

// You can unpack `Option`s by using `match` statements, but it's often easier to use the `?` operator.
// If `x` is an `Option` then evaluating `x?` will return the underlying value if `x` is `Some`, otherwise it will terminate whatever function is being executed and return `None`.

fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // If `current_age` is `None`, this returns `None`
    // if `current_age` is `Some`, the inner `u8` value + 1 gets assigned to `next_age`
    let next_age: u8 = current_age? + 1;
    Some(format!("Next year I will be {}", next_age))
}

// You can chain many `?`s together to make your code much more readable.

struct Person {
    job: Option<Job>,
}

#[derive(Debug, Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // Gets the area code of the phone number of the person's job, if it exists
    fn work_phone_area_code(&self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator
        self.job?.phone_number?.area_code
    }
}

fn showcase_option_chaining() {
    let person_with_job = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(123),
                number: 4567890,
            }),
        }),
    };

    let person_without_job = Person { job: None };

    assert_eq!(person_with_job.work_phone_area_code(), Some(123));
    assert_eq!(person_without_job.work_phone_area_code(), None);
}

// ## 18.3.2 Combinators: map

// `match` is avlid method for handling `Option`s. However, you may eventually find heavy usage tedious, especially with operations only valid with an input.
// In these cases, combinators can be used to manage the control flow in a modular fashion.

// `Option` has a builtin method called `map()`, a combinator for the simple mapping of `Some -> Some` and `None -> None`.
// Multiple `map()` call can be chained together for even more flexibility.

mod map {
    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }

    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    // Peeling food. If there isn't any then return None. Otherwise return the peeled foold
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }

    // Chopping food. If there isn't any then return None. Otherwise return the chopped foold
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }

    // Cooking food. Here, we showcase `map()` instead of match for case handling
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
    }

    // A function to peel, chop and cook food all in sequence.
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }

    // Check whether there's food or not before tyring to eat it!
    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mmm, delicious {:?}", food),
            None => println!("Oh no, nothing to eat!"),
        }
    }

    pub fn showcase_option_combinators() {
        let apple = Some(Food::Apple);
        let carrot = Some(Food::Carrot);
        let potato = None;

        let cooked_apple = process(apple);
        let cooked_carrot = process(carrot);
        let cooked_potato = process(potato);

        eat(cooked_apple);
        eat(cooked_carrot);
        eat(cooked_potato);
    }
}

// ## 18.3.3 Combinators: and_then

// `map()` was described as a chainable way to simplify match statements. However, using map on a function that returns an Option<T> results
// in nested `Option<Option<T>>`. Chaining multiple calls together can the become confusing.That's where antoher combinator called `and_then()` comes in,
// known in some languages as flatmap.

// `and_then()` calls its function with the wrapped value and returns the result. if the Option is None, it returns None instead.

mod and_then {
    #[derive(Debug)]
    enum Food {
        CordonBleu,
        Steak,
        Sushi,
    }

    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
    }

    //We don't have the ingredients to make Sushi
    fn have_ingredients(food: Food) -> Option<Food> {
        match food {
            Food::Sushi => None,
            _ => Some(food),
        }
    }

    // We have th recipe for everything except Cordon Bleu
    fn have_recipe(food: Food) -> Option<Food> {
        match food {
            Food::CordonBleu => None,
            _ => Some(food),
        }
    }

    // To make a dish, we need both the recipe and the ingredients.
    // We can represent the logic with a chain of matches
    fn cookable_v1(food: Food) -> Option<Food> {
        match have_recipe(food) {
            None => None,
            Some(food) => have_ingredients(food),
        }
    }

    // This can be convineiently be rewritten more compactly with `and_then()`
    fn cookable_v3(food: Food) -> Option<Food> {
        have_recipe(food).and_then(have_ingredients)
    }

    // Otherwise we'd need to `flatten` and `Option<Option<Food>>`
    fn cookable_v2(food: Food) -> Option<Food> {
        have_recipe(food).map(have_ingredients).flatten()
    }

    fn eat(food: Food, day: Day) {
        match cookable_v3(food) {
            Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
            None => println!("Oh no. We don't get to eat on {:?}?", day),
        }
    }

    pub fn showcase_option_combinators() {
        let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);
        eat(cordon_bleu, Day::Monday);
        eat(steak, Day::Tuesday);
        eat(sushi, Day::Wednesday);
    }
}

// ### 18.3.4 Unpacking options and defaults

// There is more than one way to unpack an Option and fall back on a default if it is None.

// or() is chainable, evaluates eagerly, keeps emtpy value intact
// Note that because `or`'s arguments are evaluated eagerly, the varaible passed to `or` is moved.

mod or {
    #[derive(Debug)]
    enum Fruit {
        Apple,
        Orange,
        Banana,
        Kiwi,
        Lemon,
    }

    pub fn showcase_option_or() {
        let apple = Some(Fruit::Apple);
        let orange = Some(Fruit::Orange);
        let no_fruit: Option<Fruit> = None;

        let first_available_fruit = no_fruit.or(orange).or(apple);
        println!("First available fruit: {:?}", first_available_fruit);
        // This will print "First available fruit: Some(Orange)"
    }
}

// or_else() is chainiable, evaluates lazily, keeps empty value intact

mod or_else {
    #[derive(Debug)]
    enum Fruit {
        Apple,
        Orange,
        Banana,
        Kiwi,
        Lemon,
    }

    pub fn showcase_option_or_else() {
        let no_fruit: Option<Fruit> = None;
        let get_kiwi_as_fallback = || {
            println!("No fruit found, providing a kiwi as a fallback.");
            Some(Fruit::Kiwi)
        };
        let get_lemon_as_fallback = || {
            println!("No fruit found, providing a lemon as a fallback.");
            Some(Fruit::Lemon)
        };

        let first_available_fruit = no_fruit
            .or_else(get_kiwi_as_fallback)
            .or_else(get_lemon_as_fallback);

        println!("First available fruit: {:?}", first_available_fruit);
    }
}

// get_or_insert() evaluates eagerly, modfies empty value in place

// To make sure an Option contains a value, we can use `get_or_insert()` to modify it in place with a fallback value.

mod get_or_insert {
    #[derive(Debug)]
    enum Fruit {
        Apple,
        Orange,
        Banana,
        Kiwi,
        Lemon,
    }

    pub fn showcase_option_get_or_insert() {
        let mut my_fruit: Option<Fruit> = None;
        let apple = Fruit::Apple;
        let first_available_fruit = my_fruit.get_or_insert(apple);
        println!("First available fruit: {:?}", first_available_fruit);
        println!("My fruit is now: {:?}", my_fruit);
    }
}

// get_or_insert_with() evaluates lazily, modifes empty value in place

// Instead of explicitly providing a value to fall back on, we can pass a closure on `get_or_insert_with`

mod get_or_insert_with {
    #[derive(Debug)]
    enum Fruit {
        Apple,
        Orange,
        Banana,
        Kiwi,
        Lemon,
    }

    pub fn showcase_option_get_or_insert_with() {
        let mut my_fruit: Option<Fruit> = None;
        let get_lemon_as_fallback = || {
            println!("No fruit found, providing a lemon as a fallback.");
            Fruit::Lemon
        };
        let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
        println!("First available fruit: {:?}", first_available_fruit);
        println!("My fruit is now: {:?}", my_fruit);

        // If the Option has a value, it is left unchanged, and the closure is not invoked
        let mut my_apple = Some(Fruit::Apple);
        let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
        println!("First available fruit: {:?}", should_be_apple);
        println!("My fruit is now: {:?}", my_apple);
    }
}

// 18.4 Result

// `Result` is a richer version of the `Option` type that describes possible _error_ instead of possible _absence_.

// That is, `Result<T, E>` could have one of two outcomes
// - `Ok(T)`: An element `T` was found
// - `Err(E)`: An error `E` was found

// By convention, the expected outcome is `Ok` while the unexpected outcome is `Err`

// Like `Option`, `Result` has many methods associated with it. `unwrap()`, for example, yields the element `T` or panics.
// For case handling, there are many combinators between `Result` and `Option` that overlap.

// In working with Rust, you will likely encounter methods that return `Result` type, such as the `parse()` method.
// It might not always be possible to parse a string into the other type, so `parse()` returns a `Result` indicating possible failure.

fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
    // Let's try using `unwrap()` to get the number out. Will it bite us?
    let first_number = first_number_str.parse::<i32>().unwrap();
    let second_number = second_number_str.parse::<i32>().unwrap();
    first_number * second_number
}

fn showcase_result() {
    let twenty = multiply("10", "2");
    println!("double is {}", twenty);

    let tt = multiply("t", "2");
    println!("double is {}", tt);
}

// In the unsuccessful case, parse() leaves us with an error for unwrap() to panic on.
// Additionally, the panic exits our program and provides an unplesant error message.
// To improve the quality of our error message, we should be more specfic about the return type and consider explictily handling the error.

// ## Using Result in main

// The Result type can also be the return type of the main function if specified explicitly.

// ## map for Result

mod result_map {
    use std::num::ParseIntError;

    // With the return type rewritten, we use pattern matching without unwrap()
    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        match first_number_str.parse::<i32>() {
            Ok(first_number) => match second_number_str.parse::<i32>() {
                Ok(second_number) => Ok(first_number * second_number),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("double is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn showcase_result_map() {
        // This still presents a reasonable answer
        let twenty = multiply("10", "2");
        print(twenty);

        // The following now provides a much more helpful error message
        let tt = multiply("t", "2");
        print(tt);
    }

    // Luckily Option's map, and_then, and many other combinators are also implemented for Result.

    fn multiply_v2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }
}

// ## 18.4.2 Aliases for Result

// Recall that Rust allows us to create aliases. Conveniently, we can define one for the specific Result in question.
// At a module level, creating aliases can be particularly helpful. Errors found in a specific module often have the same Err type,
// so a single alias can succintly define all assocaited Results. This is so useful that the std library even supplies one: std::io::Result!

mod result_aliases {
    use std::num::ParseIntError;

    // Define a generic alias for a `Result` with the error type ParseIntError
    type AliasedResult<T> = Result<T, ParseIntError>;

    // Use the above alais to refer to our specific Result type
    fn multiply(first_number_str: &str, second_number_str: &str) -> AliasedResult<i32> {
        first_number_str.parse::<i32>().and_then(|first_number| {
            second_number_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    // Here, the alias again allows us to save some space
    fn print(result: AliasedResult<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn showcase_result_aliases() {
        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }
}

// ### 10.4.3 Early Returns

// Another way to deal with this case analysis is to use a combination of match statements and early returns
// That is, we can simply stop executing the function and return the error if one occurs.For some, this form of code can be easier to both read and write.

mod early_return {
    use std::num::ParseIntError;

    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = match first_number_str.parse::<i32>() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        let second_number = match second_number_str.parse::<i32>() {
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        Ok(first_number * second_number)
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn showcase_early_return() {
        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }
}

// Introducing ?

// Sometimes we just want the simplicity of unwrap without the possibility of a panic.
// Until now, unwrap has forced us to nest deeper and deeper when what we really want was to get the variable out.
// This is exactly the purpose of ?

// Upon finding an Err, there are two valid actions to take:
// 1. panic! which we already decided to try to avoid if possible
// 2. return because an Err means it cannot be handled

// `?` is almost exactly equivalent to an unwrap which returns instead of panicking on an Err.

mod result_question_mark {
    use std::num::ParseIntError;

    fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = first_number_str.parse::<i32>()?;
        let second_number = second_number_str.parse::<i32>()?;
        Ok(first_number * second_number)
    }

    fn print(result: Result<i32, ParseIntError>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn showcase_result_question_mark() {
        print(multiply("10", "2"));
        print(multiply("t", "2"));
    }
}

// ## 18.5 Multiple error types

// The previous examples have always been very convenient; Result interact with other Result and Option interact with other Option.

// Sometimes an Option needs to interact with a Result, or a `Result<T, Error1>` needs to interact with a `Result<T, Error2>`.
// In those cases, we want to manage our different error types in a way that makes them composable and easy to interact with.

// In the following code, two instances of unwrap generate different error types. Vec::first returns an Option, while parse::<i32> returns a Result<i32, ParseIntError>

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn showcase_multiple_error_types() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {}", double_first(strings));
    // Error 2: the first element is not a number
}

// ## 18.5.1 Pulling Results out of Options

// The most basic way of handing mixed error types is to just embed them in each other

mod mixed {
    use std::num::ParseIntError;

    fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
        vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n))
    }

    pub fn showcase_mixed() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        println!("The first doubled is {:?}", double_first(numbers));

        println!("The first doubled is {:?}", double_first(empty));
        // Error 1: the input vector is empty

        println!("The first doubled is {:?}", double_first(strings));
        // Error 2: the first element is not a number
    }
}

// There are times when we'll want to stop processing on errors (like with ?) but keep going when the Option is None.
// The transpose function comes in handly to swap the Result and Option

mod transpose {
    use std::num::ParseIntError;

    fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
        let opt = vec.first().map(|first| first.parse::<i32>().map(|n| 2 * n));
        opt.transpose()
    }

    fn showcase_transpose() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        println!("The first doubled is {:?}", double_first(numbers));
        println!("The first doubled is {:?}", double_first(empty));
        println!("The first doubled is {:?}", double_first(strings));
    }
}

// ### 18.5.2 Defining an error type

// Sometimes it simplifies the code to maks all of the different errors with a single type of error.

// Rust allows us to define our own error types. In general, a "good" error type:
// - Represents different errors with the same type
// - Presents nice error messages to the user
// - Is easy to compare with other types
//   - Good Err(EmptyVec)
//   - Bad Err("Please use a vector with at least one element".to_owned())
// - Can hold informatino about the error
//   - Good Err(BadChar(c, position))
//   - Bad Err("+ cannot be used here".to_owned())
// - Composes well with other errors

mod defining_errors {
    type Result<T> = std::result::Result<T, DoubleError>;

    // Define our error types. These may be customized for our error handling cases.
    // Now we will be able to write our own errors, defer to an underlying error implementation, or to do something in between
    #[derive(Debug, Clone)]
    struct DoubleError;

    // Generation of an error is completely separate from how it is displayed.
    // There's no need to be concerned about clutting complex logic with the display style

    // Note that we don't store the extra info about the errors. This means we can't state
    // which string failed to parse without modifying our types to carry that information.
    impl std::fmt::Display for DoubleError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Invalid first item to double")
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            // Change the error to our new type
            .ok_or(DoubleError)
            .and_then(|s| {
                s.parse::<i32>()
                    // Update to the new error type here also
                    .map_err(|_| DoubleError)
                    .map(|i| 2 * i)
            })
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn showcase_defining_errors() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

// ### 18.5.3 Boxing errors

// A way to write simple code while preserving the orignal errors is to `Box` them. The drawback is that the underlying error type is only known at runtime and not statically determined.

// The stdlib helps in boxing our errors by having Box implement conversion from any type that implements the Error trait into the trait object Box<Error> via From

mod boxing_errors {
    // Change the alias to use `Box<dyn std::error::Error>`
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[derive(Debug, Clone)]
    struct EmptyVec;

    impl std::fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Please use a vector with at least one element")
        }
    }

    impl std::error::Error for EmptyVec {}

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        vec.first()
            .ok_or_else(|| EmptyVec.into()) // Converts to Box using Into trait
            .and_then(|s| {
                s.parse::<i32>()
                    .map_err(From::from) // Convert to Box using From::from fn pointer
                    .map(|i| 2 * i)
            })
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn showcase_boxing_errors() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

// ### 18.5.4 Other uses of ?

mod question_mark_other_uses {
    // Change the alias to use Box<dyn std::error::Error>
    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[derive(Debug, Clone)]
    struct EmptyVec;

    impl std::fmt::Display for EmptyVec {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Please use a vector with at least one element")
        }
    }

    impl std::error::Error for EmptyVec {}

    // The same structure as before but rather than chain all Results and Options along, we ? to get the inner value out immediately
    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(EmptyVec)?;
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn showcase_question_mark_other_uses() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}

// ## 18.5.5 Wrapping Errors

// An alternative to boxing errors is to wrap them in your own error type

mod wrapping_errors {
    use std::num::ParseIntError;

    type Result<T> = std::result::Result<T, DoubleError>;

    #[derive(Debug, Clone)]
    enum DoubleError {
        EmptyVec,
        // We will defer to the parse error implementation for their error
        // supplying extra info requires  adding more data to the type.
        Parse(ParseIntError),
    }

    impl std::fmt::Display for DoubleError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match *self {
                DoubleError::EmptyVec => write!(f, "please use a vector with at least one element"),
                // The wrapped error contains addtitional information and is available via the source() method
                DoubleError::Parse(ref e) => write!(f, "Parse error: {}", e),
            }
        }
    }

    impl std::error::Error for DoubleError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match *self {
                DoubleError::EmptyVec => None,
                // The cause is the underlying implementation error type. Is implicitly cast to the trait object &std::error::Error.
                // This works because underlying type already implements the `Error` trait
                DoubleError::Parse(ref e) => Some(e),
            }
        }
    }

    // Implement the conversion from `ParseIntError` to `DoubleError`
    // This will be automatically called by ? if a ParseIntError needs to be converted into a DoubleError
    impl From<ParseIntError> for DoubleError {
        fn from(err: ParseIntError) -> DoubleError {
            DoubleError::Parse(err)
        }
    }

    fn double_first(vec: Vec<&str>) -> Result<i32> {
        let first = vec.first().ok_or(DoubleError::EmptyVec)?;
        // Here we implicitly use the ParseIntError implementation of From in order to create a DoubleError
        let parsed = first.parse::<i32>()?;
        Ok(2 * parsed)
    }

    fn print(result: Result<i32>) {
        match result {
            Ok(n) => println!("The first doubled is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn showcase_wrapping_errors() {
        let numbers = vec!["42", "93", "18"];
        let empty = vec![];
        let strings = vec!["tofu", "93", "18"];

        print(double_first(numbers));
        print(double_first(empty));
        print(double_first(strings));
    }
}
