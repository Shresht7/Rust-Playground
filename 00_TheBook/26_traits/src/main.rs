// Defining Shared Behavior with Traits

// A trait defines the functionality a particular type has and can share with other types.
// We can use traits to define shared behavior in an abstract way.
// We can use trait bounds to specify that a generic type can be any type that has certain behavior.

// Note: Traits are similar to a feature often called _interfaces_ in other languages, although with some differences.

// ## Defining a Trait

// A type's behavior consists of methods we can call on that type.
// Different types share the same behavior if we can call the same emthods on all of those types.
// Trait definitions are a way to group method signatures together to define a set of behaviours necessary to accomplish some purpose.

pub trait Summary {
    fn summarize(&self) -> String;
}

// Here, we declare a trait using the `trait` keyword and then the trait's name.
// Inside the curly brackets, we declare the method signatures that describe the behaviours of the types that implement this trait

// After the method signature, instead of providing an implementation within curly brackets, we can use a semicolon.
// Each type implementing the trait must provide its own implementation of the method. The compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.

// A trait can have multiple methods in its body: The method signatures are listed one per line, and each line ends in a semicolon.

// ## Implementing a Trait on a Type

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Implementing a trait on a type is similar to implementing regular methods. The difference is that after impl, we put the trait name we want to implement,
// then use the for keyword, and then specify the name of the type we want to implement the trait for.
// Within the impl block, we put the method signatures that the trait definition has defined.
// Instead of ending the method signature with a semicolon, we provide the method body with an implementation for that type.

fn main_v1() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as your probably already know, people"),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize());
}

// Other crates will need to bring in the `Summary` trait into scope to use the `summarize` method.

// NOTE: One restriction to note is that we can implement a trait on a type only if either the trait or the type, or both, are local to our crate.
// !! But we can't implement external traits on external types. e.g. we can't implement the `Display` trait on `Vec<T>`.
// !! This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present.
// !! This rule ensures that other people's code can't break your code and vice-versa.
// !! Without this rule, two crates could implement the same trait for the same type, and Rust wouldn't know which one to use.

// ## Default Implementations

// Sometimes it's useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.
// Then, as we implement the trait on a particular type, we can keep or override each method's default behaviour as needed.

pub trait SummaryV2 {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// To use a default implementation without overrides, we can simply provide an empty impl block for the trait on the type.
impl SummaryV2 for NewsArticle {}

// Default implementations can call other methods in the same trait, even if those other methods don't have default implementations.
// In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.

pub trait SummaryV3 {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// To use this version of summary, we only need to define `summarize_author` when we implement the trait
impl SummaryV3 for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

// ## Using Traits as Parameters

// We use traits to define functions that accept many different types. To do this, we use the `impl Trait` syntax.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name.
// This parameter accepts any type that implements the specific trait. In the body of notify, we can call any methods on item
// that come from the Summary trait, such as summarize.

// ## Trait Bound Syntax

// The `impl Trait` Syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound.
pub fn notify_v2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// ## Multiple Trait Bounds with the + Syntax

// We can also specify more than one trait bound. Say we wanted `notify` to use display formatting as well as summarize on item.
// We specify that item must implement both the `Summary` and `Display` traits using the + syntax.
pub fn notify_v3(item: &(impl Summary + std::fmt::Display)) {
    println!("Breaking news! {}", item.summarize());
}

// or, better yet

pub fn notify_v4<T: Summary + std::fmt::Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// ## Clearer Trait Bounds with where Clauses

// Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple generic type parameters can contain
// lost of trait bound information between the function's name and its parameter list, making the function signature hard to read.
// For this reason, Rust has some alternate syntax for specifying trait bounds inside a where clause after the function signature

pub fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + std::fmt::Debug,
    U: std::fmt::Debug + std::clone::Clone,
{
    0
}

// ## Returning Types that Implement Traits

// We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait
fn return_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as your probably already know, people"),
        reply: false,
        repost: false,
    }
}

// By using impl Summary for the return type, we specify that the returns_summarizable function returns some type that implements the Summary trait without naming the concrete type.

// The ability to specify a return type only by the trait it implements is especially useful in the context of closures and iterators.
// Closures and iterator create types that only the compiler knows or types that are very long to specify.
// The impl Trait syntax lets you conscisely specify that a function returns some type that implements the Iterator trait without needing to write out a very long type.

// However, you can only use impl Trait, if you're returning a single type.

// Using Trait Bounds to Conditionally Implement Methods

// By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfy the trait bounds are called blanket implementations and are used extensively in the Rust standard library.
// For example, the standard library implements the `ToString` trait for any type that implements the `std::fmt::Display` trait.

// Because the standard library has this blanket implementation, we can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait.
