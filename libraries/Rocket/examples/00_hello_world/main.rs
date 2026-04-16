//! A simple "Hello, world!" example using Rocket.
//!
//! Creates a web server that listens on `localhost:8000` and responds with "Hello, world! 🚀" when the root URL is accessed.
//!
//! Run this example using `cargo run --example 00_hello_world` from the root of the project.

#[macro_use]
extern crate rocket;

/// Responds to GET requests at the root URL ("/") with a static string.
#[get("/")]
fn index() -> &'static str {
    "Hello, world! 🚀"
}

/// Launches the Rocket web server and mounts the `index` route at the root URL ("/").
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
