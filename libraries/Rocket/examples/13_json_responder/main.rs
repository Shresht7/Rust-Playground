// # JSON

// The `Json` Responder allows you to easily respond with well-formed JSON data: simply return a value of type `Json<T>` where T is the type
// of the structure to serialize into JSON. The type `T` must implement the `Serialize` trait from `serde` which will be automatically derived.

#[macro_use]
extern crate rocket;

use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    description: String,
    completed: bool,
}

#[get("/todo")]
fn todo() -> Json<Task> {
    Json(Task {
        description: "Learn Rocket".into(),
        completed: false,
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![todo])
}
