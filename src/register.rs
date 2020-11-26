use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;
use crate::db::{ InsertableTournament, InsertableTeam };
use crate::schema::tournaments::dsl::*;
use crate::schema::teams::dsl::*;
use diesel::prelude::*;
use rocket::http::RawStr;


#[get("/registerTournament")]
pub fn register_tournament() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("registerTournament", context)
}

#[post("/registerTournament", data="<data>")]
pub fn register_tournament_post(data: Form<InsertableTournament>, conn: crate::TournamentDbConn) -> Result<Template, Template> {
    //send data to make new database entry
    println!("{:?}", &data.0);
    if diesel::insert_into(tournaments).values(&data.0).execute(&conn.0).is_ok() {//success
        Ok(Template::render("RegisterTournamentSuccess", &data.0))
    } else {
        Err(Template::render("RegisterTournamentFailure", &data.0))
    }
}

#[get("/registerTeam?<tournamentId>")]
pub fn register_team(tournamentId: &RawStr) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("registerTeam", context)
}

#[post("/registerTeam?<tournamentId>", data="<data>")]
pub fn register_team_post(data: Form<InsertableTeam>, tournamentId: &RawStr, conn: crate::TournamentDbConn) -> Result<Template, Template> {
    //send data to make new database entry
    println!("{:?}", &data.0);
    if diesel::insert_into(teams).values(&data.0).execute(&conn.0).is_ok() {//success
        Ok(Template::render("RegisterTeamSuccess", &data.0))
    } else {
        Err(Template::render("RegisterTeamFailure", &data.0))
    }
}
