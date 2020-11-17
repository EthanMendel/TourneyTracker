#![feature(proc_macro_hygiene, decl_macro)]

mod register;
mod schema;
mod db;

#[macro_use]
extern crate diesel;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use dotenv::dotenv;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("x","test value");
    Template::render("index", &context)
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .mount("/", routes![index,crate::register::register])
        .attach(Template::fairing())
        .launch();
}
