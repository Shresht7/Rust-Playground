// # Responses

// Any type that implements the `Responder` trait can be returned from a route.
// Types that implement `Responder` know how to generate a `Response` from their values. A `Response` includes anHTTP status, header, and body.
// The `body` may either be fixed-size or streaming. The given `Responder` implementation decides which to use.
// Responders may dynamically adjust their response according to the incoming `Request` they are responding to.

#[macro_use]
extern crate rocket;

// ## Wrapping

// It is typical for Responders to wrap other responders. A wrapping responder modifies the response before returning the same response.

use rocket::response::status;

#[post("/<id>")]
fn new(id: usize) -> status::Accepted<String> {
    status::Accepted(format!("Created new resource with id: {id}"))
}

// Similarly, the types in the `content` module can be used to override the `Content-Type` of a response.

use rocket::http::Status;
use rocket::response::content;

#[get("/json/1")]
fn json() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}

// The builtin `(Status, R)` and `(ContentType, R)` responders, where `R: Responder`, also override the `Statusnd `Content-Type` of responses:

use rocket::http::ContentType;

#[get("/json/2")]
fn json_part_two() -> (Status, (ContentType, &'static str)) {
    (
        Status::ImATeapot,
        (ContentType::JSON, "{ \"hi\": \"world\" }"),
    )
}

// ## Errors

// Responders may fail instead of generating a response by returning `Err` with a status code. When this happens,
// Rocket forwards the request to the "error catcher" for that status code.

// If an error catcher has been registered for the given status code, Rocket will invoke it. The catcher creates and returns a response to the client.
// If no error catcher has been registered and the error status code is one of the standard HTTP status code, a default error catcher will be used.
// Default error catchers return an HTMP apge with the status code and description. If there are no catchers for a custom status code, Rocket
// uses the 500 error catcher to return a response.

// ## Custom Responders

// The `Responder` trait can be implemented for custom types to allow them to be returned from routes.
// If your custom responder wraps an existing responder, headers, set of custom status or content-type, `Responder` can be automatically derived.

// ```rs
// use rocket::http::{Header, ContentType, Status};
//
// #[derive(Responder)]
// #[response(status = 500, content_type = "json")]
// struct MyResponder {
//     inner: OtherResponder,
//     // Override the Content-Type declared above
//     header: ContentType,
//     more: Header<'static>,
//     #[response(ignore)]
//     unrelated: MyType,
// }
// ```

// To set an HTTP status dynamically, leverage the `(Status, R: Responder)` responder.

// You can also derive `Responder` for `enum`s, allowing dynamic selection of a responder:

use rocket::fs::NamedFile;

#[allow(dead_code)]
#[derive(Responder)]
enum Error {
    #[response(status = 500, content_type = "json")]
    A(String),

    #[response(status = 404)]
    B(NamedFile, ContentType),

    C {
        inner: (Status, Option<String>),
        header: ContentType,
    },
}

// ## Implementations

// Rocket implements `Responder` for many types in the Rust's standard library including `String`, `&str`, `File`, `Option`, and `Result`.

// ### Strings

// You can directly return `String` and `&str` because of their `Responder` implementations.

#[get("/string")]
fn handler() -> &'static str {
    "Hello there! I'm a string!"
}

// ### Option

// `Option` is a wrapping responder: an `Option<T>` can only be returned when `T` implements `Responder`.
// If the `Option` is `Some`, the wrapped responder is used to respond to the client. Otherwise, an error of `404 - Not Found` is returned to the client.

// This implementation makes `Option` a convenient type to return when it is not known until process-time whether content exists.
// For example, because of `Option`, we can implement a file server that returns a `200` when a file is found and a `404` when a file is not found in 4 idomatic lines.

use std::path::{Path, PathBuf};

#[get("/file/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// ### Result

// `Result` is another wrapping responder: a `Result<T, E>` can only be returned when `T` implements `Responder` and `E` implmenents `Responder`.

// The wrapped `Responder` in `Ok` and `Err`. whichever it might be, is used to respond to the client.
// This means that the responder can be dynamically chosen at run-time, and two different kinds of responses can be used depending on the circumstances.

#[get("/file-results/<file..>")]
async fn files_result(file: PathBuf) -> Result<NamedFile, status::NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path)
        .await
        .map_err(|e| status::NotFound(e.to_string()))
}

// ## Rocket Responders

// Some of Rocket's best features are implemented through responders.
// - `NamedFile`: Streams a file to client; automatically set the `Content-Type` based on the file's extension.
// - `Redirect`: Redirects the client to a different URI.
// - `content`: Contains types that override the `Content-Type` of a response.
// - `status`: Contains types that override the HTTP status code of a response.
// - `Flash`: Sets a "flash" cookie that is removed when accessed.
// - `Json`: Automatically serializes values into `MessagePack`
// - `Template`: Renders a dynamic template using handlebars or Tera.

// ### Async Streams

// The `stream` responders allow serving potentially infinite async `Stream`s. A stream can be created from any async `Stream` or `AsyncRead` type, or
// via a generator syntax using the `stream!` macro and its typed equivalents. Streams are the building blocks for unidirectional real-time communication.

use std::io;
use std::net::SocketAddr;

use rocket::response::stream::ReaderStream;
use rocket::tokio::net::TcpStream;

#[get("/stream")]
async fn stream() -> io::Result<ReaderStream![TcpStream]> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 9999));
    let stream = TcpStream::connect(addr).await?;
    Ok(ReaderStream::one(stream))
}

// Straems can also be created using generator syntax. The following example returns an infinite `TextStream` that produces one "hello" every second.

use rocket::response::stream::TextStream;
use rocket::tokio::time::{Duration, interval};

/// Produces an infinite stream of "hello"s one per second
#[get("/infinite-hellos")]
fn hello() -> TextStream![&'static str] {
    TextStream! {
        let mut interval = interval(Duration::from_secs(1));
        loop {
            yield "hello";
            interval.tick().await;
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            new,
            json,
            json_part_two,
            handler,
            files,
            files_result,
            stream,
            hello
        ],
    )
}
