#![allow(unused)]

// # 8. Flow of Control

// An integral part of any programming language are ways to modify the control flow: `if/else`, `for` and `others`

fn main() {
    showcase_if_else();
    showcase_loop();
    showcase_while();
    showcase_for_in();
    showcase_iter();
    showcase_into_iter();
    showcase_iter_mut();
    showcase_match();
    showcase_destructuring();
    showcase_if_let();
    showcase_let_else();
    showcase_while_let();
}

// ## 8.1 `if/else`

// Branching with `if/else` is similar to other languages, but there are some subtle differences.
// For one, the boolean condition does not need to be enclosed in parantheses, and each condition is followed by a block.
// The conditions are expressions, and all branches must return the same type. The `else` block is optional.

fn showcase_if_else() {
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

    // Because `if` is an expression, we can use it in a `let` statement to assign the result of the `if` expression to a variable.
    let big_number = if number < 10 && number > -10 {
        println!(", and is a small number, increase ten-fold");

        // This expression returns a value, and assigns it to `big_number`
        10 * number
    } else {
        println!(", and is a big number, reduce ten-fold");

        // This expression returns a value, and assigns it to `big_number`
        number / 10
    };

    println!("{}{}.", number, big_number);
}

// ## 8.2 `loop`

// Rust provides a `loop` keyword to indicate an infinite loop.
// The `break` keyword can be used to exit a loop at any time, whereas the `continue` keyword can be used to skip the rest of the current iteration

fn showcase_loop() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue; // Skip the rest of the loop when count is 3
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");
            break; // Exit the loop
        }
    }

    // It's possible to `break` or `continue` outer loops when dealing with nested loops. In these cases,
    // the loops must be annotated with some `'label`, and the label must be passed to `break`/`continue` so that it can identify which loop to take action on.
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // break; // This will only break the inner loop

            // This will break the outer loop
            break 'outer;
        }

        // This will never be reached
        println!("This will never be printed");
    }

    println!("Exited the outer loop");

    // One of the uses of a `loop` is to retry an operation until it succeeds.
    // If the operation returns a value though, you might need to pass it to the rest of the code: put it after the break, and it will
    // be returned by the `loop` expression.

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // This will return 20 from the loop expression
        }
    };

    println!("The result is {}", result); // The result is 20
}

// ## 8.3 `while`

// The `while` keyword can be used to run a loop while a condition is true.

fn showcase_while() {
    // A counter variable
    let mut n = 1;

    // Loop while `n` is less than or equal to 100
    while n <= 100 {
        // If n is divisible by 15, print "fizzbuzz"
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment the counter
        n += 1;
    }
}

// ## 8.4 `for` and `range`

// The `for in` construct can be used to iterate through an `Iterator`.
// One of the easiest ways to create an iterator is to use the range notation `a..b`.
// This yields values from `a` (inclusive) to `b` (exclusive) in steps of one.

fn showcase_for_in() {
    for n in 0..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Alternatively, `a..=b` can be used for a range that is inclusive on both ends.
    for n in 0..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

// ### 8.4.2 for and iterators

// The `for in` construct is able to interact with an `Iterator` in several ways.
// By default, the `for` loop will apply the `into_iter` function to the collection.
// However, this is not only the only means of converting collections into iterators.

// `into_iter`, `iter` and `iter_mut` all handle the conversion of a collection into an iterator in different ways,
// by providing different views on the data within.

// #### `iter`

// This borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop.

fn showcase_iter() {
    let names = vec!["Bob", "Framk", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            name => println!("Hi there {}!", name),
        }
    }
}

// #### `into_iter`

// This consumes the collection so that on each iteration the exact data is provided.
// Once the collection has been consumed, it is no longer available for reuse as it has been "moved" within the loop.

fn showcase_into_iter() {
    let names = vec!["Bob", "Framk", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            name => println!("Hi there {}!", name),
        }
    }

    // Error! `names` has been moved into the loop and can no longer be used.
    // println!("names: {:?}", names); // error: use of moved value: `names`
}

// #### `iter_mut`

// This mutably borrows each element of the collection, allowing for the collection to be modified in place.

fn showcase_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names); // names: ["Bob", "Frank", "There is a rustacean among us!"]
}

// ## 8.5 `match`

// Rust provides pattern matching via the `match` keyword, which can be used like a C `switch`.
// The first matching arm is evaluated and all possible values must be covered.

fn showcase_match() {
    let number = 13;

    println!("Tell me about{}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of the match must cover all possible values, so we don't need a catch-all case here
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary); // true -> 1
}

// ### 8.5.1 Destructuring

// A match block can destructure items in a variety of ways

fn showcase_destructuring() {
    showcase_destructuring_tuples();
    showcase_destructuring_arrays();
    showcase_destructuring_enums();
    showcase_destructuring_pointers();
    showcase_destructuring_structs();
    showcase_match_guard();
    showcase_bindings();
    showcase_option_bindings();
}

// #### 8.5.1.1 Tuples

fn showcase_destructuring_tuples() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    // Match can be used to destructure a tuple
    match triple {
        // Destructure the second and third elements if the first element is `0`
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        // Match if the first element is '1' regardless of the values of the second and third elements
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // Match if the third element is `2` regardless of the values of the first and second elements
        (.., 2) => println!("Last is `2` and the rest doesn't matter"),
        // Match if the first element is `3` and the third element is `4` regardless of the value of the second element
        (3, .., 4) => println!("First is `3`, last is `4` and the rest doesn't matter"),
        // .. is used to ignore the rest of the tuple

        // Handle all other cases. _ means don't bind this to any variable,
        _ => println!("It doesn't matter what they are"),
    }
}

// #### 8.5.1.2 Arrays and Slices

// Like tuples, arrays and slices can be destructured this way:

fn showcase_destructuring_arrays() {
    let array = [1, -2, 6];

    match array {
        // Binds the second and third elements to the respective variables if the first element is 0
        [0, y, z] => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),

        // Single value can be ignored with _
        [1, _, z] => println!(
            "First is `1`, `z` is {:?}, and the second element doesn't matter",
            z
        ),

        // You can also bind some and ignore the rest with ..
        [-1, second, ..] => println!(
            "First is `-1`, `second` is {:?}, and the rest doesn't matter",
            second
        ),

        // Store them in another array, slice (the type depends on that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "First is `3`, `second` is {:?}, and the rest of the array is {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and last values, and store the rest of them in a single array:
        [first, middle @ .., last] => println!(
            "First is `{:?}`, middle is `{:?}`, and last is `{:?}`",
            first, middle, last
        ),
    }
}

// ### 8.5.1.3 Enumerations

#[allow(dead_code)] // silence warnings because only one variant is used in the example
enum Color {
    // These 3 are specified solely by their name
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn showcase_destructuring_enums() {
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");

    // An `enum` can be destructured using a `match`
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, Saturation: {}, Value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, Saturation: {}, Lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, Magenta: {}, Yellow: {}, Key (Black): {}",
            c, m, y, k
        ),
        // Don't need a catch-all case because all possible cases are covered
    }
}

// #### 8.5.1.4 Pointers and References

// For pointers, a distinction needs to be made between destructuring and dereferencing as they are different concepts
// which are used differently from languages like C or C++.

// Deferencing uses the `*` operator, and is used to access the value that a pointer points to.
// Destructuring uses the `&`, `ref` and `ref mut` keywords. It is used to destructure a reference into its referent, and to bind the referent to a variable.

fn showcase_destructuring_pointers() {
    // Assign a reference of type `i32`. The `&` signifies there is a reference being assigned
    let reference = &4;

    match reference {
        // If `reference` is pattern matched against `&val`, it results in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32` should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching:
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&` because the right side was already a reference.
    // This is not a reference because the right side is not one
    let _not_a_reference = 3;

    // Rust provides `ref` keyword or exactly this purpose. It modifies the assignment so that a reference is created for the element; this reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining two values without references, references can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly
    match mut_value {
        ref mut m => {
            // Got a reference. Goota dereference it before we can add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", mut_value);
        }
    }
}
// #### 8.5.1.5 Structs

// Simiarly, a struct can be destructured by matching against its name and its fields.

fn showcase_destructuring_structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see how the match block reacts
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, `b` is {:?}, and `y` is {:?}", b, y),

        // you can destructure structs and rename the variables, the order does not matter
        Foo { y: 2, x: i } => println!("`y` is 2 and `x` is {:?}", i),

        // You can also ignore some fields with `..`
        Foo { x: (3, ..), .. } => println!("First of x is 3 and the rest doesn't matter"),

        Foo { x: (a, ..), y: 4 } => println!(
            "First of x is {:?}, `y` is 4, and the rest of x doesn't matter",
            a
        ),

        // Handle all other cases
        _ => println!("It doesn't matter what they are"),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // Yo do not need a match block to destructure a struct:
    let Foo { x: (a, b), y } = faa;
    println!(
        "Destructured `faa` into `a`, `b`, and `y`: {:?}, {:?}, {:?}",
        a, b, y
    );

    // Destructuring works with nested structs as well
    struct Bar {
        foo: Foo,
    }

    let bar = Bar {
        foo: Foo { x: (1, 2), y: 3 },
    };
    let Bar {
        foo: Foo { x: (a, b), y },
    } = bar;
    println!(
        "Destructured `bar` into `a`, `b`, and `y`: {:?}, {:?}, {:?}",
        a, b, y
    );
}

// ### 8.5.2 Match Guards

// A `match` guard can be added to fitler the arm

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn showcase_match_guard() {
    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30C", t),
        Temperature::Celsius(t) => println!("{}C is below or equal to 30C", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86F", t),
        Temperature::Fahrenheit(t) => println!("{}F is below or equal to 86F", t),
    }

    // Note that the compiler won't take guard conditions into account when checking if all patterns are covered by the match expression
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("positive"),
        _ => unreachable!("negative, but this will never happen. but the compiler still complains if this case is not handled"),
    }
}

// ### 8.5.3 Bindings

// Indirectly accessing a variable makes it impossible to branch and use that variable without re-binding.
// `match` provides the `@` sigil for binding values to names:

// A function `age` which returns a `u32`
fn age() -> u32 {
    15
}

fn showcase_bindings() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could match 1..=12 directly but then what age would the child be?
        // Could match n and use an if guard, but would not contribute to exhaustiveness checks (Although in this case it would be fine because the catch-all case is present at the bottom)
        // Instead, bind to `n` for sequence of 1..=12, now the age can be reported
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // A similar binding can be done when matching several values
        n @ (1 | 7 | 15 | 13) => println!("I'm either 1, 7, 15, or 13 years old: {:?}", n),
        // Nothing bound. Return the result
        n => println!("I'm an old person of age {:?}", n),
    }
}

// You can also use binding to "destructure" enum variants such as Option

fn some_number() -> Option<u32> {
    Some(42)
}

fn showcase_option_bindings() {
    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`, is equal to 42.
        // Could also use `Some(42)` and print `The answer is 42` but what would hardcode the value 42 and not take it from n.
        // Could also use `Some(n) if n == 42` and print `The answer is {n}` but that would not contribute to exhaustiveness checks because of the guard.
        Some(n @ 42) => println!(
            "The answer to the Ultimate Question of Life, The Universe, and Everything: {:?}",
            n
        ),
        Some(n) => println!("Just a regular number: {:?}", n),
        // Match anything else, (`None` variant)
        _ => println!("No answer"),
    }
}

// ## 8.6 `if let`

// For some use-cases, when matching enums, `match` can be awkward.

fn showcase_if_let() {
    let optional = Some(7);

    match optional {
        Some(i) => println!(
            "This is a really long way to say that the optional contains {:?}",
            i
        ),
        _ => (),
        // ^ Required because `match` must be exhaustive, but looks like unnecessary ceremony when you only care about one case
    }

    // if let is cleaner for this use case and in addtion allows various failure options to be specified
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: if `let` destructures `number` into `Some(i)`, evaluate the block. Otherwise, ignore the result and move on.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure case, use an else
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // Destructure failed. Evaluate an `else if` condition to see if the alternate failure branch should be taken
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    // In the same way, `if let` can be used to match any enum value
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // Variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match Foo::Bar, so this will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    // Variable c matches Foo::Qux, which has a value, similar to Some in the previous examples
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    // Another benefit is that `if let` allows us to match non-parameterized enum variants.
    // This is true even in cases where the enum doesn't implement or derive `PartailEq`.
    // In such cases, `if Foo::Bar == a` would fail to compile, because instances of the enum cannot be equated, however,
    // if let will continue to work.

    // This enum purposely neither implements nor derives `PartialEq`
    enum Foo2 {
        Bar,
        Baz,
    };

    let a = Foo2::Bar;

    // Variable a matches Foo2::Bar
    if let Foo2::Bar = a {
        println!("a is foobar");
    }
}

// ## 8.7 `let else`

// stable since Rust 1.65

// With `let-else`, a refutable pattern can match and bind variables in the surrounding scope like a normal `let`
// or else diverge (e.g. `break`, `return`, `panic!`) when the pattern doesn't match.

use std::str::FromStr;

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'")
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("Count was not a number: '{count_str}'")
    };

    (count, item)
}

fn showcase_let_else() {
    let (count, item) = get_count_item("3 chairs");
    println!("There are {count} {item}");
}

// The scope of name bindings is the main thing that makes this different from `match` or `if let else` expressions.
// You could previously approximate these patterns with an unfortunate bit of repitition and outer let:
// let (count_str, item) = match (it.next(), it.next()) {
//     (Some(count_str), Some(item)) => (count_str, item),
//     _ => panic!("Can't segment count item pair: '{s}'"),
// };
// let count = if let Ok(count) = u64::from_str(count_str) {
//     count
// } else {
//     panic!("Count was not a number: '{count_str}'")
// };

// ## 8.8 `while let`

// Similar to `if let`, `while let` can make awkward `match` sequences more tolerable.

fn showcase_while_let() {
    let mut optional = Some(0);

    // Repeatedly try this test
    loop {
        match optional {
            // If optional destructures, evaluate the block
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None; // This will cause the loop to stop after this iteration
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1); // This will cause the loop to continue after this iteration
                }
                // ^ Requires 3 indentations!
            }
            // Quit the loop when destructuring fails
            _ => {
                break;
            } // ^ Why should this be required? There must be a better way.
        }
    }

    let mut optional = Some(0);

    // This reads: `while let` destructures `optional` into `Some(i)` evaluate the block, else break
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None; // This will cause the loop to stop after this iteration
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1); // This will cause the loop to continue after this iteration
        }
        // ^ Less rightward drift and doesn't require explicit handling of the failure case!
    }
    // ^ `if let` had addtitional `else`/`else if` clauses. `while let` does not have these.
}
