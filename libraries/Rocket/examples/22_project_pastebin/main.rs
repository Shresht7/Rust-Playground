#[macro_use]
extern crate rocket;

use rocket::tokio::fs::File;

mod paste;
use crate::paste::PasteId;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE
        
        POST /

            accepts raw data in the body of the request and responds with a URL of a page containing the body's contents

        GET /<id>

            retrieves the content for the paste with the id `<id>`
    "
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve])
}
