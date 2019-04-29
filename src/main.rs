#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{serve::StaticFiles};

#[get("/")]
fn index() -> &'static str {
    ""
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {

    let server = rocket::ignite()
        .mount("/", routes![index])
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![hello]);

    server.launch();
}
