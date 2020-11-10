#![feature(proc_macro_hygiene, decl_macro)]

mod register;

use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Template {
    let mut context = HashMap::new();
    context.insert("x","test value");
    Template::render("index", &context)
}



fn main() {
    rocket::ignite()
        .mount("/", routes![index,crate::register::register])
        .attach(Template::fairing())
        .launch();
}
