// # Keep Alive

// Actix Web keeps connections open to wait for subsequent requests.

use actix_web::{HttpServer, http::KeepAlive};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set keep-alive to 75 seconds
    let _one = HttpServer::new(app).keep_alive(Duration::from_secs(75));

    // Use OS's default keep-alive timeout (usually quite long)
    let _two = HttpServer::new(app).keep_alive(KeepAlive::Os);

    // Disable keep-alive
    let _three = HttpServer::new(app).keep_alive(KeepAlive::Disabled);

    Ok(())
}

// # Graceful Shutdown

// `HttpServer` supports graceful shutdown. After receiving a stop signal, workers have a specific amount of time to finish serving requests.
// Any worker still alive after the timeout are force-dropped. By default, the shutdown timeout is set to 30 seconds.
// You can change this parameter with the `HttpServer::shutdown_timeout` method.
