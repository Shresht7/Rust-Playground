#[macro_use]
extern crate rocket;

// ## Format
// A route can specify the data-format that it is willing to accept or respond with using the `format` route parameter.
// The value of the parameter is a string identifying an HTTP media type or a shorthand variant. For example, for JSON, the string
// `application/json` or the shorthand `json` can be used.

// When a route indicates a payload-supporting method (`PUT`, `POST`, `DELETE` and `PATCH`), the `format` route parameter instructs Rocket
// to check against the `Content-Type` header of the incoming request. Only requests where `Content-Type` header matches the `format` parameter will match to the route.

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    age: u8,
}

#[post("/user", format = "application/json", data = "<user>")]
fn new_user(user: User) -> String {
    format!("New user: {} ({})", user.name, user.age)
}

// The `format` parameter in the route abouve declares that only requests with `Content-Type: application/json` will match `new_user`.
// The `data` parameter is described below.
// When a route indicates a non-payload supporting method (`GET`, `HEAD` and `OPTIONS`), the `format` route parameter instructs Rocket
// to check against the `Accept` header of the incoming request. ONly request where the preferred media type in the `Accept` header matches the `format` parameter will match to the route.

#[get("/user", format = "json")]
fn get_user() -> String {
    // In a real application, you would fetch the user data here and serialize it to JSON.
    // For this example, we'll just return a dummy JSON string.
    r#"{"name": "Alice", "age": 30}"#.into()
}

// ## Body Data
// The body data processing, much like everything else in Rocket, is typed directly.
// To indicate that a handler expects body data, annotate it with `data = "<param>"`, where `param` is an argument in the handler.
// The argument's type must implement the `FromData` trait.

use rocket::Request;
use rocket::data::{self, Data, FromData, ToByteUnit};

#[rocket::async_trait]
impl<'r> FromData<'r> for User {
    type Error = String;

    async fn from_data(_req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        // In a real application, you would read the data and parse it into a User struct here.
        // For this example, we'll just return a dummy user.
        let _ = data.open(512.kibibytes()).into_string().await;
        data::Outcome::Success(User {
            name: "Bob".into(),
            age: 25,
        })
    }
}

#[post("/user", data = "<user>")]
fn create_user(user: User) -> String {
    format!("Created user: {} ({})", user.name, user.age)
}

// Any type that implements the `FromData` trait is also known as a data guard.

// ## JSON
// The `Json<T>` guard deserializes the body data as JSON. The only condition being that the generic type `T` implementes the `Deserialize` trait from `serde`.

use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    description: &'r str,
    completed: bool,
}

#[post("/task", format = "json", data = "<task>")]
fn create_task(task: Json<Task<'_>>) -> String {
    format!(
        "Created task: {} (completed: {})",
        task.description, task.completed
    )
}

// ## Temporary Files
// The `TempFile` data guard streams data directly into a temporary file which can be persisted.

use rocket::fs::TempFile;

#[post("/upload", data = "<_file>")]
async fn upload(_file: TempFile<'_>) -> String {
    // In a real application, you would persist the file here.
    // For this example, we'll just return a dummy response.
    // file.persist_to("permanent_location").await;
    "File uploaded successfully".into()
}

// ## Streaming
// Sometimes you want to just handle incoming data directly as a stream without buffering it into memory or a temporary file.
// Rocket makes it as simple as possible via the `Data` type:

use rocket::tokio;

#[post("/stream", data = "<data>")]
async fn stream(data: Data<'_>) -> std::io::Result<()> {
    // Stream at most 512KiB all of the body data to stdout
    data.open(512.kibibytes())
        .stream_to(tokio::io::stdout())
        .await?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![new_user, get_user, create_user, create_task, upload, stream],
    )
}
