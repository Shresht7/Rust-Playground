#![allow(dead_code, unused_variables)]

// # 14 Generics

// Generics is the topic of generalizing types and functionalities to broader cases. This is extremely useful for reducing code duplication in many ways,
// but can call for rather involved syntax. Namely, being generic requires taking great care to specify which types a generic type is actually considererd valid.
// The simple and most commmon use case of generics is for type parameters.

// A type parameter is specified as generic by the use of angle brackets and upper camel case: <Aaa, Bbb, ...>.
// "Generic type parameters" are typically represented as `<T>`. In Rust, "generic" also describes anything that accepts one or more generic type parameters `<T>`.
// Any type specified as a generic type parameter is generic, and everything else is concrete (non-generic)

fn main() {
    showcase_generics();
    showcase_generic_functions();
    showcase_generic_impl();
    showcase_generic_traits();
    showcase_trait_bounds();
    showcase_trait_empty_bounds();
    showcase_multiple_bounds();
    showcase_where_clauses();
    showcase_newtype_idiom();
    showcase_associated_items();
    showcase_phantom_types();
    showcase_phantom_add();
}

// For example, defining a generic function named `foo` that takes an argument `T` of any type:
fn foo<T>(x: T) {
    // Function body goes here
}

// Becuase `T` has be specified as a generic type parameter using `<T>`, it is considered generic when used here as `(arg: T)`.
// This is the case even if `T` has previosuly been defined as a struct.

// A concrete type A
struct A;

// In difining the type `Single`, the first use of `A` is not preceded by `<A>`
// Therefore `Single` is a concrete type, and `A` is defined above.
struct Single(A);

// Here, `<T>` precedes the first use of `T`, so `SingleGeneric` is a generic type
// Because, the type parameter `T` is generic, it could be anything, including the concrete type `A`
struct SingleGeneric<T>(T);

fn showcase_generics() {
    // `Single` is concrete and explicitly takes `A`
    let _s = Single(A);

    // Creats a variable `_char` of type `SingleGeneric<char>`
    // and give it the value `SingleGeneric('a')`
    // Here, `SingleGeneric` is generic, so the type parameter explicitly specified.
    let _char: SingleGeneric<char> = SingleGeneric('a');

    // `SingleGeneric` can also have type paramters implicitly specified:
    let _t = SingleGeneric(A);
    let _i32 = SingleGeneric(3);
    let _char = SingleGeneric('a');
}

// ## 14.1 Functions

// The same set of rules can be applied to functions: a type `T` becomes generic when preceded by `<T>`

// Using generic functions sometimes requires explictly specifying type parameters. This may be the case if function is called when the return type is generic,
// or if the compiler doesn't haven enough information to infer the necessary type parameters.

// A function call with explicitly specified type parameters looks like: `fun::<A, B, ...>(args)`

// The following functions all take ownership of the variable passed into them and immediately go out of scope, freeing the variable.

// Define a function `reg_fn` that takes an argument `_s` of type `Single`.
// This has no `<T>` so this is not a generic function
fn reg_fn(_s: Single) {}

// Define a function `gen_spec_t` that takes an argument `_s` of type `SingleGeneric<T>`
// It has been explicitly given the type parameter `A`, but because `A` has not been specified as a generic type parameter for `gen_spec_t`, it is not generic.
fn gen_spec_t(_s: SingleGeneric<A>) {}

// Define a function `gen_spec_i32` that takes an argument `_s` of type `SingleGeneric<i32>`
// It has been explicitly given the type parameter `i32`, which is a specified type.
// Because `i32` is not a generic type, this function is also not generic.
fn gen_spec_i32(_s: SingleGeneric<i32>) {}

// Define a function `generic` that takes an argument `_s` of type `SingleGeneric<T>`
// Because `SingleGeneric<T>` is preceded by `<T>`, this function is generic over `T`.
fn generic<T>(_s: SingleGeneric<T>) {}

fn showcase_generic_functions() {
    // Using the non-generic functions
    reg_fn(Single(A)); // Concrete type
    gen_spec_t(SingleGeneric(A)); // Generic type with specified type parameter
    gen_spec_i32(SingleGeneric(3)); // Generic type with specified type parameter

    // Explicitly specified type parameter `char` to `generic()`
    generic::<char>(SingleGeneric('a'));

    // Implicitly specified type parameter `char` to `generic()`
    generic(SingleGeneric('a'));
}

// ## 14.2 Implementations

// Similar to functions, implementaitons require care to remain generic.

struct S; // Concrete type S
struct GenericVal<T>(T); // Generic type `GenericVal`

// impl of GenericVal where we explicitly speify type parameters:
impl GenericVal<f32> {} // Specify `f32`
impl GenericVal<S> {} // Specify `S` as defined aove

// `<T>` Must precede the type to remain generic
impl<T> GenericVal<T> {} // Generic implementation for any `T`

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// impl of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// impl of GenVal for generic type `T`
impl<T> GenVal<T> {
    fn gen_val(&self) -> &T {
        &self.gen_val
    }
}

fn showcase_generic_impl() {
    let x = Val { val: 3.14 };
    let y = GenVal { gen_val: 3.14 };

    println!("Val: {}", x.value());
    println!("GenVal: {}", y.gen_val());
}

// ### 14.3 Traits

// Of course, `traits` can also be generic. Here we define one which reimplements `Drop` as a generic method to `drop` itself and an input.

// Non-copyable types
struct Empty;
struct Null;

// A trait generic over `T`
trait DoubleDrop<T> {
    // Define a method on the caller type which takes an additional single paramter `T` and does nothing with it
    fn double_drop(self, _: T);
}

// Implement `DoubleDrop<T>` for any generic parameter `T` and caller `U``
impl<T, U> DoubleDrop<T> for U {
    // This method takes ownership of both passed arguments, deallocating both when the method ends
    fn double_drop(self, _: T) {}
}

fn showcase_generic_traits() {
    let empty = Empty;
    let null = Null;

    // Deallocate`empty` and `null`
    empty.double_drop(null);

    // empty;
    // null;
    // ^ try uncommenting these lines
}

// ## 14.4 Bounds

// When working with generics, the type parameters often must use traits as bounds to stipulate what functionality a type implements.
// For example, the following example uses the trait `Display` to print and so it requires `T` to be bound by `Display`
// That is, `T` must implement `Display` to be used as a valid type parameter.

/// Define a function `printer` that takes a generic type `T` which is bound by the `Display` trait
fn printer<T: std::fmt::Display>(t: T) {
    println!("{}", t);
}

// Bounding restricts the generic to types that conform to these bounds.

struct S2<T: std::fmt::Display>(T);

// Error! `Vec<T>` does not implement `Display`. This specialization will fail
// let s = S2(vec![1, 2, 3]);

// Another effect of bounding is that generic instances are allowed to access the methods of traits specified in the bounds.

// A trait which implements the print marker: `{:?}`
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}
#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// The generic `T` must implement `Debug`. Regardless of the type, this will work properly.
fn print_debug<T: Debug>(t: T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets the bound can access `HasArea`'s function `area`
fn area<T: HasArea>(t: T) -> f64 {
    t.area()
}

fn showcase_trait_bounds() {
    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", area(rectangle));

    // print_debug(&_triangle); // Error! `Triangle` does not implement `Debug`
    // println!("Area: {}", area(_triangle)); // Error! `Triangle` does not implement `HasArea`
}

// Note: `where` clause can also be used to apply bounds in some cases to be more expressive

// ### 14.4.1 Testcase: Empty Bounds

// A consequence of how bounds work is that even if a trait doesn't include any functionality, you can still use it as a bound.
// `Eq` and `Copy` are examples of such traits from the `std` library.

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// These functions are only valid for types which implement these traits. The fact that the traits are empty is irrelevant.
fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str {
    "blue"
}

fn showcase_trait_empty_bounds() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    // `red()` won't work on a blue jay nor vice versa because of the bounds
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey)); // Error! `Turkey` does not implement `Red`
    // println!("A turkey is {}", blue(&_turkey)); // Error! `Turkey` does not implement `Blue`
}

// ## 14.5 Multiple Bounds

// Multiple bounds for a single type can be applied with a `+`. Like normal, different types are separated with `,`

use std::fmt::Display;

fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: {:?}", t);
    println!("Display: {}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
    println!("t: {:?}, u: {:?}", t, u);
}

fn showcase_multiple_bounds() {
    let string = "words";
    let array = [1, 2, 3];
    let vec = vec![1, 2, 3];

    compare_prints(&string);
    // compare_prints(&array); // Error! `[i32; 3]` does not implement `Display`

    compare_types(&array, &vec);
}

// ## 14.6 Where clauses

// A bound can also be expressed using a `where` clause immediately before opening the function body with `{`, rather than at the type's first mention.
// Additionally, `where` clauses can apply bounds to arbitrary types, rather than just to type parameters.

// When specifying generic types and bounds spearately is cleaner
// impl<A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}
// Expressing bounds with a where clause
// impl<A, D> MyTrait<A, D> for YourType where
//   A: TraitB + TraitC,
//   D: TraitE + TraitF
// {
//    // method definitions go here
// }

// When using a `where` clause is more expensive than using normal syntax. The `impl` in this example cannot be directly expressed without a `where` clause.

trait PrintInOption {
    fn print_in_option(self);
}

// Becuase we would otherwise have to express this as `T: Debug` or use another method of indirect appraoch, this requries a `where` clause
impl<T> PrintInOption for T
where
    Option<T>: Debug,
{
    // We want `Option<T>: Debug` as our bound because that is what's being printed. Doing otherwise would be using the wrong bound
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn showcase_where_clauses() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
}

// ## 14.7 New Type Idiom

// The `newtype` idiom gives compile time guarantees that the right type of value is supplied to a program.

// For example, an age verification function that checks age in years, must be given a value of type `Years`

struct Years(i64);
struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

fn showcase_newtype_idiom() {
    let age = Years(5);
    let age_days = age.to_days();
    println!("Is an adult? {}", is_adult(&age));
    println!("Is an adult? {}", is_adult(&age_days.to_years()));
    // println!("Is an adult? {}", is_adult(&age_days)); // Error! `is_adult()` requires a `Years` type, not `Days`

    // to obtain the newtype's value as the base type, you may use the tuple or destructuring syntax
    let Years(age_in_years) = age;
    println!("Age in years: {}", age_in_years);
    let years_as_primitive = age.0;
    println!("Age in years: {}", years_as_primitive);
}

// ## 14.8 Associated Items

// Associated items refer to a set of rules pertaining to items of various types. It is an extension to `trait` generics, and allows `traits` to internally define new items.

// One such item is called an associated type, providing simpler usage patterns when the trait is generic over its container type.

// A trait that is generic over its container type has type specifications requirements - users of the trait must specify all of its generic types.

// The `Contains` trait below, allows the use of generic types A and B. The trait is then implemented for the `Container` type, specifying `i32`
// for `A` and `B` so that it can be used with `fn difference()`

// Because `Contains` is generic, we are forced to explicitly state all of the generic types for `fn difference()`.
// In practice, we want a way to express that `A` and `B` are determined by the input `C`.

struct Container(i32, i32);

// A trait which checks if 2 items are stored inside of a container
// Also retrieves first or last value
trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool; // Explicitly requries `A` and `B`.
    fn first(&self) -> i32; // Doesn't explicitly require `A` or `B`
    fn last(&self) -> i32; // Doesn't explicitly require `A` or `B`
}

impl Contains<i32, i32> for Container {
    // True if the numbers stored are equal
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // Grab the first number
    fn first(&self) -> i32 {
        self.0
    }

    // Grab the last number
    fn last(&self) -> i32 {
        self.1
    }
}

// `C` contains `A` and `B`. In light of that, having to express `A` and `B` again is a nuisance
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn showcase_associated_items() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!(
        "Does container contain {} and {}: {}",
        number_1,
        number_2,
        container.contains(&number_1, &number_2)
    );

    println!("The first number is: {}", container.first());
    println!("The last number is: {}", container.last());
    println!("The difference is: {}", difference(&container));
}

// ### 14.8.2 Associated Types

// The use of "Associated Types" improves the overall readability of code by moving inner types locally into a trait as output types.

// `A` and `B` are defind in the trait via the `type` keyword.
// Note: `type` in this context is different from `type` when used for aliases
trait Contains2 {
    type A;
    type B;

    // Updated syntax to refer to these new types generically
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

// Note that functions that use the trait `Contains2` are no longer requried to express `A` or `B` at all

// Without using associated types
fn difference_old<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}

fn difference2<C: Contains2>(container: &C) -> i32 {
    container.last() - container.first()
}

//# # 14.9 Phantom Type Parameters

// A Phantom type parameter is one that doesn't show up at runtime, but is checked statically (and only) at compile time.

// Data types can use extra generic type parameters to act as markers or to perform type check at compile time.
// These extra parameters hold no storage values, and have no runtime behaviour.

use std::marker::PhantomData;

// A phantom tuple struct which is generic over `A` with hidden parameter `B`
#[derive(PartialEq)] // All equality test for this type
struct PhantomTuple<A, B>(A, PhantomData<B>);

// A phantom type struct which is generic over `A` with hidden parameter `B`
struct PhantomStruct<A, B> {
    value: A,
    phantom: PhantomData<B>,
}

// Note: Storage is allocated for generic type `A`, but not for `B`. Therefore `B` cannot be used in computations

fn showcase_phantom_types() {
    // Here `f32` and `f64` are hidden parameters.
    // PhantomTuple type specified as `<char, f32>`
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // PhantomTuple type specified as `<char, f64>`
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // Type specified as `<char, f32>`
    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        value: 'Q',
        phantom: PhantomData,
    };
    // Type specified as `<char, f64>`
    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        value: 'Q',
        phantom: PhantomData,
    };

    // Compile-time Error! Type mismatch so these cannot be compared
    // println!("_tuple1 == _tuple2 yields: {}", _tuple1 == _tuple2);

    // Compile-time Error! Type mismatch so these cannot be compared
    // println!("struct1 == struct2 yields: {}", _struct1 == _struct2);
}

// A useful method of unit conversion can be examined by implementing `Add` with a phantom type parameter.

// This construction would impose: `Self + RHS = Output`
// Where RHS defaults to `Self` if not specified in the implementation
pub trait MyAdd<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}

use std::ops::Add;

// Create void enumerations to define unit types
#[derive(Debug, Clone, Copy)]
enum Inch {}
#[derive(Debug, Clone, Copy)]
enum Mm {}

// `Length` is a type with phantom type parameter `Unit`, and is not generic over the length type (that is `f64`)
// `f64` already implements the `Clone` and `Copy` traits
#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

// The `Add` trait defines the behaviour of the `+` operator.
impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    // `add()` returns a new `Length` struct containing the sum
    fn add(self, rhs: Length<Unit>) -> Length<Unit> {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn showcase_phantom_add() {
    // Specifies `one_foot` to have phantom type parameter `Inch`
    let one_foot: Length<Inch> = Length(12.0, PhantomData);

    // `one_meter` has phantom type parameter `Mm`
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    // `+` calls the `add()` method we implemented for `Length<Unit>`
    //
    // Since `Length` implements `Copy`, `add()` does not consume `one_foot` and `one_meter` but copies them into `self` and `rhs`.
    let two_feet = one_foot + one_foot;
    let two_meters = one_meter + one_meter;

    // Addition works
    println!("one_foot + one_foot = {:?}", two_feet);
    println!("one_meter + one_meter = {:?}", two_meters);

    // Nonsensical operations fail as they should:
    // println!("one_foot + one_meter = {:?}", one_foot + one_meter); // Error! Mismatched types
}
