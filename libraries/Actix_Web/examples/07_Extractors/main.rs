// # Extractors

// Actix Web provides a facility for type-safe request information access called "extractors" (i.e. `impl FromRequest`).
// There are a lot of built-in extractor implementations.

// An extractors can be accessed as an argument to a handler function. Actix Web supports up to 12 extractors per handler function.
// In most cases, the argument position does not matter. However, if the extractor takes the body (i.e. reads any bytes from request body stream),
// then only the first extractor will succeed.If you need fallback behaviour such as "read body as JSON or just give me bytes if that fails", then use the `Either` extractor. (`Either<Json<T>, Bytes>`)

// ## Path

// Path provides information that is extracted from request's path. Parts of the path that are extractable are called "dynamic segments"
// and are marked with curly braces. You can deserialize any variable segments from the path.

// For instance, for resource that registered for the `/users/{user_id}/{friend}` path, two segments could be deserialized, `user_id` and `friend`.
// These segments could be extracted as a tuple in the order they are declared (e.g. `Path<(u32, String)>`)

use actix_web::{App, HttpServer, Result, get, web};

/// extract path info from "/users/{user_id}/{friend}"
/// `user_id`: deserialize to u32
/// `friend`: deserialize to String
#[get("/users/{user_id}/{friend}")]
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id: {}", friend, user_id))
}

// It is also possible to extract path information to a type that implements the `Deserialize` trait from `serde` by matching dynamic segment names with field names.

use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    user_id: u32,
    friend: String,
}

/// extract path info using serde
#[get("/users2/{user_id}/{friend}")]
async fn index2(info: web::Path<Info>) -> Result<String> {
    Ok(format!(
        "Welcome {}, user_id: {}",
        info.friend, info.user_id
    ))
}

// ## Query

// The `Query<T>` type provides extraction functionality for the request's query parameters.
#[derive(Deserialize)]
struct QueryInfo {
    username: String,
}

// This handler gets called if the query deserializes into `QueryInfo` successfully. Otherwise, a 400 Bad Request error response is returned.
#[get("/user3")]
async fn index3(query: web::Query<QueryInfo>) -> Result<String> {
    Ok(format!("Welcome {}", query.username))
}

// ## JSON

// The `Json<T>` allows deserialization of a request body into a struct.To extract typed information for a request's body, the type `T` must implement `serde::Deserialize`

#[derive(Deserialize)]
pub struct JsonInfo {
    username: String,
    age: u8,
}

/// deserialize `JsonInfo` from request body
#[post("/json")]
async fn submit(json: web::Json<JsonInfo>) -> Result<String> {
    Ok(format!("Welcome {}, age: {}", json.username, json.age))
}

// ## URL Encoded Forms

// A URL-Encoded Form body can be expanded to a struct, much like `Json<T>`. This type must implement `serde::Deserialize` as well.

#[derive(Deserialize)]
pub struct URLFormInfo {
    username: String,
    age: u8,
}

#[post("/url_form")]
async fn submit_url_form(form: web::Form<URLFormInfo>) -> Result<String> {
    Ok(format!("Welcome {}, age: {}", form.username, form.age))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index2)
            .service(index3)
            .service(submit)
            .service(submit_url_form)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Actix Web provides many other extractors like:
// - `Data`: for accessing pieces of application state
// - `HttpRequest`: `HttpRequest` is itself an extractor, in case you need other parts of the request
// - `String`: You can convert a request's payload into a `String`
// - `Bytes`: You can convert a request's payload into `Bytes`
// - `Payload`: Low-level payload extractor primarily for building other extractors.
