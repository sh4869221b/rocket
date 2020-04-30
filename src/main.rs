#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::status;

#[get("/")]
fn index() -> &'static str{
    "Hello World"
}

#[get("/list")]
fn list() -> status::Accepted<String>{
    status::Accepted(Some(format!("id: '{}'", "")))
}

fn main() {
    rocket::ignite().mount("/", routes![index, list]).launch();
}
