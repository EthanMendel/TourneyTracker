#![feature(proc_macro_hygiene, decl_macro)]

mod register;
mod display;
mod schema;
mod db;
mod error;

#[macro_use]
extern crate diesel;

use rocket_contrib::templates::Template;
use std::collections::HashMap;
use dotenv::dotenv;
use diesel::prelude::*;
use crate::schema::tournaments::dsl::*;
use crate::schema::teams::dsl::*;
use crate::db::{ Tournament, Team };

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;


#[database("tournaments")]
pub struct TournamentDbConn(diesel::MysqlConnection);

#[get("/")]
fn index(conn: TournamentDbConn) -> Template {
    let mut hash = HashMap::new();
    let tourneys_db = tournaments.load::<Tournament>(&conn.0).unwrap();
    let teams_db = teams.load::<Team>(&conn.0).unwrap();
    hash.insert("tournaments",serde_json::json!(tourneys_db));
    hash.insert("teams",serde_json::json!(teams_db));
    Template::render("index", &hash)
}

fn main() {
    dotenv().ok();

    rocket::ignite()
        .attach(TournamentDbConn::fairing())
        .mount("/", routes![
            index,
            register::register_tournament,
            register::register_tournament_post,
            register::register_team,
            register::register_team_post,
            display::show_tournament])
        .attach(Template::fairing())
        .launch();
}
