// ## TLS / HTTPS

// Actix Web supports two TLS implementations out-of-the-box: `rustls` and `openssl`.

// The `rustls` crate is for `rustls` integration and `openssl` crate for `openssl` integration.

use actix_web::{App, HttpRequest, HttpServer, Responder, get};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello, secure world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load TLS key
    // to create a self-signed temporary certificate for testing:
    // `openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
