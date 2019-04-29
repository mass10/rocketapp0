#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[allow(unused_imports)]
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::{serve::StaticFiles};
//use rocket::http::RawStr;
//use std::collections::HashMap;

//#[allow(unused_imports)]
//use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> rocket_contrib::templates::Template {
    let mut context: std::collections::HashMap<&str, String> = std::collections::HashMap::new();
    context.insert("title", String::from("none"));
    return rocket_contrib::templates::Template::render("index", context);
}

#[get("/hello")]
fn hello() -> &'static str {
    return "Hello, world!";
}

#[get("/hello2?<id>")]
fn hello2(id: String) -> String {
    return format!("id is {}", id.as_str());
}

//#[post("/apply", format = "application/json", data = "<user>")]
//fn apply(user: Json<User>) -> T {
//
//}

//#[post("/apply", data = "<input>")]
//fn apply(input: &RawStr) -> String {
//    format!("{}", input.as_str())
//}

fn main() {
    let server = rocket::ignite()
        .attach(rocket_contrib::templates::Template::fairing())
        .mount("/", StaticFiles::from("static/"))
        .mount("/", routes![index])
        .mount("/", routes![hello])
        .mount("/", routes![hello2]);
    server.launch();
}
