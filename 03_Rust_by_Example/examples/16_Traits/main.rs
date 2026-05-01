#![allow(dead_code, unused_variables)]

// # Traits

// A `trait` is a collection of methods defined for an unknown type: `Self`. The can access other methods declared in the same trait.

// Traits can be implemented for any data type.
// In the example below, we define `Animal`, a group of methods. The `Animal` trait is then implemented for the `Sheep` data type, allowing the use of
// methods from `Animal` within a `Sheep` instance.

fn main() {
    showcase_traits();
    showcase_derive();
    showcase_dyn();
    showcase_operator_overloading();
    showcase_drop();
    showcase_drop_example().unwrap();
    showcase_iterator();
    showcase_impl_trait();
    showcase_clone_and_copy();
    showcase_supertraits();
}

struct Sheep {
    naked: bool,
    name: &'static str,
}

trait Animal {
    // Asssociated function signature; `Self` refers to the implementor type
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`
impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaah?"
        } else {
            "baaaah!"
        }
    }

    // Default trait methods can be overridden
    fn talk(&self) {
        // For example, we can add some quiet contemplation
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn showcase_traits() {
    // Type annotation is necessary in this case
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.shear();
    dolly.talk();
}

// ## Derive

// The compiler is capable of providing basic implementations for some traits via the `#[derive]` attribute.
// These traits can still be manually implementeed if a more complex behaviour is required.

// The following is a list of derivable traits:
// - Comparison traits: `Eq`, `PartialEq`, `Ord`, `PartialOrd`
// - `Clone`, to create `T` from `&T` via a copy
// - `Copy`, to give a type 'copy semantics' instead of 'move semantics' (i.e. the type can be duplicated simply by copying bits, without needing to call `clone`)
// - `Hash`, to compute a hash from `&T`
// - `Default`, to create an empty instance of a data type
// - `Debug`, to format a value using the `{:?}` formatter

// `Centimeters`, a tuple struct that can be compared
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

// `Seconds`, a tuple struct with no additional attributes
struct Seconds(i32);

fn showcase_derive() {
    let _one_second = Seconds(1);

    // Error: `Seconds` can't be printed; it doesn't implement `Debug` trait
    // println!("one second looks like: {:?}", _one_second);
    // ^ uncommenting the above line will cause a compiler error

    // Error: `Seconds` can't be compared; it doesn't implement `PartialEq` trait
    // let _is_equal = _one_second == Seconds(1);
    // ^ uncommenting the above line will cause a compiler error

    let foot = Inches(12);
    println!("One foot is {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp = if foot.to_centimeters() < meter {
        "smaller"
    } else {
        "bigger"
    };
    println!("One foot is {} than one meter.", cmp);
}

// ## 16.2 Returning Traits with dyn

// The Rust compiler needs to know how much space every function's return type requires. This means all your functions have to return a concrete type.
// Unlike other languages, if you have a trait like `Animal`, you can't write a function that returns `Animal`, because its different implementations will need
// different amounts of memory.

// However, there's an easy workaround. Instead of returning a trait object directly, our functions return a `Box` which contains some `Animal`.
// A `Box` is just a reference to some memory on the heap. Because a reference has a statically-known size, and the compiler can guarantee it points to a
// heap allocated `Animal`, we can return a trait object wrapped in a `Box` from our function!

// Rust tries to be as explicit as possible, whenever it allocates memory on the heap. So if your function returns a pointer-to-trait-on-heap in this way,
// you need to write the return type with the `dyn` keyword, e.g. `Box<dyn Animal>`

struct Dog {}
struct Cat {}

trait Pet {
    fn name(&self) -> &'static str;
}

impl Pet for Dog {
    fn name(&self) -> &'static str {
        "Dog"
    }
}

impl Pet for Cat {
    fn name(&self) -> &'static str {
        "Cat"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time
fn random_animal(random_number: f64) -> Box<dyn Pet> {
    if random_number < 0.5 {
        Box::new(Dog {})
    } else {
        Box::new(Cat {})
    }
}

fn showcase_dyn() {
    let random_number = 0.2345;
    let animal = random_animal(random_number);
    println!("Random animal is a {}", animal.name());
}

// ## 16.3 Operator Overloading

// In Rust, many of the operators can be overloaded with traits. That is, some operators can be used to accomplish different tasks based on their input arguments.
// This is possible because operators are syntactic suger for method calls. For example, the `+` operator in `a + b` call the `add` method (as in `a.add(b)`).
// This `add` method is part of the `Add` trait. Hence the `+` operator can beused by any implementor of the `Add` trait.

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of the `+` operator.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: `Foo` + `Bar` = `FooBar`
impl std::ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`
// This block implements the operation: Bar + Foo = BarFoo
impl std::ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        BarFoo
    }
}

fn showcase_operator_overloading() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}

// ## 16.4 Drop

// The `Drop` trait only has one method: `drop`, which is called automatically when an object goes out of scope.
// The main use of the `Drop` trait is to free the resources that the implementor instance owns.

// `Box`, `Vec`, `String`, `File` and `Process` are some examples of types that implement the `Drop` trait to free resources.
// The `Drop` trait can also be manually implemented for any custom data type

struct Droppable {
    name: &'static str,
}

// This trivial implementation of `drop` adds a print to the console
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn showcase_drop() {
    let _a = Droppable { name: "a" };

    // block A
    {
        let _b = Droppable { name: "b" };

        // block B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // Variable can be manually dropped using the `drop` function
    drop(_a);

    showcase_drop_example().unwrap();

    println!("End of the function");
    // `_a` won't be dropped again here, because it has already been dropped manually
}

struct TempFile {
    file: std::fs::File,
    path: std::path::PathBuf,
}

impl TempFile {
    fn new(path: std::path::PathBuf) -> std::io::Result<Self> {
        // Note: File::create() will overwrite existing files
        let file = std::fs::File::create(&path)?;
        Ok(Self { file, path })
    }
}

// When TempFile is dropped:
// 1. Our drop implementation will remove the file's name from the filesystem
// 2. File's drop will close the file, removing its underlying contents from the disk
impl Drop for TempFile {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.path) {
            eprintln!("Failed to remove temp file {}: {}", self.path.display(), e);
        }
        println!("Dropped TempFile at {}", self.path.display());
        // File's drop is implicitly called here because it is a field of this struct that is being dropped
    }
}

fn showcase_drop_example() -> std::io::Result<()> {
    // Create a new scope to demonstrate drop behaviour
    {
        let temp = TempFile::new("test.txt".into())?;
        println!("Created temp file at {}", temp.path.display());
        // File will be automatically closed and removed when `temp` goes out of scope at the end of this block
    }
    println!("End of scope - file should be dropped");

    // We can also manually drop if needed
    let temp2 = TempFile::new("test2.txt".into())?;
    println!("Created temp file at {}", temp2.path.display());
    drop(temp2);
    println!("Manually dropped temp2");

    Ok(())
}

// ### 16.5 Iterators

// The `Iterator` trait is used to implement iterators over collections suchas arrays.

// The trait requires only a method to be defined for the `next` element, which may be manually defined in an `impl` block or automatically defined (as in arrays and ranges)

// As a point of convenience for common situations, the `for` construct turns some collections into iterators using the `.into_iter()` method

struct Fibonacci {
    curr: u32,
    next: u32,
}

// Implement `Iterator` for `Fibonacci`
// The `Iterator` trait only requires a method to be defined for the `next` element and an `associated type` to declare the return type of the iterator
impl Iterator for Fibonacci {
    // We can refer to this type using Self::Item
    type Item = u32;

    // Here, we define the sequence using `.curr` and `.next`
    // The return type is `Option<T>`:
    // - When the `Iterator` is finished, `None` is returned
    // - Otherwise, the next value is wrapped in `Some` and returned
    // We use Self::Item in the return type, so we can change the type without haing to update the function signature
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        // Since there's no endpoint to a Fibonacci sequence, the `Iterator` will never return `None`, and `Some` is always returned
        Some(current)
    }
}

// Returns a Fibonacci sequence generator
fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { curr: 0, next: 1 }
    }
}

fn showcase_iterator() {
    // `0..3` is an `Iterator` that generates the sequence 0, 1, 2
    let mut sequence = 0..3;

    println!("Fource consecutive `next` calls on 0..3");
    println!("> Next: {:?}", sequence.next());
    println!("> Next: {:?}", sequence.next());
    println!("> Next: {:?}", sequence.next());
    println!("> Next: {:?}", sequence.next());

    // `for` works through an `Iterator` until it returns `None`
    // Each `Some` value is unwrapped and bound to a variable (here, `i`)
    println!("Iterating through 0..3 with `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    // The `take(n)` method reduces an `Iterator` to its first `n` terms
    println!("The first four terms of the Fibonacci sequence are:");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    // The `skip(n)` method shortens an `Iterator` by dropping its first `n` terms
    println!("The Fibonacci sequence without the first four terms:");
    for i in fibonacci().skip(4).take(5) {
        println!("> {}", i);
    }

    let array = [1u32, 3, 3, 7];

    // The `iter` method produces an `Iterator` over an array/slice
    println!("Iterating over the array {:?} with `iter`", array);
    for i in array.iter() {
        println!("> {}", i);
    }
}

// ## 16.6 impl Trait

// `impl Trait` can be used in two locations:
// 1. as an argument type
// 2. as a return type

//### As an argument type

// If your function is generic over a trait but you don't mind the specific type, you can simplify the function delcaration using `impl Trait` as the type of the argument

fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|l| {
                // If the line was read successfully, process it, if not, return the error
                l.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace from each entry, and convert it to a String
                    .collect() // Collect the entries into a Vec<String>
            })
        })
        .collect() // Collect the lines into a Vec<Vec<String>>
}

// `parse_csv_document` is generic, allowing it to take any type which implements `BufRead`, such as `BufRead<File>` or `[u8]`, but it's not
// important what type R isnd `R` is only used to declare the type of `src` so the function can also be written as
// parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> { ... }

// ## As a return type

// If your function returns a type that implements `MyTrait`, you can write its return type as `-> impl MyTrait`.
// This can help simplify your type signature quite a lot

use std::iter;
use std::vec::IntoIter;

// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated the return type is without `impl Trait`!
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but with `impl Trait` to simplify the return type
fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// You can also use `impl Trait` to return an iterator that uses `map`/`filter` closures! This makes using `map`/`filter` easier.
// Because closure types don't have names, you can't write out an explicit return type if your function returns iterators with closures.
// But with `impl Trait` you can do this easily:
fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
    numbers.iter().filter(|x| x > &&0).map(|x| x + 2)
}

fn showcase_impl_trait() {
    let v = vec![1, 2, 3];
    let u = vec![4, 5, 6];

    // We can use the `combine_vecs` function without worrying about its return type
    for i in combine_vecs(v, u).take(10) {
        println!("> {}", i);
    }

    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 5]);
}

// ## 16.7 Clone and Copy

// When dealing with resources, the default behaviour, in rust, is to transfer them during assigments or function calls.
// However, sometimes, we need to make a copy of the resource instead of transferring it. This is where the `Clone` and `Copy` traits come into play.

// The `Clone` traits allows us to create a copy of a resource by implementing the `clone` method. This is useful for types that manage resources, such as `String` or `Vec<T>`, which need to allocate memory on the heap.

// ## Copy: Implicit Cloning

// The `Copy` trait allows a type to be duplicated by simply coping bits, with no additional logic required.
// When a type implements `Copy`, assignments and function calls will implicitly copy the value instead of moving it.

// Important: `Copy` requries `Clone` - any type that implements `Copy` must also implement `Clone`. This is because `Copy` is defined as a subtrait:
// `trait Copy: Clone {}`. The `Clone` implementation for `Copy` types simply copes the bits.

// Not all types can implement `Copy`. A type can only be `Copy` if:
// - All of its components are `Copy`
// - It doesn't manage external resources (like heap memory, file-handles, etc)

// A unit struct without resources.
// Note: Copy requires Clone, so we must derive both
#[derive(Debug, Clone, Copy)]
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
// This CANNOT be Copy because `Box<T>` is not Copy
#[derive(Debug, Clone)]
struct Pair(Box<i32>, Box<i32>);

fn showcase_clone_and_copy() {
    // Instantiate `Unit`
    let unit = Unit;
    // Copy `Unit` - this is an implicit copy, not a move!
    // Because `Unit` implements `Copy`, the value is duplicated automatically
    let copied_unit = unit;

    // Both units can be used independently
    println!("Original unit: {:?}, Copied unit: {:?}", unit, copied_unit);

    // Instantiate `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("Original pair: {:?}", pair);

    // Move pair into `moved_pair`, moves resources
    // Pair does not implement `Copy`, so this is a move
    let moved_pair = pair;
    println!("Moved pair: {:?}", moved_pair);

    // Error! `pair` has lost is resources
    // println!("Original pair: {:?}", pair);

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    // Unlike Copy, Clone is explicit - we must call .clone()
    let cloned_pair = moved_pair.clone();

    // Drop the moved original pair std::mem::drop(moved_pair);
    drop(moved_pair);

    // Error! `moved_pair` has been dropped
    // println!("Moved pair: {:?}", moved_pair);

    // The result from `.clone()` can still be used
    println!("Cloned pair: {:?}", cloned_pair);
}

// ## 16.8 Supertraits

// Rust doesn't have "inheritance", but you can define a trait as being a superset of another trait.

trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Studen
// Implementing Student requires you to also implement Person
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// ComSciStudent is a subtrait of both Programmer and Student. Implementing ComSciStudent requries you to implement both Programmer and Student
trait ComSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn ComSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git Username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

impl Programmer for CSStudent {
    fn fav_language(&self) -> String {
        self.fav_language.clone()
    }
}

impl Student for CSStudent {
    fn university(&self) -> String {
        self.university.clone()
    }
}

impl Person for CSStudent {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl ComSciStudent for CSStudent {
    fn git_username(&self) -> String {
        self.git_username.clone()
    }
}

fn showcase_supertraits() {
    let student = CSStudent {
        name: "Alice".to_string(),
        university: "MIT".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "alice123".to_string(),
    };

    println!("{}", comp_sci_student_greeting(&student));
}

// ## 16.9 Disambiguating overlapping traits

// A type can implement may different traits. What if two traits both requrie the same name for a function?
// For example, many traits might have a method named `get()`. They might even have different return types!

// Becuase each trait implementation gets its own `impl` block, it's clear which trait's `get` method you're implementing.
// You can use the Fully Qualified Syntax to disambiguate between them.

trait UsernameWidget {
    // Get the selected username out of this widget
    fn get(&self) -> String;
}

trait AgeWidget {
    // Get the select age out of this widget
    fn get(&self) -> u32;
}

// A form with both a UsernameWidget and an AgeWidget
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u32 {
        self.age as u32
    }
}

fn showcase_disambiguating_overlapping_traits() {
    let form = Form {
        username: "alice123".to_string(),
        age: 30,
    };

    // If you uncomment this line, you'll get an error saying "multiple `get` found".
    // Because, after all, there are multiple methods named `get`
    // println!("{}", form.get());

    let username = <Form as UsernameWidget>::get(&form);
    let age = <Form as AgeWidget>::get(&form);
    println!("Username: {}, Age: {}", username, age);
}
