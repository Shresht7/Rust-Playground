// # Shared Mutable State

// `HttpServer` accepts an application factory rather than an application instance. An `HttpServer` constructs an application instance
// for each thread it runs on. Therefore, application data must be constructed multiple times, once per thread.
// If you wish to share some data between different threads, a shareable object should be used, e.g. `Send` + `Sync`

// Internally, `web::Data` uses `Arc`. So in order to avoid creating two `Arc`s, we should create our Data before registering it using `App::app_data()`

use actix_web::{App, HttpServer, web};

use std::sync::Mutex;

struct AppState {
    counter: Mutex<i32>, // Mutex is necessary to safely mutate the counter across multiple threads
}

async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap(); // get counter's mutex guard
    *counter += 1; // access counter inside MutexGuard
    format!("Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new() closure
    let counter = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone()) // register the created data
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// - State initialized inside the closure passed to `HttpServer::new()` is local to the worker thread and may become de-synced if modified
// - To achieve globally shared state, it must be created outside of the closure passed to `HttpServer::new` and moved/cloned into the closure.
