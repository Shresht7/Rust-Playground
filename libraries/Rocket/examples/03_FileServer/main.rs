use rocket::fs::FileServer;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", FileServer::from("static"))
}
