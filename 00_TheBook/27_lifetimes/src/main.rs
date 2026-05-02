// # Validating References with Lifetimes

// Lifetimes are another kind of generic.
// They ensure that references are valid as long as we need them to be.

// Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
// Most of the time, lifetimes are implicit and inferred, just like, most of the time, types are inferred.

// Rust requires us to annotate the relationship using generic lifetime parameters to ensure that actual references used at runtime will definitely be valid.

// Annotating lifetimes is not even a concept most other programming languages have, so this is going to feel unfamiliar.

// ## Dangling References

// The main aim of lifetimes is to prevent dangling references, which, if they were allowed to exist, would cause a program to reference data other than the ata it's intended to referene.

// The Rust compiler has a _borrow_checker_ that compares scopes to determine whether all borrows are valid.

// ## Generic Lifetimes in Functions

// ## Lifetime Annotation Syntax

// Lifetime annotations don't change how long any of the references live. Rather, they describe the relationship of the lifetimes of multiple references to each other
// without affecting the lifetimes.

// Functions can accept references with any lifetime by specifying a generic lifetime parameter.

// Lifetime annotations have a slightly unusual syntax: The names of lifetime parameters must start with an apostrophe (`'`) and are all usually lowercase and very short.
// &i32 // a reference
// &'a i32 // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// One lifetime annotation by itself doesn't have much meaning, because annotations are meant to tell Rust how generic lifetime parameters of multiple references related to each other.

// ## In Function Signatures

// To use lifetime annotations in function signatures, we need to declare the generic lifetime parameter inside angle brackets between the function name and the parameters, just like generics.

// We want the signature to express the following constraint: The returned reference will be valid as long as both of the parameters are valid.
// This is the relationship between lifetimes of the parameters and the return value.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a.
// The function signature also tells Rust that the string slice returned from the function will live at least as long as the lifetime of 'a.
// In practice, it means that lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function.
// These relationships are what we want Rust to use when analyzing this code.

// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they're connected, Rust has enough information to allow
// memory-safe operations and disallow the operations that would create dangling pointers or otherwise violate memory safety.

// In struct definitions

// We can define structs to hold references, but in that case, we would need to specify a lifetime annotation every reference in the struct's definition.

struct ImportantExcerpt<'a> {
    parts: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        parts: first_sentence,
    };
}

// This struct has the single field part that holds a string slice, which is a reference.
// As with generic data-types we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct.
// This annotation means that an instance of ImportantExcerpt can't outlive the reference it holds in its part field.

// The main function here create an instance of the ImportantExcerpt struct that holds a reference to the first sentence of the String owned by the variable novel.
// The data in novel exists before the ImportantExcerpt instance is created.In addition, novel doesn't go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid.

// ## The Static Lifetime

// One special lifetime is the 'static lifetime, which denotes that the affected reference can live for the entire duration of the program.
// All string literals have the 'static lifetime, which we can annotate as follows:
let s: &'static str = "I have a static lifetime.";

// The text of this string is stored directly in the program's binary, which is always available.Therefore, the lifetime of all string literals is 'static.
