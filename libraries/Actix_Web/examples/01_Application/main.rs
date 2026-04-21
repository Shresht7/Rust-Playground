// # Application

// Actix-Web provides various primitives to build web servers and applications in Rust.
// It provides routing, middleware, pre-processing of requests, post-processing of responses, and more.

// All `actix-web` servers are built around the `App` instance. It is used for registering routes for resources and middleware.
// It also stores application state shared across all handlers within the same scope.

// An application's `scope` acts as a namespace for all routes, i.e. all routes for a specific application scope have the same url path prefix.
// The application prefix always contains a leading "/" slash. If a supplied prefix does not contain leading slash, it is automatically inserted.

use actix_web::{App, HttpServer, Responder, web};

async fn index() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            // prefixes all resources and routes attached to it...
            web::scope("/app")
                // ... so this handles requests for `GET /app/index.html`
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
