// # Error Catchers

// Application processing is fallible. Errors can arise from the following soruces:
// - A failing guard
// - A failing responder
// - A routing failure

// If any of these errors occur, Rocket returns an error to the client. To generate the error, Rocket invokes the catcher
// corresponding to the error's status code and scope. Catchers are simialr to routes except in that:
// 1. Catchers are only invoked on error conditions
// 2. Catchers are declared with the `catch` attribute
// 3. Catchers are registered with `register()` instead of `mount()`.
// 4. Any modifications to cookies are cleared before a catcher is invoked.
// 5. Error catchers should not fail to produce a response.
// 6. Error catchers cannot invoke guards.
// 7. Catchers are scoped to a path prefix.

// To declare a catcher, for a given status code, use the `catch` attribute, which takes a single integer corresponding to the HTTP status code to catch.
// For instance, to declare a catcher for `404 Not Found` you'd write:

#[macro_use]
extern crate rocket;

use rocket::Request;

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

// Catchers may take zero, one, or two arguments. If the catcher takes one argument, it must be of type `&Request`. If it takes two, they must be of type
// `Status` and `&Request`, in that order. As with routes, the return type must implement `Responder`.

// ## Scoping

// The first argument to `register()` is a path to scope the catcher under called the catcher's base. A catcher's base determines which requests it
// will handle errors for. Specifically, a catcher's base must be a prefix of the erroring request for it to be invoked.
// When multiple catchers can be invoked, the catcher with the longest base takes precedence.

#[catch(404)]
fn food_not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path. (food)", req.uri())
}

// ## Default Catchers

// A default catcher is a catcher that handles all status codes. They are invoked as a fallback if no status-specific catcher is registered for a given error.
//Declaring a default catcher is done with `#[catch(default)]` and must similarly be registered with `register()`.

use rocket::http::Status;

#[catch(default)]
fn default_catcher(status: Status, req: &Request) -> String {
    format!("Sorry, '{}' resulted in an error: {}.", req.uri(), status)
}

// Also, as with routes, Rocket needs to know about a catcher before it is used to handle errors.
// The process, known as "registering" a catcher, is similar to mounting a route: call the `register()` method with a list of catchers via the `catchers!` macro.

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, default_catcher])
        .register("/food", catchers![food_not_found])
}

// ## Built-In Catchers
// Rocket provies a built-in default catcher.
// It produces HTML or JSON, depending on the value of the `Accept` header. As such, custom catchers only need to be registerd for custom error handling.
