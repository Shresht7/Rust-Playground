// Errors are a fact of life! even in software. Rust requires you to acknowledge the possibility of errors and take action at compile time.

// Rust groups errors into two major types: recoverable and unrecoverable errors.
// Rust doesn't have exceptions. Instead it has the type Result<T, E> for recoverable errors and panic! for unrecoverable errors.

// Sometimes bad things happen in your code. Very bad things; and there's nothing you can do about it. (but panic!)
// For these cases, Rust has the panic! macro. The panic! macro will print a failure message, unwind, cleanup the stack and quit.
// Via environment variables, you can also have Rust display the call stack when a panic occurs to make it easier to track down
// the source of the panic.

// panic!("crash and burn");

// By default, when a panic occurs the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.
// However, walking back and cleaning up is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.
// Memory that the program was using will then need to be cleaned up by the operating system.
// If in your project you need to make the resultant binary as small as possible, you can switch from unwinding to aborting upon a panic
// by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file.
// For example, if you want to abort on panic in release mode, add this:
// [profile.release]
// panic = 'abort'

// Most errors are not serious enough to warrant the program to stopping entirely.
// Sometimes when a function fails it’s for a reason that you can easily interpret and respond to
// For this we have the Result<T, E> enum in Rust:
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

using std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}

// The unwrap method is a shortcut method. If the Result value is the Ok variant, unwrap will return the value inside the Ok.
// If the Result is the Err variant, unwrap will call the panic! macro for us.
// Similarly, the expect method lets us also choose the panic! error message.
// Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier. 

// In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed.
// That way, if your assumptions are ever proven wrong, you have more information to use in debugging.
