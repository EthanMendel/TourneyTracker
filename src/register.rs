use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;
use crate::db::{ InsertableTournament, InsertableTeam };
use crate::schema::tournaments::dsl::*;
use crate::schema::tournaments;
use crate::schema::teams::dsl::*;
use diesel::prelude::*;
use crate::db::Tournament;


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

#[get("/registerTeam?<tourney_id>")]
pub fn register_team(tourney_id: i32, conn: crate::TournamentDbConn) -> Template {
    let mut context = HashMap::new();
    let tourney = tournaments.filter(tournaments::dsl::id.eq(tourney_id)).first::<Tournament>(&conn.0).unwrap();
    context.insert("tournament",serde_json::json!(tourney));
    Template::render("registerTeam", context)
}

#[post("/registerTeam?<tourney_id>", data="<data>", format="application/x-www-form-urlencoded")]
pub fn register_team_post(data: Form<InsertableTeam>, tourney_id: i32, conn: crate::TournamentDbConn) -> Result<Template, Template> {
    //send data to make new database entry
    println!("{:?}", &data.0);
    if diesel::insert_into(teams).values(&data.0).execute(&conn.0).is_ok() {//success
        Ok(Template::render("RegisterTeamSuccess", &data.0))
    } else {
        Err(Template::render("RegisterTeamFailure", &data.0))
    }
}
