use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;
use serde::Serialize;
use crate::db::InsertableTournament;
use crate::schema::tournaments::dsl::*;
use diesel::prelude::*;


#[get("/register")]
pub fn register() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("register", context)
}

#[post("/register", data="<data>")]
pub fn register_post(data: Form<InsertableTournament>, conn: crate::TournamentDbConn) -> Result<Template, Template> {
    //send data to make new database entry
    println!("{:?}", &data.0);
    if diesel::insert_into(tournaments).values(&data.0).execute(&conn.0).is_ok() {//success
        Ok(Template::render("RegisterSuccess", &data.0))
    } else {
        Err(Template::render("RegisterFailure", &data.0))
    }
}
