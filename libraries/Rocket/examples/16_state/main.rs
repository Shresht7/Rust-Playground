// # State

// Many web applications have a need to maintain state. This can be as simple as maintaining a counter for the number of visits
// or as complex as needing to access jobs, queues multiple database connections.

// To instruct Rocket to manage state for your application, call the `manage` method on an instance of `Rocket`.

// For example, to ask Rocket to manage a `HitCount` structure with an internal `AtomicUsize` with an initial value of `0`

#[macro_use]
extern crate rocket;

use std::sync::atomic::{AtomicUsize, Ordering};

struct HitCount {
    count: AtomicUsize,
}

// State that is being managed by Rocket can be retrieved via the `&State` type: a request guard for managed state.

#[get("/")]
fn index(hit_count: &rocket::State<HitCount>) -> String {
    let current_count = hit_count.count.load(Ordering::Relaxed);
    format!("Visit count: {}", current_count)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(HitCount {
            count: AtomicUsize::new(0),
        })
        .mount("/", routes![index])
}
