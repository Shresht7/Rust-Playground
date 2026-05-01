#![allow(dead_code)]

// # Clone and Copy

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

fn main() {
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
