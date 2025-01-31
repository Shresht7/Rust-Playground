// The `if let` syntax lets you combine `if` and `let` into a less verbose way to handle values that match
// one pattern while ignoring the rest.

fn using_match() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
}

fn using_if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

// Using `if let` means less typing, less indentation, and less boilerplate code. However you lose the
// exhaustive checking that match enforces. We can include an else with an if let to handle the other cases.

fn main() {
    println!("Hello, world!");
}
