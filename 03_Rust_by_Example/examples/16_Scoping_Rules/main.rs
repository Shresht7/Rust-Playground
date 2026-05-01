#![allow(dead_code, unused_variables)]

// # 16 Scoping Rules

// Scopes play an imporatnt part in ownership, borrowing, and lifetimes. That is, they indicate to the compiler when borrows are valid,
// when resources can be freed, and when variables are created or destroyed.

fn main() {
    showcase_raii();
    showcase_drop();
    showcase_ownership_and_moves();
    showcase_mutability();
    showcase_partial_move();
    showcase_borrowing();
    showcase_mutable_borrowing();
    showcase_aliasing();
    showcase_lifetimes_main();
}

// ## 15.1 RAII

// Variables in Rust do more than just hold data in the stack: they also own resources, e.g. `Box<T>` owns memory in the heap.
// Rust enforces `RAII` (Resource Acquisition is Initialization), so whenever an object goes out of scope, its desctructor is called, and its owned resources are freed.

// This behaviour shields against resource leak bugs, so you'll never have to manually free memory or worry about memory leaks again!

fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);
    // `_box1` is destroyed here, and memory gets freed
}

fn showcase_raii() {
    // Allocate an integer on the heap
    let _box2 = Box::new(5i32);

    // A nested box
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);
        // `_box3` is destroyed here, and memory gets freed
    }

    // Creating lots of boxes just for fun
    // There's no need to manually free memory!
    for _ in 0u32..1_000 {
        create_box();
    }
}

// We can double check for memory errors using using `valgrind` http://valgrind.org/info/

// ## 15.1.1 Destructors

// The notion of destructor in Rust is provided though the `Drop` trait. The destructor is called when the resource goes out of scope.
// This trait is not required to be implemented for every type, only implement it for your type if you require its own destructor logic.

// Run the below example to see how the `Drop` trait works. When the variable in the `main` function goes out of scope, the custom destructor will be invoked.

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped!");
    }
}
fn showcase_drop() {
    let _x = ToDrop;
    println!("Made a ToDrop!");
}

// ## 15.2 Ownership and Moves

// Because variables are in charge of freeing their own resources, resources can only have one owner.
// This prevents resources from being freed more than once. Note that not all variables own resources (e.g. references)

// When doing assigments (let x = y) or passing function arguments by value (`foo(x)`) the ownership of the resource is transferred.
// In Rust-speak, this is known as a move.

// After moving resources, the previous owner can no longer be used. This avoid creating dangling pointers.

// This function takes ownership of the heap allocated memory
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
    // `c` is destroyed and the memory freed
}

fn showcase_ownership_and_moves() {
    // Stack allocated integer
    let x = 5u32;

    // Copy `x` into `y`, - no resources are moved
    let y = x;

    // both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a heap allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // Move `a` into `b`
    let b = a;

    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but `b` now owns it.

    // Error! `a` can no longer access the data, because it no longer owns the heap memory
    // println!("a contains: {}", a);

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would result in deferencing freed memory, but it's forbidden by the compiler
    // println!("b contains: {}", b);
}

// ### 15.2.1 Mutability

// Mutability of data can be changed when ownership is transferred.

fn showcase_mutability() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains: {}", immutable_box);

    // Mutability error
    // *immutable_box = 4u32;

    // Move the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains: {}", mutable_box);

    // Now we can change the data, because `mutable_box` is mutable
    *mutable_box = 4u32;

    println!("mutable_box now contains: {}", mutable_box);
}

// ### 15.2.2 Partial Moves

// Within the destructuring of a single variable, both `by-move` and `by-reference` pattern bindings can be used at the same time.
// Doing this will result in a _partial move_ of the variable, which means that parts of the variable will be moved while other parts stay.
// In such a case, the parent varaible cannot be used afterwards as a whole, however the parts that are only referenced (and not moved) can still be used.
// Note that types that implement the `Drop` trait cannot be partially moved from, because its `drop` method would use it afterwards as a whole.

fn showcase_partial_move() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    // Error! cannot move out of a type which implements the `Drop` trait
    // impl Drop for Person {
    //    fn drop(&mut self) {
    //        println!("Dropping the person struct: {:?}", self);
    //    }
    // }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);
    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    // println!("The person struct is {:?}", person");

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);

    // In this example, we store the `age` variable on the heap to illustrate the partial move: deleting `ref` in the above code would give an error
    // as the ownership of `person.age` would be moved to the variable `age`. If `Person.age` we stored on the stack, `ref` would not be required
    // as the definition of `age` would copy the data from `person.age` without moving it.
}

// ## 15.3 Borrowing

// Most of the time, we'd like to access data without taking ownership of it. To accomplish this, Rust uses a _borrowing_ mechanism.
// Instead of passing objects by value (`T`), objects can be passed by reference (`&T`).

// The compiler statically guarantees (via its borrow checker) that references _always_ point to valid objects.
// That is, while references to an object exist, the object cannot be destroyed.

// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borwwos an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn showcase_borrowing() {
    // Create a boxed i32 in the heap, and an i32 on the stack
    // Remember: numbers can have arbitrary underscores added for readability. i.e. 5_i32 is the same as 5i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // borrow the contents of the box. Ownership is not taken, so the contents can be borrowed again
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope
        // eat_box_i32(boxed_i32);

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed
    }

    // `boxed_i32` can now give up ownership to `eat_box_i32` and be destroyed
    eat_box_i32(boxed_i32);
}

// ### 15.3.1 Mutability

// Mutable data can be mutably borrowed using `&mut T`. This is called a _mutable reference_ and gives read/write access to the borrower.
// In contrast, `&T` borrows the data via an immutable reference, and the borrower can read the data but not modify it.

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read-only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

// This function takes a reference to a mutable book and changes `year` to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn showcase_mutable_borrowing() {
    // Create an immutable Book named `immutabook`
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Gödel, Escher, Bach",
        year: 1979,
    };

    // Create a mutable copy of `immutabook` and call it `mutabook`
    let mut mutabook = immutabook;

    // Immutably borrow an immutable object
    borrow_book(&immutabook);

    // Immutably borrow a mutable object
    borrow_book(&mutabook);

    // Borrow a mutable object as mutable
    new_edition(&mut mutabook);

    // Error! Cannot borrow an immutable object as mutable
    // new_edition(&mut immutabook);
}

// ### 15.3.2 Aliasing

// Data can be immutably borrowed any number of times, but while immutably borrowed, the original data cannot be mutably borrowed.
// On the other hand, only _one_ mutable borrow is allowed at a time. The original data can be borrowed again only _after_ the mutable reference
// has been used for the last time.

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn showcase_aliasing() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_borrow = &point;

    // Data can be accessed via the references and the original owner
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // Error! Can't borrow `point` as mutable because it's currently borrowed as immutable
    // let mutable_borrow = &mut point;
    // ^ Try uncommenting this line.

    // The borrowed values are used again here
    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z
    );

    // The immutable references are no longer used for the rest of the code so it is possible to reborrow with a mutable reference.
    let mutable_borrow = &mut point;

    // Change data via mutable reference
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // Error! Can't borrow `point` as immutable because it's currently borrowed as mutable
    // let y = &point.y;
    // ^ Try uncommenting this line

    // Error! Can't print because `println!` take an immutable reference
    // println!("Point Z coordinates is {}", point.z);
    // ^ Try uncommenting this line

    // Ok! Mutable references can be passed as immutable to `println!`
    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    // The mutable reference is no longer used for the rest of the code so it is possible to reborrow
    let new_borrowed_point = &point;
    println!(
        "Point has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

// ### 15.3.3 The `ref` pattern

// When doing pattern matching or destructuring via the `let` binding, the `ref` keyword can be used to take references to the field of a struct/tuple.

#[derive(Clone, Copy)]
struct Point2 {
    x: i32,
    y: i32,
}

fn showcase_ref_pattern() {
    let c = 'Q';

    // A `ref` borrow on the left side of an assigment is equivalent to an `&` borrow on the right side
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point2 { x: 0, y: 0 };

    // `ref` is also valid when destructuring a struct
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`
        let Point2 {
            x: ref ref_to_x,
            y: _,
        } = point;

        // Return a copy of the `x` field of `point`
        *ref_to_x
    };

    // A mutable copy of `point`
    let mut mutable_point = point;

    {
        // `ref` can be paired with `mut` to take mutable references
        let Point2 {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        // Mutate the `y` field of `mutable_point` via mutable reference
        *mut_ref_to_y += 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!(
        "mutable_point is ({}, {})",
        mutable_point.x, mutable_point.y
    );

    // A mutable tuple that includes a pointer
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // Destructure `mutable_tuple` to change the value of `last`
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}

// ## 15.4 Lifetimes

// A _lifetime_ is a construct the compiler (or more specifically, the borrow checker) uses to ensure all borrows are valid.
// Specifically, a variable's lifetime begins when it is created and ends when it is destroyed. While lifetimes and scopes are often referred to together,
// they are not the same concept.

// When we borrow a variable via `&`. The borrow has a lifetime that is determined by where it is declared. As a result, the borrow is valid
// as long as it ends before the lender is destroyed. However, the scope of the borrow is determined by where the reference is used.

fn showcase_lifetimes_main() {
    showcase_lifetimes();
    showcase_lifetime_annotations();
    showcase_functions_with_lifetimes();
    showcase_methods_with_lifetimes();
    showcase_structs_with_lifetimes();
    showcase_traits_with_lifetimes();
    showcase_bounds_with_lifetimes();
    showcase_coercion();
    showcase_static_lifetime();
    showcase_elision();
}

fn showcase_lifetimes() {
    let i = 3; // Lifetime for `i` starts

    {
        let borrow1 = &i; // `borrow1` lifetime starts

        println!("borrow1: {}", borrow1);
    } // `borrow1` lifetime ends

    {
        let borrow2 = &i; // `borrow2` lifetime starts
        println!("borrow2: {}", borrow2);
    } // `borrow2` lifetime ends
} // Lifetime for `i` ends

// ## 15.4.1 Explicit Annotations

// The borrow checker uses explicit lifetime annotations to determine how long refeences should be valid.
// In cases where lifetimes are not elided, Rust requires explicit annotations to determine what the lifetime of a reference should be.
// The syntax for explicitly annotating a lifetime uses an apostrophe character like `foo<'a>`.

// Similar to closures, using lifetimes requires generics. Additionally, this lifetime syntax indicates that the lifetime of `foo` may not exceed that of `'a`.
// Explicit annotation of a type has the form `&'a T` where `'a` has already been introduced.

// In case of multiple lifetimes, the syntax is similar. `foo<'a, 'b>`. In this case, the lifetime of `foo` may not exceed that of either `'a` or `'b`.

// `print_refs` takes two references to `i32` which have different lifetimes `'a` and `'b`. These two lifetimes must be both at least as long as the function `print_refs`
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`
fn failed_borrow<'a>() {
    let _x = 12;

    // ERROR: `_x` does not live long enough
    // let _y: &'a i32 = &_x;
    // Attempting to use the lifetime `'a` as an explicit type annotation inside the function will fail because the lifetme of `&_x`
    // is shorter than the lifetime of `_y`. A short lifetime cannot be coerced into a longer one.
}

fn showcase_lifetime_annotations() {
    // Create variables to be borrowed below
    let (four, nine) = (4, 9);

    // Borrows ('`&') of both varaibles are passed into the function
    print_refs(&four, &nine);
    // Any input which is borrowed must outlive the borrower.
    // In other words, the lifetime of `four` and `nine` must be longer than that of `print_refs`.

    failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`
}

// ### 15.4.2 Functions

// Ignoring elision, function signatures with lifetimes have a few constraints:
// - any references _must_ hanve an annotated lifetime
// - any reference being returned _must_ have the same lifetime as an input or be `'static`

// Additionally, note that returning references without input is banned if it would result in returning references to invalid data.

// One input reference with lifetime `'a` which must live at least as long as the function
fn print_one<'a>(x: &'a i32) {
    println!("print_one: x is {}", x);
}

// Mutable references are possible with lifetimes as well
fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

// Multiple elements with different lifetimes. In this case, it would be fine for both to have the same lifetime `'a`, but in
// more complex cases, different lifetimes may be required
fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print_multi: x is {}, and y is {}", x, y);
}

// Returning references that have been passed in is aceptable
// However, the correct lifetime must be returned.
fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

// fn invalid_output<'a>() -> &'a String { &String::from("foo") }
// The above is invalid: `'a` must live longer than the function, but here, `&String::from("foo")` would create a `String`,
// followed by a reference to it. Then the data is dropped upon exiting the scope, leaving a reference to invalid data to be returned.

fn showcase_functions_with_lifetimes() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

// ### 15.4.3 Methods

// Methods are annotated simiarly to functions

struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }

    fn print<'a>(&'a self) {
        println!("print: {}", self.0);
    }
}

fn showcase_methods_with_lifetimes() {
    let mut owner = Owner(5);

    owner.print();
    owner.add_one();
    owner.print();
}

// ### 15.4.4 Structs

// Annotation of lifetimes in structures are also similar to functions.

// A type `Borrowed` which houses a reference to an `i32`. The reference to `i32` must outlive `Borrowed`
#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

// Similarly, both references here must outlive this structure
#[derive(Debug)]
struct NamedBorrow<'a> {
    x: &'a i32,
    y: &'a i32,
}

// An enum which is either an `i32` or a reference to one
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn showcase_structs_with_lifetimes() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrow { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("x is borrowed in {:?}", single);
    println!("x and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
}

// ### 15.4.5 Traits

// Annotation of lifetimes in trait method basically are similar to regular functions or methods.
// Note that `impl` may have annotation of lifetimes too.

// A struct with annotation of lifetimes
#[derive(Debug)]
struct Borrowed2<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl
impl<'a> Default for Borrowed2<'a> {
    fn default() -> Self {
        Borrowed2 { x: &0 }
    }
}

fn showcase_traits_with_lifetimes() {
    let borrowed = Borrowed2::default();
    println!("borrowed is {:?}", borrowed);
}

// ### 15.4.6 Bounds

// Just like generic types can be bounded, lifetimes (themselves generic) use bounds as well.
// The `:` character has a slightly different meaning here, but `+` is the same.

// 1. `T: 'a` -- All references in `T` must outlive lifetime `'a`
// 2. `T: Trait + 'a` -- `T` must implement `Trait` and all references in `T` must outlive lifetime `'a`

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` contains a reference to a generic type `T` that has some lifetime `'a` unknwon by `Ref`.
// `T` is bounded such that any *references* in `T` must outlive `'a`. Additionally, the lifetime of `Ref` may not exceed ''a'

// A generic function which prints using `Debug` trait
fn print<T>(t: T)
where
    T: std::fmt::Debug,
{
    println!("`print`: t is {:?}", t);
}

// Here a reference to `T` is taken where `T` implements `Debug` and all references in `T` outlive `'a`.
// In addition, `'a` must outlive this function
fn print_ref<'a, T>(t: &'a T)
where
    T: std::fmt::Debug + 'a,
{
    println!("`print_ref`: t is {:?}", t);
}

fn showcase_bounds_with_lifetimes() {
    let x = 7;
    let ref_to_x = Ref(&x);

    print_ref(&ref_to_x);
    // `print_ref` requires that all references in `Ref` outlive `'a`, and the lifetime of `print_ref` may not exceed `'a`.
    // Because `Ref` contains a reference to `x`, the lifetime of `x` must outlive the lifetime of `print_ref`.
    // This is true because `x` is declared in the same scope as `print_ref`, so it must live at least as long as `print_ref`.

    print(ref_to_x);
}

// ### 15.4.7 Coercion

// A longer lifetime can be coerced into a shorter one, so that it works inside a scope it normall wouldn't work in.
// This comes in the form of inferred coercion by the Rust compiler, and also in the form of declaring a lifetime difference

// Here, Rust infers a lifetime that is as short as possible
// The two references are then coerced to that lifetime
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`
// Here, we take in an `&'a i32` and return a `&'b i32` as a result of coercion.
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn showcase_coercion() {
    let first = 2; // Longer lifetime

    {
        let second = 3; // Shorter lifetime

        println!("the product is {}", multiply(&first, &second));
        println!("the first is {}", choose_first(&first, &second));
    }
}

// ### 15.4.8 Static

// Rust has few reserverd lifetime names. One of those is `'static`.

fn showcase_static_lifetime() {
    // A reference with 'static lifetime
    let s: &'static str = "I have a static lifetime.";

    // 'static as a part of a trait bound
    fn generic<T>(_x: T)
    where
        T: 'static,
    {
        println!("generic function called with a 'static type parameter");
    }

    // As a reference lifetime `'static` indicates that the data pointed to by the reference lives for the remaining lifetime of the running program.
    // It can still be coerced to a shorter lifetime.

    // There are two common ways to make a variable with `'static` lfietime, and both are stored in read-only memory of the binary:
    // - Make a constant with the `static` declaration
    // - Make a `string` literal which has type `&'static str`

    // Make a constant with `'static` lifetime
    static NUM: i32 = 18;

    // Returns a reference to `NUM` where its `'static` lifetime is coerced to that of the input argument
    fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
        &NUM
    }

    {
        // Make a `string` literal and print it
        let static_string = "I'm in read-only memory!";
        println!("{}", static_string);

        // When `static_string` goes out of scope, the reference can no longer be used, but the data remains in the binary
    }

    {
        // Make an integer to use for `coerce_static`
        let lifetime_num = 9;

        // Coerce `NUM` to lifetime of `lifetime_num`
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}

// ### 15.4.8 Elision

// Some lifetime patterns are overwhelmingly common and so the borrow checker will allow you to omit them to save typing and to improve readability.
// This is known as elision. Elision exists in Rust solely because these patterns are common.

// `elided_input` and `annotated_input` essentially have identical signatures because the lifetime of `elided_input` is inferred by the compiler:
fn elided_input(x: &i32) {
    println!("elided_input: x is {}", x);
}

fn annotated_input<'a>(x: &'a i32) {
    println!("annotated_input: x is {}", x);
}

// Similarly, `elided_pass` and `annotated_pass` have identical signatures because the lifetime is added implicitly to `elided_pass`:
fn elided_pass(x: &i32) -> &i32 {
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32 {
    x
}

fn showcase_elision() {
    let x = 5;

    elided_input(&x);
    annotated_input(&x);

    let y = elided_pass(&x);
    let z = annotated_pass(&x);

    println!("y: {}, z: {}", y, z);
}
