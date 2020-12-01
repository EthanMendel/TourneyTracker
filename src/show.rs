use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::schema::tournaments::dsl::*;
// use crate::schema::games::dsl::*;
use crate::db::Tournament;
use diesel::prelude::*;

#[get("/showTournament?<tournament_id>")]
pub fn show_tournament(tournament_id: i32, conn: crate::TournamentDbConn) -> Template {
    let mut context = HashMap::new();
    let tourney = tournaments.filter(id.eq(tournament_id)).first::<Tournament>(&conn.0).unwrap();
    context.insert("tournament",serde_json::json!(tourney));
    Template::render("showTournament", context)
}

// #[get("/showGame?<game_id>")]
// pub fn show_game(game_id: i32, conn: TournamentDbConn) -> Template {
//     let mut context = HashMap::new();
//     let game = games.filter(id.eq(game_id)).load::<Game>(&conn.0).unwrap();
//     context.insert("tournament",serde_json::json!(game));
//     Template::render("showGame", context)
// }
#[get("/showGame")]
pub fn show_game() -> Template{
    let mut context: HashMap<&str, &str> = HashMap::new();
    Template::render("showGame", context)
}
