#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

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
        .mount("/", routes![hello]);

    server.launch();
}
