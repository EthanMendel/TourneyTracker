use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;
use serde::Serialize;

#[derive(FromForm, Serialize)]
pub struct RegisterPostData {
    name: String
}

#[get("/register")]
pub fn register() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("register", context)
}

#[post("/registerPost", data="<data>")]
pub fn register_post(data: Form<RegisterPostData>) -> Template {
    //send data to make new database entry
    if true {//success
        Template::render("RegisterSuccess", &data.0)
    } else {
        Template::render("RegisterFailure", &data.0)
    }
}
