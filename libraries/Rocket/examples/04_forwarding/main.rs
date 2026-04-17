// When Rocket fails to match a route, it forwards the request to the next matching route, if any.
// This continues until a route succeeds or there are no more matching routes to try.
// When there are no remaining routes, the error-catcher associated with the statis set by the last forwarding guard is called.s

// Routes are attempted in increasing rank order. Every route has an associated rank. If one is not specified, Rocket chooses a default rank
// to avoid collisions. A rank can also be manually set with the `rank` attribute.

#[macro_use]
extern crate rocket;

#[get("/user/<id>")]
fn user(id: usize) -> String {
    format!("User #{}", id)
}

#[get("/user/<id>", rank = 2)]
fn user_forward(id: isize) -> String {
    format!("Forwarding user #{} to the next route", id)
}

#[get("/user/<id>", rank = 3)]
fn user_final(id: &str) -> String {
    format!("Final user #{}", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![user, user_forward, user_final])
}
