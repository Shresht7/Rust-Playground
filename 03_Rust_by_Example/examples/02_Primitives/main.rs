#![allow(unused_variables, unused_assignments, unused_parens)]

// # Primitives

// ## 2.1 Scalar Types

// - **Signed Integers**: `i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer-size)
// - **Unsigned Integers**: `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer-size)
// - **Floating-Point Types**: `f32` and `f64` (the default)
// - **Boolean Type**: `bool` (values are either `true` or `false`)
// - **Character Type**: `char` (represents a Unicode Scalar Value) like 'a', 'α', '∞', '😀', etc (4 bytes each).
// - **Unit Type**: `()` (represents an empty value or an empty tuple). Despite the value of a unit type being a tuple, it is not considered a compound type as it does not contain multiple values.

// ## 2.2 Compound Types

// - **Array**: `[T; N]` (where `T` is the type of the elements and `N` is the number of elements)
// - **Tuple**: `(T1, T2, T3, ...)` (where `T1`, `T2`, `T3`, etc. can be different types)

fn main() {
    // Variables can be type annotated, or the compiler can infer it based on the value and how it's used.
    let a = 42; // i32
    let b: f64 = 3.14; // f64
    let c: bool = true; // bool
    let d: char = 'x'; // char
    let e: () = (); // unit type
    let f: [i32; 3] = [1, 2, 3]; // array of 3 i32s
    let g: (i32, f64, char) = (42, 3.14, 'x'); // tuple of (i32, f64, char)

    // Numbers may additionally be annotated via a suffix
    let suffix_annotation = 42u32; // u32

    // A mutable variables value can be changed
    let mut mutable = 42; // i32
    mutable = 43;

    // Variables can also be shadowed by reusing the same name
    let shadowed = 42; // i32
    let shadowed = "Now I'm a string!"; // &str

    // ## 2.1 Literals and Operators

    // Integers `1`, Floats `1.2`, Characters `a`, Strings `abc`, Booleans `true/false` and the Unit type `()` can be expressed as literals.

    // Integers can, alternatively, be expressed in hexadecimal (base 16), octal (base 8) or binary (base 2) nomations using the prefixes `0x`, `0o` and `0b` respectively.

    // Underscores can be inserted in numeric literals to improve readability. e.g. 1_000 is the same as 1000. and 0.000_001 is the same as 0.000001.
    let million = 1_000_000;
    let small = 0.000_001;

    // Integers can also be expressed using the scientific notation. e.g. 1e7 is the same as 10_000_000
    let scientific = 1e7; // 10_000_000.0

    // Arithmetic Operators
    println!("1 + 2 = {}", 1 + 2); // 3
    println!("5 - 3 = {}", 5 - 3); // 2
    println!("4 * 2 = {}", 4 * 2); // 8
    println!("10 / 2 = {}", 10 / 2); // 5
    println!("10 % 3 = {}", 10 % 3); // 1

    // Short-circuiting boolean logic
    println!("true && false: {}", true && false); // false
    println!("true || false: {}", true || false); // true

    // Bitwise Operators
    println!("0b1010 & 0b1100 = {:04b}", 0b1010 & 0b1100); // 1000
    println!("0b1010 | 0b1100 = {:04b}", 0b1010 | 0b1100); // 1110
    println!("0b1010 ^ 0b1100 = {:04b}", 0b1010 ^ 0b1100); // 0110
    println!("!0b1010 = {:04b}", !0b1010); // 0101
    println!("0b1010 << 1 = {:04b}", 0b1010 << 1); // 0100
    println!("0b1010 >> 1 = {:04b}", 0b1010 >> 1); // 0101

    // 2.2 Tuples
    show_tuple();

    // 2.3 Arrays and Slices
    show_arrays_and_slices();
}

// ## 2.2 Tuples

// A tuple is a collection of values of different types. Tuples are constructed using parentheses `()`, and each tuple itself is a value with type signature `(T1, T2, T3, ...)`
// Functions can use tuples to return multiple values, as tuples can hold any number of values

// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn show_tuple() {
    // A tuple can have any number of any type of values.
    let tuple = (1, "hello", 4.5);
    println!("Tuple: {:?}", tuple); // Tuple: (1, "hello", 4.5)

    // Values can be extracted from a tuple using tuple indexing, which starts at 0.
    let first = tuple.0; // 1
    let second = tuple.1; // "hello"
    let third = tuple.2; // 4.5

    // Tuples can be tuple members
    let nested_tuple = (1, (2, 3), 4);
    println!("Nested Tuple: {:?}", nested_tuple); // Nested Tuple: (1, (2, 3), 4)

    // Caution: Long tuples (more than 12 elements) cannot be printed with the `{:?}` format specifier

    // Tuples can be returned from functions
    reverse((42, true)); // returns (true, 42)

    // To create a one-element tuple, you need to include a trailing comma after the value. Otherwise, it will be interpreted as just the value in parentheses.
    let single_element_tuple = (42,); // This is a tuple containing the value 42
    let not_a_tuple = (42); // This is just the value 42 in parentheses, not a tuple

    // Tuples can be destructured in a `let` statement to bind their values to variables
    let (x, y, z) = tuple;
    println!("Destructured Tuple: x = {}, y = {}, z = {}", x, y, z); // Destructured Tuple: x = 1, y = hello, z = 4.5
}

// ## 2.3 Arrays and Slices

// An array is a collection of objects of the same type T, stored in contiguous memory.
// Arrays are created using the square bracket `[]` syntax, and their length, which is known at compile time, is part of their type signature: `[T; N]` where `T` is the type of the elements and `N` is the number of elements in the array.

// Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-word object, the first word is a pointer to the data,
// the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture (64 bits on x86-64).
// Slices can be used to borrow a section of an array and have the type signature &[T].

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("Length of the slice: {}", slice.len());
}

fn show_arrays_and_slices() {
    // Fixed-size arrays (type signature is superfluous)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialized to the same value
    let ys: [i32; 5] = [0; 5]; // [0, 0, 0, 0, 0]

    // Indexing starts at 0
    println!("First element of xs: {}", xs[0]); // 1
    println!("Second element of xs: {}", xs[1]); // 2

    // The `len` returns the count of elements in the array
    println!("Length of xs: {}", xs.len()); // 5

    // Arrays are stack allocated
    println!("Size of xs in bytes: {}", std::mem::size_of_val(&xs)); // 20 (5 elements * 4 bytes each)

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice:");
    analyze_slice(&xs); // First element of the slice: 1, Length of the slice: 5

    // Slices can also point to a section of an array.
    // they are often of the form [start_index..end_index].
    // `start_index` is the first position of the slice inclusive.
    // `end_index` is the last position of the slice exclusive.
    println!("Borrow a section of the array as a slice:");
    analyze_slice(&xs[1..4]); // First element of the slice: 2, Length of the slice: 3

    // Example of empty slice `&[]`
    let empty_array: [u32; 0] = [];
    assert_eq!(empty_array.len(), 0);

    // Arrays can be safely accessed using the `.get` method, which returns an `Option<&T>`.
    // This can be pattern-matched as shown below, or used with `.expect()`.
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(value) => println!("Element at index {}: {}", i, value),
            None => println!("No element at index {}", i),
        }
    }

    // Out of bounds indexing on array with constant value causes a compile time error
    // println!("{}", xs[5]); // error: index out of bounds: the len is 5 but the index is 5

    // Out of bound indexing on slices cause a runtime error (panic)
    // let slice = &xs[1..4];
    // println!("{}", slice[3]); // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 3
}
