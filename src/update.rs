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

#[post("/updateDB?<tourney_id>&<game_id>&<to_update>&<how_to_change>")]
pub fn update_db(tourney_id: i32, game_id: i32, to_update: String, how_to_change: String, conn: crate::TournamentDbConn) -> Result<Template, Template> {
    let mut context = HashMap::new();
    let tourney_game = games.filter(games::dsl::tournament_id.eq(tourney_id)).first::<Game>(&conn.0).unwrap();
    let mut updated_row;
    match to_update.as_str() {
        "team_batting" => updated_row = diesel::update(games.filter(games::dsl::tournament_id.eq(tourney_id)))
            .set(team_batting.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).unwrap(),
        "team_1_batter" => updated_row = diesel::update(games.filter(games::dsl::tournament_id.eq(tourney_id)))
            .set(team_1_batter.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).unwrap(),
        "team_2_batter" => updated_row = diesel::update(games.filter(games::dsl::tournament_id.eq(tourney_id)))
            .set(team_2_batter.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).unwrap(),
        "inning" => updated_row = diesel::update(games.filter(games::dsl::tournament_id.eq(tourney_id)))
            .set(inning.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).unwrap(),
        "score" => updated_row = diesel::update(games.filter(games::dsl::tournament_id.eq(tourney_id)))
            .set(score.eq(how_to_change.as_str()))
            .execute(&conn.0).unwrap(),
        "batter" => updated_row = diesel::update(games.filter(games::dsl::tournament_id.eq(tourney_id)))
            .set(batter.eq(how_to_change.as_str()))
            .execute(&conn.0).unwrap(),
        "strike" => updated_row = diesel::update(games.filter(games::dsl::tournament_id.eq(tourney_id)))
            .set(strikes.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).unwrap(),
        "ball" => updated_row = diesel::update(games.filter(games::dsl::tournament_id.eq(tourney_id)))
            .set(balls.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).unwrap(),
        "out" => updated_row = diesel::update(games.filter(games::dsl::tournament_id.eq(tourney_id)))
            .set(outs.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).unwrap(),
        


        _ => {}
    }
    

    context.insert("game",serde_json::json!(tourney_game));

    todo!();
    // if diesel::insert_into(tournaments).values(&data.0).execute(&conn.0).is_ok() {//success
    //     Ok(Template::render("registerTournamentSuccess", &data.0))
    // } else {
    //     Err(Template::render("registerTournamentFailure", &data.0))
    // }
}
