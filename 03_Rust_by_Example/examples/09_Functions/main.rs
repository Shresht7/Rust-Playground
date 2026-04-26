#![allow(dead_code)]

// # 9. Functions

// Functions are declared using the `fn` keyword. Its arguments are type annotated, just like variables
// and, if the function returns a value, the return type must be specified after an arrow (`->`).

// The final expression in the function will be implicitly returned, so it should not end with a semicolon (`;`).
// Alternatively, the `return` statement can be used to return a value earlier from within the function, even from inside loops or `if` statements.

fn main() {
    showcase_functions();
    showcase_associated_functions_and_methods();
    showcase_closures();
    showcase_closure_syntax();
    showcase_closure_capturing();
    showcase_closure_as_input_parameters();
    showcase_type_anonymity();
    showcase_input_functions();
    showcase_closure_as_output_parameters();
    showcase_iterator_any();
    showcase_iterator_find();
    showcase_iterator_position();
    showcase_higher_order_functions();
    showcase_never_type();
}

//Unlike C/C++, there's no restrictions on the order of function definitions in Rust. Functions can be defined after they are called, and the compiler will still be able to find them.

fn showcase_functions() {
    // We can use this function here, and define it somewhere later
    fizzbuzz_to(100);
}

// Function that returns a boolean value
fn is_divisible_by(lsh: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not needed here
    lsh % rhs == 0
}

// Functions that don't return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the signature
fn fizzbuzz_to(n: u32) {
    for i in 1..=n {
        fizzbuzz(i);
    }
}

// ## 9.1 Associated functions and Methods

// Some functions are connected to a particular type. These come in two forms: associated functions and methods.

// Associated functions are functions that are defined on a type generally, while methods are associated functions that are called on a particular instance of a type.

struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions and methods will be defined in this block
impl Point {
    // This is an assocaited function because this function is assocaited with a particular type, i.e., Point.
    // Associated functions don't need to be called with an instance. i.e. they don't need to access `self` parameter.
    // These functions are generally used as constructors, but they can be used for any purpose.
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    // Another associated function, this one is for the origin point
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method.
    // The `&self` is sugar for `self: &Self`, where `Self` is the type alias for the caller object. In this case, `Self` is an alias for `Rectangle`.
    fn area(&self) -> f64 {
        // `self` gives access to the struct's fields via the dot operator
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of a number
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires caller object to be mutable as it modifies the caller object.
    // `&mut self` is syntactic sugar for `self: &mut Self`
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;
        self.p2.x += x;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object.
    // `self` is syntactic sugar for `self: Self`, which means that the caller object will be moved into the method, and the method will take ownership of the caller object.
    fn destroy(self) {
        // Destructure `self`
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // first and second go out of scope and get dropped here
    }
}

fn showcase_associated_functions_and_methods() {
    let rectangle = Rectangle {
        // Assocaited functions are called using double colon syntax
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed
    // i.e. `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter()); // Rectangle perimeter: 14
    println!("Rectangle area: {}", rectangle.area()); // Rectangle area: 12

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requries a mutable object
    // rectangle.translate(1.0, 1.0);

    // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // pair.destroy();
}

// ## 9.2 Closures

// Closures are functions that can capture the enclosing environment. They are also called "anonymous functions" because they don't have a name.
// The syntax and capabilities of closures make them very convenient for on the fly usage. Calling a closure is exactly like calling a function.
// However, both input and return types can be inferred and input variable names must be specified.

fn showcase_closures() {
    showcase_closure_syntax();
}

fn showcase_closure_syntax() {
    let outer_var = 42;

    // A regular function can't refer to variables in the enclosing scope
    // fn regular_function(i: i32) -> i32 { i + outer_var } // error: cannot find value `outer_var` in this scope
    // Uncomment the line above to see the compiler error. The compiler suggests thatwe define a closure instead.

    // Closures are anonymous, here we are binding them to references.
    // Annotation is identical to function annotations but is optional.
    // as are the `{}` wrapping the body for single expression closures.
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred = |i| i + outer_var;

    // Call the closures
    println!("closure annotated: {}", closure_annotated(1)); // closure annotated: 43
    println!("closure inferred: {}", closure_inferred(1)); // closure inferred: 43

    // Once the closure's type has been inferred, it cannot be inferred again with another type.
    // println!("closure inferred: {}", closure_inferred(1.0)); // error: mismatched types

    // A closure taking no arguments wihch returns an `i32`
    // the return type is inferred
    let one = || 1;
    println!("closure returning one: {}", one()); // closure returning one: 1
}

// ### 9.2.1 Capturing

// Closures are inherently flexible and will do what the functionality requires to make the closure work without annotations.
// This allows capturing to flexibly adapt to the use case, sometimes moving and sometimes borrowing.

// Closures can capture variables:
// - by referencing: `&T`
// - by mutable reference: `&mut T`
// - by value: `T`

// They preferentially capture variables by reference, and only go lower when required.

fn showcase_closure_capturing() {
    let color = String::from("green");

    // A closure to print `color` which immediately borrows (`&`) `color` and stores the borrow and closure in the `print` variable.
    // It will remain borrowed until `print` is used the last time, at which point the borrow will be released.
    // `println!` only requires arguments by immutable reference so it doesn't impose anything more restrictive.
    let print = || println!("`color`: {}", color);

    // Call the closure using the borrow
    print(); // `color`: green

    // `color` can be borrowed immutably again, because the closure only holds an immutable reference to `color`
    let _reborrow = &color;
    print(); // `color`: green

    // A move or reborrow is allowed after the final use of `print`
    let _color_moved = color;

    let mut count = 0;
    // A closure to increment `count` could either take `&mut count` or `count`, but `&mut count` is less restrictive so it will take that. Immediately borrows `count`.

    // A `mut` is required on `inc` because a `&mut` is stored inside. Thus, calling the closure mutates `count` which requires a `mut`.
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // Call the closure using a mutable borrow
    inc(); // `count`: 1

    // The closure still mutably borrows `count` because it is called later.
    // An attempt to reborrow will lead to an error.
    // let _reborrow = &mut count; // error: cannot borrow `count` as mutable more than once at a time
    inc(); // `count`: 2

    // The closure no longer needs to borrow `&mut count`. Therefore, it is possible to reborrow without an error.
    let _count_reborrowed = &mut count;

    // A non-copy type
    let movable = Box::new(3);

    // `std::mem::drop` requires `T` so this must take by value. A copy type would copy into the closure leaving the original untouched.
    // A non-copy type must move and so `movable` immedaitely moves into the closure
    let consume = || {
        println!("`movable`: {:?}", movable);
        std::mem::drop(movable);
    };

    // `consume` comsumes the variable so this can only be called once.
    consume(); // `movable`: 3

    // consume(); // error: use of moved value: `consume`

    // Using `move` before vertical pipes forces colsure to take ownership of captured variables

    // `Vec` has non-copy semantics
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("contains 1 in haystack: {}", contains(&1)); // contains 1 in haystack: true
    println!("contains 4 in haystack: {}", contains(&4)); // contains 4 in haystack: false

    // println!("haystack: {:?}", haystack); // error: use of moved value: `haystack`
    // `haystack` has been moved into the closure and cannot be used anymore

    // Removing `move` from closure's signature will cause closure to borrow _haystack_ variable immutably, hence _haystack_ is still
    // available and uncommenting above line will not cause an error.
}

// ### 9.2.2 As Input Parameters

// While Rust chooses how to capture variables on the fly mostly without type annotations, this ambiguity is not allowed when writing functions.
// When taking a closure as an input parameter, the closure's complete type must be annotated using one of a few `traits`, and they're determined
// by what the closure does with the captured value. In order of decreasing restriction, they are:
// - `Fn`: the closure uses the captured value by reference (`&T`)
// - `FnMut`: the closure uses the captured value by mutable reference (`&mut T`)
// - `FnOnce`: the closure uses the captured value by value (`T`)

// The compiler will try to capture the variable in the least restrictive way possible, on a variable-by-variable basis.

// For instance, consider a parameter annotated as `FnOnce`. This specifies that the closure may capture by `&T`, `&mut T`, or `T`, but
// the compiler will ultimately choose based on how the captured variables are used in the closure body.

// This is because if a move is possible, then any type of borrow should also be possible. Note that the reverse is not true.
// If the parameter is annotated as `Fn`, then capturing variables by `&mut T` or `T` are not allowed. However, `&T` is allowed.

// A function which takes a closure as an argument and calls it. <F> denotes that F is a "Generic type parameter"

fn apply<F>(f: F)
where
    //The closure takes no input and returns nothing
    F: FnOnce(),
{
    // ^ Try changing `FnOnce` to `FnMut` or `Fn` and see how the behavior changes when you uncomment the lines below
    f();
}

// A function which takes a closure and returns an `i32`
fn apply_to_3<F>(f: F) -> i32
where
    // The closure takes an `i32` and returns an `i32`
    F: Fn(i32) -> i32,
{
    f(3)
}

fn showcase_closure_as_input_parameters() {
    let greeting = "hello";

    // A non-copy type. `to_owned` creates owned data from borrowed one.
    let mut farewell = "goodbye".to_owned();

    // Capture 2 varaibles: `greeting` by reference and `farewell` by value
    let diary = || {
        // `greeting` is by reference. requires: `Fn`
        println!("I said {}", greeting);

        // Mutation forces `farewell` to be captured by mutable reference. Now requires: `FnMut`
        farewell.push_str("!!!");
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. zzz...");

        // Manually calling drop forces `farewell` to be captured by value. Now requires: `FnOnce`
        std::mem::drop(farewell);
    };

    // Call the function which applies the closure
    apply(diary);

    // `double` satisfies `apply_to_3`'s trait bound so it can be passed as an argument
    let double = |x| 2 * x;

    println!("apply_to_3 with double: {}", apply_to_3(double)); // apply_to_3 with double: 6
}

// ### 9.2.3. Type Anonymity

// Closures succinctly capture variables from enclosing scopes.

// When a closure is defined, the compiler implicitly creates a new anonymous structure to store the captured variable inside,
// meanwhile implementing the functionality via one of the traits: `Fn`, `FnMut` or `FnOnce` for this unknown type.

// Since this new type is of unkown type, any usage in a function will require generics. Howwever, an unbounded type parameter `<T>` would still
// be ambiguous and not be allowed. Thus, bounding by one of the traits is sufficeint to specify its type.

fn showcase_type_anonymity() {
    let x = 7;

    // Capture `x` into an anonymous type and implement `Fn` for it. Store it in `print`
    let print = || println!("`x`: {}", x);

    // Call the closure
    apply(print); // `x`: 7
}

// ### 9.2.4 Input Functions

// Since closures may be used as arguments, you might wonder if the the same can be said about functions. The answer is yes!
// If you declare a function that takes a closure as a parameter, then any function that satisfies the trait bound of the closure can be passed as an argument.

// Define a function which takes a generic `F` argument bounded by `Fn` and calls it
fn call_me<F: Fn()>(f: F) {
    f()
}

// Define a wrapper function satisfying `Fn` trait bound
fn function() {
    println!("I'm a function!");
}

fn showcase_input_functions() {
    // Define a closure satisfying the `Fn` bound
    let closure = || println!("I'm a closure!");

    call_me(closure); // I'm a closure!
    call_me(function); // I'm a function!
}

// ### 9.2.5 As Output Parameters

// Closures as input parameters are possible, so returning closures as output parameters should also be possible.
// However, anonymous closure types are, by definition, unknown, so we have to use `impl Trait` to return them.

// The valid trats for returning a closure are:
// - `Fn`
// - `FnMut`
// - `FnOnce`

// Beyond this, the move keyword must be used, which signals that all captures occur by value. This is required because any captures by
// refeferenc would be dropped as soon as the function exited, leaving invalid references in the closure.

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fn_mut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fn_once() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn showcase_closure_as_output_parameters() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fn_mut();
    let fn_once = create_fn_once();

    fn_plain(); // This is a: Fn
    fn_mut(); // This is a: FnMut
    fn_once(); // This is a: FnOnce
}

// ### 9.2.6 Examples in std lib

// #### 9.2.6.1 Iterator::any

// `Iterator::any` is a function which when passed an iterator, will return true if any element satisfies the predicate. Otherwise, it returns false.

pub trait Iterator {
    // The type of the elements being iterated over
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool
    where
        // `FnMut` meaning any captured variables may at most be modified, not consumed. `Self::Item` is the closure parameter type,
        // which is determined by the iterator (e.g. `&T` for `.iter()`), `T` for `.into_iter()`).
        F: FnMut(Self::Item) -> bool;
}

fn showcase_iterator_any() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2)); // 2 in vec1: true
                                                             //

    // `iter` gives an iterator over immutable references to the elements of the vector, so the closure takes `&i32`
    println!("2 in vec2: {}", vec2.iter().any(|&x| x == 2)); // 2 in vec2: false

    // `iter` only borrows `vec1` and its elements, so they can be used again
    println!("vec1: {:?}", vec1); // vec1: [1, 2, 3]
    println!("First element in vec1: {}", vec1[0]); // First element in vec1: 1

    // `into_iter()` does move `vec2` and its elements, so they cannot be used again
    // println!("vec2: {:?}", vec2); // error: use of moved value:

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`. Destructure to `i32`
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2)); // 2 in array1: true

    // `into_iter()` for arrays yields `i32` by value. No need to destructure
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2)); // 2 in array2: false
}

// ## 9.2.6.2 Searching through Iterators

// `Iterator::find` is a function which iterators over an interator and searches for the first value that satisfies some condtion.
// If none of the values satisfy the condition, it returns `None`. Its signature:

pub trait IteratorFind {
    // The type being iterated over
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed and modified, but not consumed.
    fn find<P>(&mut self, prediate: P) -> Option<Self::Item>
    where
        // `FnMut` meaning any captured variable may at most be modified, not consumed.
        // `&Self::Item` states it takes arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool;
}

fn showcase_iterator_find() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `vec1.iter()` yields `&i32`
    let mut iter = vec1.iter();
    // `vec2.into_iter()` yields `i32`
    let mut into_iter = vec2.into_iter();

    // `iter()` yields `&i32`, and `find` passes `&Item` to the predicate.
    // Since `Item = &i32`, the closure argument has type `&&i32`, which we pattern-match to dereference down to `i32`
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));

    // `into_iter()` yields `i32`, and `find` passes `&Item` to the predicate.
    // Since `Item = i32`, the closure argument has type `&i32`, which we pattern-match to dereference down to `i32`
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `array.iter()` yields `&32`
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2)); // Find 2 in array1: Some(2)

    // `array.into_iter()` for arrays yields `i32` by value, so the closure takes `&i32`
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&x| x == 2)
    ); // Find 2 in array2: None
}

// `Iterator::find` gives you a reference to the item. But if you want the index of the item use `Iterator::position`

fn showcase_iterator_position() {
    let vec1 = vec![1, 9, 3, 3, 13, 2];

    // `position` passes the iterator's `Item` by value to the predicate.
    // `vec.iter()` yields `&i32`, so the predicate receives `&i32`, which we pattern-match to dereference down to `i32`
    let index_of_first_even_number = vec1.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_of_first_even_number, Some(5));

    // `vec.into_iter()` yields `i32`, so predicate receives `i32` directly
    let index_of_first_negative_number = vec1.into_iter().position(|x| x < 0);
    assert_eq!(index_of_first_negative_number, None);
}

// ## 9.3 Higher Order Functions

// Rust provides Higher Order Functions (HOFs). These are functions that take one or more functions and/or produce a more useful function.
// HOFs and lazy iterators give Rust its functional flavor.

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn showcase_higher_order_functions() {
    println!("Find the sum of all numbers with odd squares under 1000");
    let upper = 1000;

    // Imperative Approach
    //Declare accumulator variable
    let mut acc = 0;
    // Iterator: 0, 1, 2, ... to inifinity
    for n in 0.. {
        // Square the number
        let n_squared = n * n;

        if n_squared >= upper {
            // Break loop if exceeded the upper limit
            break;
        } else if is_odd(n_squared) {
            // Accumulate value, if it's odd
            acc += n;
        }
    }

    println!("Imperative approach: {}", acc);

    // Functional Approach
    let sum: u32 = (0..)
        .take_while(|&n| n * n < upper) // Below upper limit
        .filter(|&n| is_odd(n * n)) // That are odd
        .sum(); // Sum them up

    println!("Functional approach: {}", sum);
}

// ## 9.4 Diverging Functions

// Diverging functions never return. They are marked using `!`, which is an empty type.

fn foo() -> ! {
    panic!("This call never returns!");
}

// As opposed to all other types, this one cannot be instantiated, because the set of all possible values this type can have is empty.
// Note that, it is different from the `()` type, which has exactly one possible value.

//Altough this might seem like an abstract concept, it is actually very useful and often handy.
// The main advantage of this type is that it can be cast to any other type, making it versatile in situations where an exact type is rquired, such as in match branches.

fn showcase_never_type() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // Notice that the return type of this match expression must be u32 because of the type of the "addition" variable
            let addtion: u32 = match i % 2 == 1 {
                // The "i" variable is of type u32, which is perfectly fine
                true => i,
                // On the other hand, the "continue" expression does not return u32, but it is still fine, because it never returns
                // and therefore does not violate the type requirements of the match expression
                false => continue,
            };
            acc += addtion;
        }
        acc
    }

    println!("Sum of odd numbers up to 10: {}", sum_odd_numbers(10)); // Sum of odd numbers up to 10: 25
}

// It is also the return type of functions that loop forever (e.g. `loop {}`) like network servers or functions that terminate the process (e.g. `exit()`)
