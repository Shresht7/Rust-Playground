// # 4. Variable Bindings

// Rust provides type safety via static typing.
// Variable bindings can be type annotated when declared. However, in most cases, the compiler can infer the type from the context, heavily reducing the annotation burden.

// Values (like literals) can be bound to variables, using the `let` binding.

fn main() {
    showcase_variable_bindings();
    showcase_mutability();
    showcase_scope();
    showcase_shadowing();
    showcase_declare_first();
    showcase_freezing();
}

fn showcase_variable_bindings() {
    let an_integer = 1u32; // Type annotation via suffix
    let a_boolean = true; // Type inference
    let unit = (); // Type inference to unit type

    println!(
        "An integer: {}, a boolean: {}, unit value: {:?}",
        an_integer, a_boolean, unit
    );

    // Copy `an_integer` into `copied_integer`
    let cpoied_integer = an_integer;
    println!("Copied integer: {}", cpoied_integer);

    // The compiler warns about unused variable bindings; these warnings can be silenced by prefixing the variable name with an underscore
    let _unused_variable = 123;
}

// ## 4.1 Mutability

// Variable bindings are immutable by default. To make them mutable, the `mut` keyword can be used.

fn showcase_mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding); // Before mutation: 1
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding); // After mutation: 2

    // The compiler will prevent us from mutating an immutable binding, and will throw an error if we try to do so.
    // _immutable_binding += 1; // error: cannot assign twice to immutable variable
}

// ## 4.2 Scope and Shadowing

// Variable bindings have a scope, and are constrained to live in a block. A block is a collection of statements enclosed by `{}`

fn showcase_scope() {
    // This binding lives in this function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!(
            "inner long: {}, short: {}",
            long_lived_binding, short_lived_binding
        ); // inner long: 1, short: 2
    }
    // End of block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding); // error: cannot find value `short_lived_binding` in this scope

    println!("outer long: {}", long_lived_binding); // outer long: 1
}

fn showcase_shadowing() {
    let shadowed_binding = 1;

    {
        println!("Before shadowing: {}", shadowed_binding); // Before shadowing: 1

        // This binding shadows the previous one
        let shadowed_binding = "abc";

        println!("After shadowing: {}", shadowed_binding); // After shadowing: abc
    }
    println!("Outside of block: {}", shadowed_binding); // Outside of block: 1

    // Shadowing is also possible in the same scope
    let shadowed_binding = 2;
    println!("Shadowed again: {}", shadowed_binding); // Shadowed again: 2
}

// ## 4.3 Declare First

// It is possible to declare variable bindings first and initialize them later, but all variable bindings must be initialized before they are used.
// The compiler forbits the use of uninitialized variable bindings, as it would lead to undefined behavior.

// It is generally recommended to initialize variable bindings at the point of declaration,
// as it reduces the chances of accidentally using an uninitialized variable binding, and makes the code easier to read and understand.

fn showcase_declare_first() {
    // Declare a variable binding
    let a_binding;

    {
        let x = 2;
        a_binding = x * x; // Initialize the variable binding
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Use of uninitialized variable binding
    // println!("another binding: {}", another_binding);

    another_binding = 1; // Initialize the variable binding
    println!("another binding: {}", another_binding);
}

// ## 4.4 Freezing

// When data is bound by the same name immutably, it also freezes. Frozen data can't be modified until the immutability binding goes out of scope.

fn showcase_freezing() {
    let mut _mutable_integer = 7i32;

    {
        // Shadowing by immutable `_mutable_integer` freezes the original mutable binding until the end of this block
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        // _mutable_integer += 1; // error: cannot assign to `_mutable_integer`, as it is not declared as mutable

        // _mutable_integer goes out of scope here, so the original mutable binding is unfrozen
    }

    // Now we can modify the original mutable binding again
    _mutable_integer += 1;

    println!("mutable integer: {}", _mutable_integer); // mutable integer: 8
}
