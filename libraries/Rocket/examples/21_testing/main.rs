// # Testing

// Rocket provides the tools to perform unit and integration tests.

// # Local Dispatching

// Rocket applications are tested by dispatching requests to a local instance of `Rocket`.The `local` module contains all of the structure necessary to do so.
// In particular, the `Client` structure that is used to create `LocalRequest` that can be dispatched against a given `Rocket` instance.

// 1. Construct a `Rocket` instance that represents the application.
// let rocket = rocket::build()

// 2. Construct a `Client` using the `Rocket` instance
// let client = Client::tracked(rocket).unwrap();

// 3. Construct requests using the `Client` instance
// let req = client.get("/");

// 4. Dispatch the request to retrieve the response
// let response = req.dispatch();

// # Validating Responses

// A `dispatch` of a `LocalRequest` returns a `LocalResponse` which can be inspected for validity. During testing, the response is usually validated against
// expected properties. These include things like response HTTP status, the inclusion of headers, and expected body data.

// `LocalResponse` type provides methods to ease this sort of validation:
// - `status`: returns the HTTP status in the response
// - `headers`: returns a map of all the headers in the response
// - `into_string`: reads the body data into a `String`
// - `into_bytes`: reads the body data into a `Vec<u8>`
// - `into_json`: deserializes the body data on-the-fly as JSON
// - `into_msgpack`: deserializes the body data on-the-fly as MessagePack
// These methods are typically used in combination with `assert_eq!` and `assert!` macros.

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }

    // The tests can be run with `cargo test`.
}

// # Asynchronous Testing

// Even though Rocket is an "asynchronous" web framework, in most situations, the blocking testing API is easier to use and should be preferred.
// However, if you need to test asynchronously, you can use the `asynchronous` API.
