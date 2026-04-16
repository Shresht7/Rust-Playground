use rocket::fs::NamedFile;
use std::path::{Path, PathBuf};

// ## Static File Server
// Rocket's `NamedFile` type allows you to serve static files from the filesystem.
// In this example, we define a route that captures a file path from the URL and serves the corresponding file from the `static/` directory.
// The `<file..>` syntax indicates that `file` is a dynamic path parameter that captures multiple segments of the URL as a `PathBuf`.
// When a request is made to a URL like `/files/images/logo.png`, the value "images/logo.png" will be captured
// and passed to the `files` function as the `file` argument.

#[rocket::get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}

// Everything else is the same as before. We just need to mount the `files` route in our Rocket instance to serve static files from the `static/` directory.

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::routes![files])
}
