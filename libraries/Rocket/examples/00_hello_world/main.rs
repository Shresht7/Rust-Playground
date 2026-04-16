//! A simple "Hello, world!" example using Rocket.
//!
//! Creates a web server that listens on `localhost:8000` and responds with "Hello, world! 🚀" when the root URL is accessed.
//!
//! Run this example using `cargo run --example 00_hello_world` from the root of the project.

// Explicitly importing with `#[macro_use]` allows us to use Rocket's macros globally anywhere in the application
// without needing to prefix them with `rocket::`. This is necessary for the `#[get]` and `#[launch]` macros used in this example.
#[macro_use]
extern crate rocket;

// Alternatively, we could have used regular imports for `rocket::get` and `rocket::launch`
// or directly use `#[rocket::get]` and `#[rocket::launch]` in the code without the `#[macro_use]` declaration.

/// Responds to GET requests at the root URL ("/") with a static string.
#[get("/")]
fn index() -> &'static str {
    "Hello, world! 🚀"
}

/// Launches the Rocket web server and mounts the `index` route at the root URL ("/").
/// Rocket begins serving requests after being _launched_, which starts a multi-threaded asynchronous server
/// and dispatches requests to the appropriate routes as needed.
/// Tip: #[launch] infers the return type of the function, so we can use `_` instead of explicitly writing `Rocket<Build>`.
/// See https://rocket.rs/guide/v0.5/overview/#launching for more details.
#[launch]
fn rocket() -> _ {
    // Before Rocket can dispatch requests to a route, the route needs to be _mounted_.
    // Mounting a route means associating it with a base path. In this case, we mount the `index` route at the root path ("/").
    // See https://rocket.rs/guide/v0.5/overview/#mounting for more details.
    rocket::build().mount("/", routes![index])
}

// Alternatively, we could use the `#[rocket::main]` route attribute instead of `#[launch]
// #[rocket::main]
// async fn main() -> Result<(), rocket::Error> {
//     let _rocket = rocket::build()
//         .mount("/hello", routes![world])
//         .launch()
//         .await?;
//     Ok(())
// }
