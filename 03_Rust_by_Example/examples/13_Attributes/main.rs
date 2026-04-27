#![allow(unused_variables, dead_code)]

// # 13 Attributes

// An attribute is metadata applied to some module, crate or item. This metadata can be used to/for:
// - conditional compilation of code.See https://doc.rust-lang.org/rust-by-example/attribute/cfg.html
// - set crate name, version and type (binary or library). See https://doc.rust-lang.org/rust-by-example/attribute/crate.html
// - disable lint warnings. https://en.wikipedia.org/wiki/Lint_%28software%29
// - enable compiler features (macros, glob imports, etc)
// - link to a foreign library
// - mark functions as unit tests
// - mark functions that will be part of a benchmark
// - attribute like macros. https://doc.rust-lang.org/book/ch19-06-macros.html#attribute-like-macros

// Attributes look like `#[outer_attribute]` or `#![inner_attribute]`, with difference between them being where they apply.

// The `#[outer_attribute]` applies to the item that immediately follows it. For example, a function, a module declaration, a constant, a struct, or an enum declaration.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// The `#![inner_attribute]` applies to th enclosing item (typically a module or a crate). In other wirds, this arritbute is interpreted as
// applying to the entire scope in which it's placed. Here's an example where `#![allow(unused_variables)]` applies to the whole crate (if placed in main.rs)

// #![allow(unused_variables)]

fn main() {
    let x = 3; // This would normally warn about an unused variable.
    showcase_conditional_compilation();
}

// Attributes can take arguments with different syntaxes:
// - #[arribute = "value"]
// - #[attribute(key = "value")]
// - #[attribute(value)]

// Attributes can have multiple values and can be separated over multiple lines, too:
// #[attribute(value1, value2, value3)]
// #[attribute(
//     value1,
//     value2,
//     value3,
// )]

// ## 13.1 `dead_code`

// The compiler provides a `dead_code` lint that will warn unused functions. An attribute can be used to disable the lint.

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {
    println!("This function is never called.");
}

// Note: In real programs, you should eliminate dead code.

// ## 13.2 Crates

// The `crate_type` attribute can be used to tell the compiler whether a crate is a binary or a library (and even which type of library),
// and the `crate_name` attribute can be used to set the name of the crate.

// However, it is important to note that both the `crate_type` and `crate_name` attributes have no effect whatsoever when using Cargo, the Rust package manager.
// Since Cargo is used for the majority of Rust projects, this means real-world uses of crate_type and crate_name are relatively limited.

// This is a library
// #![crate_type = "lib"]
// The library is named "rary"
// #![crate_name = "rary"]

// When the `crate_type` attribute is used, we no longer need to pass the `--crate-type` flag to `rustc`

// ## 13.3 cfg

// Configuration conditional checks are possible through two different operators:
// - the `cfg` attribute: `#[cfg(...)]` in attribute position
// - the `cfg!` macro: `cfg!(...)` in boolean expressions

// While the former enames conditional compilation, the latter conditionally evaluated to `true` or `false` literals allowing for checks at run-time.
// both utilize identical argument syntax.

// `cfg!`, unlike `#[cfg]`, does not remove any code and only evaluates to true or false. For example, all blocks in an if/else expression
// need to be valid when `cfg!` is used for the condition, regardless of what `cfg!` is evaluating.

// This function only gets compiled if the target OS is linux
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is *not* linux
#[cfg(not(target_os = "linux"))]
fn are_you_not_on_linux() {
    println!("You are *not* running linux!");
}

fn showcase_conditional_compilation() {
    are_you_not_on_linux();
}

// ### 13.3.1 Custom

// Some conditionals like `target_os` are implicitly provided by `rustc`, but custom conditionals must be passed to `rustc` using the `--cfg` flag.
