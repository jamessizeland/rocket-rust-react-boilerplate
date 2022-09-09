use std::env;

use rocket::{
    fs::{relative, FileServer},
    get, routes,
};

#[get("/hello")]
fn hello() -> &'static str {
    "Hello World"
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("../dist")))
        .mount("/api", routes![hello])
}
