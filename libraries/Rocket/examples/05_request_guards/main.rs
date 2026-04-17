// A Request Guard is one of Rocket's most powerful instruments.
// A Request Guard protects a handler from being called erroneously based on information contained in an incoming request.
// More specifically, a request guard is a type that represents an arbitrary validation policy, implemented through a `FromRequest` trait.
// Every type that implements the `FromRequest` trait is a Request Guard.

#[macro_use]
extern crate rocket;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    response::Redirect,
    Request,
};

// Request Guards appear as inputs to route handlers. An arbitrary number of request guards can be used in a single handler.
// Rocket automatically invokes the `FromRequest` implementation for each guard before calling the handler.
// Rocket only dispatches requests to a handler when all its request guards pass.

struct SomeTypeGuard;

#[async_trait]
impl<'r> FromRequest<'r> for SomeTypeGuard {
    type Error = ();

    async fn from_request(_request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Implement your guard logic here. For example, you might check for a specific header or query parameter.
        // If the guard condition is met, return `Outcome::Success(Self)`.
        // If the guard condition is not met, return `Outcome::Failure((Status::Forbidden, ()))`.
        // If the guard cannot determine the outcome (e.g., due to missing information), return `Outcome::Forward(())`.
        Outcome::Success(SomeTypeGuard)
    }
}

#[get("/guarded/<param>")]
fn guarded_route_handler(param: isize, g: SomeTypeGuard) -> String {
    format!("Guarded route with param: {}", param)
}

// Request Guards always fire in a left-to-right declaration order and short-circuit on failure.
// If any guard fails, the request is not dispatched to the handler and Rocket continues searching for another matching route.

// You can implement `FromRequest` for your own types. For instance, to protect a `/sensitive` route from running unless a `ApiKey` is present in the request headers.

struct ApiKey<'r>(&'r str);

#[async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("x-api-key") {
            None => Outcome::Error((Status::BadRequest, "API key missing".to_string())),
            Some(key) if key == "expected_api_key" => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Error((Status::Unauthorized, "Invalid API key".to_string())),
        }
    }
}

#[get("/sensitive")]
fn sensitive_route_handler(api_key: ApiKey) -> String {
    format!("Sensitive route accessed with API key: {}", api_key.0)
}

// ## Guard Transparency
// When a request guard type can only be created through its `FromRequest` implementation, and the type is not `Copy`, the existence
// of a request guard value provides a type-level proof that the current request has been validated against the arbitrary policy represented by the guard at compile-time. 


// ## Forwarding Guards

#[get("/login")]
fn login() -> &'static str {
    "Please log in to access this resource."
}

struct User {
    id: usize,
    name: String,
}

#[async_trait]
impl <'r> FromRequest<'r> for User {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("x-user") {
            None => Outcome::Forward(Status::Unauthorized),
            Some(name) => Outcome::Success(User { id: 1, name: name.to_string() }),
        }
    }
}

struct AdminUser {
    id: usize,
    name: String,
}

#[async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("x-admin-user") {
            None => Outcome::Forward(Status::Unauthorized),
            Some(name) => Outcome::Success(AdminUser { id: 1, name: name.to_string() }),
        }
    }
}

#[get("/admin")]
fn admin(admin: AdminUser) -> String {
    format!("Welcome, admin user: {}", admin.name)
}

// ## Fallible Guards
// A failing or forwading guard can be "caught" in handler, preventing it from failing or forwarding, via the `Option<T>` and `Result<T, E>` guards.
// When a guard `T` fails or forwards, `Option<T>` will be None. If a guard `T` fails with an error `E`, `Result<T, E>` will be Err(E).

#[get("/admin", rank = 2)]
fn admin_panel_user(user: Option<User>) -> Result<&'static str, Redirect> {
    match user {
        // If the user guard forwards or fails, the `Option<User>` will be None. In this case, we redirect the user to the login page.
        None => Err(Redirect::to(uri!(login))),
        // If the user guard succeeds, we welcome the user to the admin panel, even though they do not have access to it.
        Some(user) => Ok(format!("Welcome, regular user: {}. You do not have access to the admin panel.", user.name).as_str()),
    }
}

#[get("/admin", rank = 3)]
fn admin_fallback() -> Redirect {
    Redirect::to(uri!(login))    
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![guarded_route_handler, sensitive_route_handler, login, admin, admin_panel_user, admin_fallback])
}
