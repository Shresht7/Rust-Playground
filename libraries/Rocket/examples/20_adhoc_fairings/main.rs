// # Adhoc Fairings

// For simpler cases, implementing the `Fairing` trait can be cumbersome.
// Rocket provides the `Adhoc` type, which creates a faring from a simple function or closure.

#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket::http::Method;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_liftoff("Liftoff Printer", |_| {
            Box::pin(async move { println!("...aaannnndddd we have liftoff!!!") })
        }))
        .attach(AdHoc::on_request("Put Rewriter", |req, _| {
            Box::pin(async move {
                req.set_method(Method::Put);
            })
        }))
        .attach(AdHoc::on_shutdown("Shutdown Printer", |_| {
            Box::pin(async move {
                println!("...shutdown has commenced!");
            })
        }))
}
