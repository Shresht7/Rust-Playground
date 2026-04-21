// # State

// Application state is shared with all routes and resources within the same scope.
// State can be accessed with the `web::Data<T>` extractor, where T is the type of the state.
// State is also accessible for middlewares.

use actix_web::{App, HttpServer, get, web};

/// This structure represents the application state
struct AppState {
    app_name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    format!("Hello from {app_name}!")
}

// Next, pass in the state when initializing the App and start the application

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: "Actix Web".to_string(),
            }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Any number of state types could be registered within the application.
