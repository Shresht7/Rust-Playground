// # Handlers

// A request handler is an async function that accepts zero or more parameters that can be extracted from a request (i.e. `impl FromRequest`)
// and returns a type that can be converted into a `HttpResponse` (i.e. `impl Responder`)

// Request handling happens in two stages
// First the handler object is called, returning any object that implements the `Responder` trait.
// Then the `respond_to` is called on the returned object, converting itself to an `HttpResponse` or `Error`

// By default, Actix Web provides `Responder` implementations for `&'static str`, `String`. etc.

use actix_web::{App, HttpServer, Result, get, web};

#[get("/str")]
async fn get_str() -> &'static str {
    "Hello world!"
}

#[get("/string")]
async fn get_string() -> String {
    "Hello world!".to_string()
}

// You can also change the signature to return `impl Responder` instead of a concrete type, which works well when complex types are involved.

#[get("/impl_responder")]
async fn get_impl_responder() -> impl actix_web::Responder {
    "Hello world!"
}

// ## Response with custom type

// To return a custom type directly from a handler function, the type needs to implement the `Responder` trait.

use actix_web::{ContentType, Error, HttpRequest, HttpResponse, Responder, body::BoxBody};

use serde::Serialize;

#[derive(Serialize)]
struct MyObject {
    name: &'static str,
}

// Responder
impl Responder for MyObject {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[get("/custom")]
async fn get_custom() -> MyObject {
    MyObject { name: "Actix Web" }
}

// ## Streaming Response Body

use futures::{future::ok, stream::once};

#[get("/stream")]
async fn stream() -> HttpResponse {
    let body = once(ok::<_, Error>(web::Bytes::from_static(b"test")));

    HttpResponse::Ok()
        .content_type("application/json")
        .streaming(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_str)
            .service(get_string)
            .service(get_impl_responder)
            .service(get_custom)
            .service(stream)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
