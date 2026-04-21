use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};

// Request handlers use async functions that accept zero or more parameters.
// These parameters  can be extracted  from a request (see `FromRequest` trait), and the function returns a type that can be converted into an `HttpResponse` (see `Responder` trait).

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// Notice that some of these handlers have routing information attached directly using the built-in macros.
// These allow you to specify the method and ptah that the handler should respond to.
// You can also set up the routing manually as shown below.

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// Compile and run the server with `cargo run`.
