// # Local State

// While managed state is global and available across the application, sometimes you want to maintain state that is local to a request.

// request-local state is local to a given request, carried along with the request, and dropped once the request is completed.
// Request-local state can be used whenever a `Request` is available, such as in a fairing, a request guard, or a responder.

// Request-local state is cached: if data of a given type has already been stored, it will be reused.
// This is especially useful for request guards that might be invoked multiple times during routing and processing of a single request, such as those
// that deal with authentication.

#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::request::{self, FromRequest, Request};

/// A global atomic counter for generating IDs
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// A type that represent's a request's ID
struct RequestID(pub usize);

/// Returns the current request's ID, assigning one only as necessary
#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r RequestID {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // The closure passed into `local_cache` will be executed at most once per request:
        // The first time the `RequestID` gaurd is used. If it is requested again, `local_cache` will return the same value.
        request::Outcome::Success(
            request.local_cache(|| RequestID(ID_COUNTER.fetch_add(1, Ordering::Relaxed))),
        )
    }
}

#[get("/")]
fn id(id: &RequestID) -> String {
    format!("This is request #{}", id.0)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![id])
}

// Note that, without request-local state, it would not be possible to:
// 1. Associate a piece of data, here an ID, directly with a request
// 2. Ensure that a value is generated at most once per request
