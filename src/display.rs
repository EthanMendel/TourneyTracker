use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::schema::tournaments::dsl::*;
use crate::db::Tournament;
use diesel::prelude::*;
use rocket::http::RawStr;

#[database("tournaments")]
pub struct TournamentDbConn(diesel::MysqlConnection);

#[get("/showTournament?<tournamentId>")]
pub fn show_tournament(tournamentId: &RawStr, conn: TournamentDbConn) -> Template {
    let mut context = HashMap::new();
    let tourneys_db = tournaments.select(id).load::<Tournament>(&conn.0)?;
    assert_eq!(vec![tournamentId], tourneys_db);
    context.insert("tournament",serde_json::json!(tourneys_db));
    Template::render("showTournament", context)//context)
}