// # 11 Crates

// A crate is a compilation unit in Rust.
// Whenever `rustc some_file.rs` is called, `some_file.rs` is treated as the `crate` file. If `some_file.rs` has `mod` declarations in it,
// then the contents of the module files would be inserted in places where the mod declarations in the crate file are found, before running the compiler over it.
// In other words, modules do not get compiled individually, only crates get compiled.

// ## 11.1 Creating a Library

// A crate can be compiled into a binary or a library. By default, `rustc` will produce a binary from a crate.
// To produce a library, you can use the `--crate-type` flag with `rustc`:
// ```bash
// rustc --crate-type=lib some_file.rs
// ```

mod my_crate {
    pub fn public_function() {
        println!("called `my_crate::public_function()`");
    }

    fn private_function() {
        println!("called `my_crate::private_function()`");
    }

    pub fn indirect_access() {
        print!("called `my_crate::indirect_access()`, that\n> ");
        private_function();
    }
}

// ```bash
// $ rustc --crate-type=lib main.rs
// $ ls lib*
// libmain.rlib
// ```

// Libraries get prefixed with "lib", and by default they get named after their crate file, but this default name can be overriden by passing
// the `--crate--name` option to `rustc` or by using the `crate_name` attribute.

fn main() {
    my_crate::public_function();
    my_crate::indirect_access();
    showcase_library_use();
}

// ## 11.2 Using a Library

// To link a crate to this new library you may use `rustc`'s `--extern` flag. All of its items will then be imported under a module named the same as the library.
// This module generally behaves the same way as any other module.

// extern crate my_crate; // May be required for Rust 2015 edition or earlier. But not needed for Rust 2018 edition or later.

fn showcase_library_use() {
    my_crate::public_function();

    // Error! `private_function` is private
    // my_crate::private_function();

    my_crate::indirect_access();
}
