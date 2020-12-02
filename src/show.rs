use rocket_contrib::templates::Template;
use std::collections::HashMap;
use crate::schema::tournaments::dsl::*;
use crate::schema::tournaments;
use crate::schema::teams::dsl::*;
use crate::schema::teams;
use crate::schema::games::dsl::*;
use crate::schema::games;
use crate::db::Tournament;
use crate::db::Team;
use crate::db::Game;
use diesel::prelude::*;

#[get("/showTournament?<tourney_id>")]
pub fn show_tournament(tourney_id: i32, conn: crate::TournamentDbConn) -> Template {
    let mut context = HashMap::new();
    let tourney = tournaments.filter(tournaments::dsl::id.eq(tourney_id)).first::<Tournament>(&conn.0).unwrap();
    context.insert("tournament",serde_json::json!(tourney));
    let tourney_teams = teams.filter(teams::dsl::tournament_id.eq(tourney_id)).load::<Team>(&conn.0).unwrap();
    context.insert("teams",serde_json::json!(tourney_teams));
    let tourney_games = games.filter(games::dsl::tournament_id.eq(tourney_id)).load::<Game>(&conn.0).unwrap();
    context.insert("games",serde_json::json!(tourney_games));
    println!("{:?}", context);
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
