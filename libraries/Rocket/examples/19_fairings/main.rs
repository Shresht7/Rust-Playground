// # Fairings

// Farings are Rocket's approach to structured middleware.
// With fairings, your application can hook into the request lifecycle to record or rewrite information about incoming requests and outgoing responses.

// Any type that implements the `Fairing` trait is a fairing! Faring hooks into Rocket's request lifecycle, receiving callbacks for events
// such as incoming requests and outgoing responses. The fairing can do whatever it wants with the information. This includes rewriting requests or responses,
// recording information about the event, or doing nothing at all.

// Rocket's fairings are a lot like middleawre from other frameworks, but they bear a few key distinctions:
// - Fairings cannot terminate or respond to an incoming request directly
// - Farings cannot inject arbitrary, non-request data into a request
// - Farings can prevent an application from launching
// - Farings can inspect and modify the application's configuration

// Note: If you're familiar with middlewares from other frameworks, you may instinctively reach for fairings.
// Before doing so, remember that Rocket provides a rich set of mechanisms such as request guards and data guards that can be used to solve problems in a clean, composable, and robust manner.

// ## Attaching

// Fairings are registerd with Rocket via the `attach` method on a `Rocket` instance.Only when faring is attached, will its callbacks fire.
// ```rs
// #[launch]
// fn rocket() -> _ {
//    rocket::build()
//        .attach(req_fairing)
//       .attach(res_fairing)
// }
// ```

// Fairings are executed in the order in which they are attached: the first attached fairing has its callbacks executed before all others.
// A fairing can be attached any number of times. Except for singleton fairings, all attached instances polled at runtime for those.

// ## Callbacks

// There are five events for which Rocket issues fairing callbacks:
// - `on_ignite`:   An ignite callback is called during ignition. An ignite callback can arbitrarily modify the Rocket instance being built.
//                  These are commonly used to parse and validate configuration values, aborting bad configuration etc.
// - `on_liftoff`:  A liftoff callback is called immediately after a Rocket application has launched. They can inspect the `Rocket` instance being launched.
// - `on_request`:  A request callback is called just after a request is received. A request callback can modify the request at will and peek into the incoming data.
// - `on_response`: A response callback is called when a response is ready to be sent to the client. A response callback can modify part of or the entire response.
// - `on_shutdown`: A shutdown callback is called when the shutdown is triggered. At this point, graceful shutdown has commenced but not completed.

// ## Implementation

// A type implementing the `Fairing` trait is required to be `Send + Sync + 'static`.
// This means that the faring must be sendable across thread boundaries (`Send`)
// and should be thread-safe (`Sync`)
// and have only static references, if any (`'static`)
// Note that these bounds do not prohibit a `Fairing` from holding state: the state need simply be thread-safe and statically available or heap allocated.

// Example:
// We want to record the number of `GET` and `POST` requests that our application has received.
// While we could do this with request guards and managed state, it would require us to annotate every `GET` and `POST` request with custom-types, polluting the handler signatures.
// Instead, we can create a fairing that acts globally.

#[macro_use]
extern crate rocket;

use std::io::Cursor;
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Method, Status};
use rocket::{Data, Request, Response};

struct Counter {
    get: AtomicUsize,
    post: AtomicUsize,
}

#[rocket::async_trait]
impl Fairing for Counter {
    // This is a request and response fairing named "GET/POST Counter"
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response,
        }
    }

    // Increment the counter for `GET` and `POST` requests
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        match request.method() {
            Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            _ => return,
        };
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Don't change a successful user's response, ever
        if response.status() != Status::NotFound {
            return;
        }

        // Rewrite the response to return the current counts
        if request.method() == Method::Get && request.uri().path() == "/counts" {
            let get_count = self.get.load(Ordering::Relaxed);
            let post_count = self.post.load(Ordering::Relaxed);
            let body = format!("GET: {}, POST: {}", get_count, post_count);
            response.set_status(Status::Ok);
            response.set_header(ContentType::Plain);
            response.set_sized_body(body.len(), Cursor::new(body));
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(Counter {
        get: AtomicUsize::new(0),
        post: AtomicUsize::new(0),
    })
}
