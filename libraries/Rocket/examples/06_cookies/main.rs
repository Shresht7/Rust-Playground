// # Cookies!

#[macro_use]
extern crate rocket;

// `CookieJar` is an important built-in request guard. It allows you to get, set and remove cookies.
// Because `&CookieJar` is a request guard, an argument of its type can simply be added to any route handler.
// This way you can access the incoming request's cookies. Cookies can also be set and removed using the `CookieJar` request guard.

use rocket::http::CookieJar;

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Option<String> {
    cookies
        .get("message")
        .map(|crumb| format!("Message: {}", crumb.value()))
}

// ## Private Cookies
// Cookies added by the `CookieJar::add()` are set in the clear, i.e. the value set is visible to the client.
// For sensitive information, Rocket provides private cookies. Private cookies are similar to regular cookies, except that they are
// encrypted using authenticated encryption, a form of encryption which simultaneously provides confidentiality, integrity and authenticity.
// Thus, private cookies cannot be inspected, tampered with, or manufactured by clients.

// Support for private cookies must be manually enabled via the `secrets` crate feature:
// ```toml
// [dependencies]
// rocket = { version = "0.5.1", features = ["secrets"] }
// ```

// The API for retrieving, adding, and removing private cookies is identical except that most methods are suffixed wtih `_private`.
// i.e. `get_private`, `add_private`, `remove_private`.

use rocket::response::{Flash, Redirect};

/// Retrieve the user's ID, if any
#[get("/user_id")]
fn user_id(cookies: &CookieJar<'_>) -> Option<String> {
    cookies
        .get_private("user_id")
        .map(|crumb| format!("User ID: {}", crumb.value()))
}

/// Remove the `user_id` cookie
#[post("/logout")]
fn logout(cookies: &CookieJar<'_>) -> Flash<Redirect> {
    cookies.remove_private("user_id");
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, user_id, logout])
}

// ## Secret Key
// To encrypt private cookies, Rocket uses the 256-bit key specified in the `secret_key` configuration parameter.
// When compiled in debug mode, a fresh key is generated automatically.
// In release mode, however, Rocket requires you to set a secret key if the `secrets` feature is enabled, else it will panic at launch.
// The value of the parameter may either be a 256-bit based base64-encoded string or a hex string or a 32-byte slice.
// Generating a string suitable for use as `secret_key` is usually done through tools like `openssl` (e.g. `openssl rand -base64 32`).
