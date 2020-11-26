use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::schema::tournaments::dsl::*;
use crate::db::Tournament;
use diesel::prelude::*;

#[database("tournaments")]
pub struct TournamentDbConn(diesel::MysqlConnection);

#[get("/showTournament?<tournament_id>")]
pub fn show_tournament(tournament_id: i32, conn: TournamentDbConn) -> Template {
    let mut context = HashMap::new();
    let tourney = tournaments.filter(id.eq(tournament_id)).load::<Tournament>(&conn.0).unwrap();
    context.insert("tournament",serde_json::json!(tourney));
    Template::render("showTournament", context)//context)
}