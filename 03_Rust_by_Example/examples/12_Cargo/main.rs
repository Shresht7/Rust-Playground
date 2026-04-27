// # 12 Cargo

// `cargo` is the official Rust package management tool. It has lots of really useful features to improve code quality and developer velocity.
// - Dependency management and integartion with `crates.io` (the official Rust package registry)
// - Awareness of unit tests
// - Awareness of benchmarks

// See the Cargo Book for more details: https://doc.rust-lang.org/cargo/

// ## 12.1 Dependencies

// Most programs have dependencies on some libraries. If you have ever managed dependencies by hand, you know how much of a pain this can be.
// Luckily, the Rust ecosystem comes standard with cargo!. `cargo` can manage dependencies for a project.

// To create a new Rust project, you can:
// ```sh
// # A binary
// cargo new foo
//
// # A library
// cargo new --lib bar
// ```

// After the above commands, you should see a file heirarchy like this:
// ```
// .
// ├── bar
// │   ├── Cargo.toml
// │   └── src
// │       └── lib.rs
// └── foo
//     ├── Cargo.toml
//     └── src
//         └── main.rs
// ```

// The `main.rs` is the root source file for your new `foo` project - nothing new here. The `Cargo.toml` is the config file for the `cargo` for this project.
// ```toml
// [package]
// name = "foo"
// version = "0.1.0"
// authors = ["mark"]
// edition = "2024"
//
// [dependencies]
// ...
//```

// The `name` field under `[package]` determines the name of the project. This is used by `crates.io` if you publish the crate.
// It is also the name of the output binary when you compile.

// The `version` field is a crate version using Semantic Versioning.

// The `authors` field is a list of authors used when publishing the crate.

// The `dependencies` section lets you add dependencies for your project.

// ```toml
// [dependencies]
// clap = "2.27.1" # from crates.io
// rand = { git = "https://github.com/rust-lang-nursery/rand" } # from online repo
// bar = { path = "../bar" } # from local path
// ```

// cargo is more than a dependency manager. It also provides a lot of other features, such as running tests and benchmarks, building documentation, and more. See the Cargo Book for more details: https://doc.rust-lang.org/cargo/

// To build our project we can execute `cargo build` anywhere in the project directory (including subdirectories).
// We can also do `cargo run` to build and run.
// These commands will resolve all dependencies, download crates if needed, and build everything, including you crate.
// Note that it only rebuilds what it has not already built, similar to `make`

fn main() {
    println!("Cargo is Awesome!");
}

// ## 12.2 Conventions

// Cargo supports having two binaries in the same project. The default binary name is `main`, but you can add addtional binaries by placing them in a `bin/` directory.
// ```
// foo
// ├── Cargo.toml
// └── src
//     ├── main.rs
//     └── bin
//         └── my_other_bin.rs
// ```

// To tell `cargo` to only compile and run this binary, we just pass cargo the `--bin my_other_bin` flag where `my_other_bin` is the name of the binary file.

// ## 12.3 Testing

// As we know testing is integral to any piece of software! Rust has first-class support for unit and integration testing.

// Each file in a `tests` folder is a separate integration test, i.e. a test that is meant to test your library as if it was being called from a dependent crate.

// `cargo` naturally provides an easy way to run all of your tests!
// ```sh
// cargo test
// ```

// you can also run tests whose name matches a pattern:
// ```sh
// cargo test some_test_name
// ```

// Caution: Cargo may run multiple tests concurrently, so make sure that they don't race with each other.

// ## 12.4 Build Scripts

// Sometimes a normal build from `cargo` is not enough. Perhaps your crate needs some pre-requisites before `cargo` will successfully compile,
// things like code generation, or some native code that needs to be compiled. To solve this problem we have build scripts that Cargo can run.

// To add a build script to your package, it can either be speciied in the `Cargo.toml` as follows:
// ```toml
// [package]
// ...
// build = "build.rs"
// ```

// Otherwise, Cargo will look for a file named `build.rs` in the root of your package and use it as the build script by default, if it exists.

// ### How to use a build script

// The build script is simply another Rust file that will be compiled and invoked prior to compiling anything else in the package.
// Hence, it can be used to fulfill pre-requisites of your crate.

// Cargo provides the script with inputs via environment variables that can be used.

// The script provides output via stdout. All lines printed are written to `target/debug/build/<pkg>/output`.
// Further, lines prefixed with `cargo`: will be interpreted by Cargo directly and hecnce can be used to define parameters for the package's compilation.
