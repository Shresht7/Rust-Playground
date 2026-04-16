/// # Requests
///
/// > https://rocket.rs/guide/v0.5/requests/

/// ## Static Routes
/// Static routes are the simplest type of route. They match against a specific path and HTTP method.
/// In this example, we define a static route that matches `GET` requests to the root path ("/") and responds with a static string.

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world! 🚀"
}

/// ## Dynamic Path Parameters
/// Rocket also supports dynamic path parameters, which allow you to capture parts of the URL as variables.

/// In this example, we define a route that captures a `name` parameter from the URL and responds with a personalized greeting.
/// The `<name>` syntax in the route definition indicates that `name` is a dynamic path parameter.
/// When a request is made to a URL like `/hello/Alice`, the value "Alice" will be captured and passed to the `hello` function as the `name` argument.

#[rocket::get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}! 🚀", name)
}

/// Any number of dynamic path segments are allowed; and a Path segment can be of any type, even custom types,
/// as long as they implement the [`FromParam`][rocket::FromParam] trait. Rocket implements `FromParam` for many of the standard library types.

#[rocket::get("/hello/<name>/<age>/<cool>")]
fn cool_hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}! 🚀", name, age)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

/// ### Multiple Segments
/// You can also match against multiple path segments by using `<params..>` in a route path.
/// The type of such parameters, known as ***segment guards***, must implement the `FromSegments` trait.
/// A segment guard must be the final component of a route path, any text after a segment guard will result in a compile time errr.

#[rocket::get("/page/<path..>")]
fn get_page(path: std::path::PathBuf) -> String {
    format!("You requested the page at path: {}", path.display())
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![index, hello, cool_hello, get_page])
}
