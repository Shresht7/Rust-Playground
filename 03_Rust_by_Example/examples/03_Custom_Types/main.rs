#![allow(dead_code)]

// # 3 Custom Types

// Rust allows you to define your own custom types using mainly two constructs:
// - `struct` (short for "structure") is a custom data type that lets you name and package together multiple related values
//   that make up a meaningful group. Each value in a struct is called a "field" and has its own name and type.
//   This is a product type, meaning that the struct's type is a combination of the types of its fields.
// - `enum` (short for "enumeration") is a custom data type that can be one of several different variants.
//  Each variant can have its own data associated with it, and the enum itself is a sum type,
// meaning that the enum's type is a choice between the types of its variants.

// Constants can also be created via the `const` and `static` keywords.

// ## 3.1 Structures

// There are basically three types of structures ("structs") that can be created using the `struct` keyword:
// - **Classic Structs**: These are the most common type of struct, where each field has a name and a type. Classic C style structs.
// - **Tuple Structs**: These are basically named tuples
// - **Unit Structs**: Which are field-less, and useful for generics.

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
#[derive(Debug)]
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
// A rectangle can be specified by where the top-left and bottom-right corners are in a cartesian plane
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn show_structs() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("Person: {:?}", peter); // Person: Person { name: "Peter", age: 27 }

    // Instantiate a `Point`
    let point1 = Point { x: 3.0, y: 4.0 };
    let point2 = Point { x: 5.0, y: 6.0 };
    println!("Point 1: {:?}", point1); // Point 1: Point { x: 3.0, y: 4.0 }
    println!("Point 2: {:?}", point2); // Point 2: Point { x: 5.0, y: 6.0 }

    // Access the fields of the point
    println!("Point 1 x: {}, y: {}", point1.x, point1.y); // Point 1 x: 3.0, y: 4.0
    println!("Point 2 x: {}, y: {}", point2.x, point2.y); // Point 2 x: 5.0, y: 6.0

    // Make a new point by using the struct update syntax to use the fields of our other one
    let point3 = Point { x: 7.0, ..point1 };
    println!("Point 3: {:?}", point3); // Point 3: Point { x: 7.0, y: 4.0 }

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point1;

    let rectangle = Rectangle {
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: point2,
    };
    println!(
        "Rectangle: top_left: {:?}, bottom_right: {:?}",
        rectangle.top_left, rectangle.bottom_right
    );
    // Rectangle: top_left: Point { x: 3.0, y: 4.0 }, bottom_right: Point { x: 5.0, y: 6.0 }

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    println!("Pair: {:?}", pair); // Pair: Pair(1, 0.1)

    // Access the fields of the tuple struct
    println!("Pair first: {}, second: {}", pair.0, pair.1); // Pair first: 1, second: 0.1

    // Destructure the tuple struct
    let Pair(integer, decimal) = pair;
    println!(
        "Destructured Pair: integer: {}, decimal: {}",
        integer, decimal
    ); // Destructured Pair: integer: 1, decimal: 0.1
}

// ## 3.2 Enums

// The `enum` keyword allows the creation of a type which may be one of a few different variants. Any variant which is a valid as a `struct` is also valid in an `enum`

// Create an enumeration to classify a web event. Note how both names and type information together specify the variant of the enum. Each is different and independent.
enum WebEvent {
    // Unit structs
    PageLoad,
    PageUnload,
    // Tuple structs
    KeyPress(char),
    Paste(String),
    // or C-style structs
    Click { x: i64, y: i64 },
}

// Functions can accept any of the enum variants as an argument
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("Page loaded"),
        WebEvent::PageUnload => println!("Page unloaded"),
        // Destructure c from inside the enum variant
        WebEvent::KeyPress(c) => println!("Pressed '{}'.", c),
        WebEvent::Paste(s) => println!("Pasted \"{}\".", s),
        // Destructure x and y from inside the enum variant
        WebEvent::Click { x, y } => {
            println!("Clicked at x={}, y={}.", x, y);
        }
    }
}

// ### Type Aliases

// You can refer to each enum variant, using a type alias. This might be useful if the enum name is too long or too generic and you want to rename it.
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// The most common place you'll see this is in the `impl` blocks using the `Self` alias
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// `enums` can also be used as C-like enums

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn show_enums() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // Using the type alias to refer to an enum variant
    let add = Operations::Add;
    match add {
        Operations::Add => println!("Adding numbers!"),
        Operations::Subtract => println!("Subtracting numbers!"),
    }

    // C-like enums can be cast to integers
    println!("Number::Zero as integer: {}", Number::Zero as i32); // Number::Zero as integer: 0
    println!("Color::Red as integer: {}", Color::Red as u32); // Color::Red as integer: 16711680
}

// ### Linked List
enum List {
    // Cons: A tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked-list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` is the empty list
        List::Nil
    }

    // Consume a list, and return the list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    // Returns the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behaviour of this method depends on the variant of `List` that `self` is referencing.
        // `self` has type `&List`, and `*self` has type `List`, matching on a concerete type `T` is preferred over matching on a reference `&T`
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed, instead take a reference to the tail
            List::Cons(_, ref tail) => 1 + tail.len(),
            // Base case: An empty list has length 0
            List::Nil => 0,
        }
    }

    // Returns representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                // `format!` is similar to `println!`, but returns a heap-allocated `String` instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            List::Nil => {
                format!("Nil")
            }
        }
    }
}

fn show_linked_list() {
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("Linked List has length: {}", list.len()); // Linked List has length: 3
    println!("Linked List representation: {}", list.stringify()); // Linked List representation: 3, 2, 1, Nil
}

// ## 3.3 Constants and Statics

// Rust has two different type of constants which can be declared in any scope, including global.Both require explicit type annoations:
// - `const`: An unchangeable value (the common use case)
// - `static`: A possibly mutable variable with `'static` lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is `unsafe`.

// Globals are declared outside of all other scopes
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access the constant in a function
    n > THRESHOLD
}

fn show_constants_and_statics() {
    println!("The language is: {}", LANGUAGE); // The language is: Rust
    println!("Is 16 bigger than the threshold? {}", is_big(16)); // Is 16 bigger than the threshold? true
    println!("Is 5 bigger than the threshold? {}", is_big(5)); // Is 5 bigger than the threshold? false
}

fn main() {
    show_structs();
    show_enums();
    show_linked_list();
    show_constants_and_statics();
}
