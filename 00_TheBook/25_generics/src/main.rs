// Every programming language has tools for effectively handling the duplication of concepts.
// In Rust, one such tool is generics: abstract stand-ins for concrete types or other properties.
// We can express the behavior of generics or how they relate to other generics without knowing what will be in their place when compiling and running the code.
// Functions can take parameters of some generic type, instead of a concrete type like i32 or String,
// in the same way they take parameters with unknown values to run the same code on multiple concrete values.

// Generics allow us to replace specific types with a placeholder that represents multiple types to remove code duplication.

// The follow two functions effectively do the same thing for different types...

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
fn largest_f32(list: &[f32]) -> &f32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// They can be combined into a single function with a generic
// To enable comparisons, the standard library has the std::cmp::PartialOrd trait that you can implement on types

fn largest<T: std::cmd::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generics can also be used on structs

struct Point<T> {
    x: T,
    y: T,
}

// and enums

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102.0, 34.0, 6000.5, 89.12, 54.34, 2.176, 43.1, 8.2];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
