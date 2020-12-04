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
pub fn update_db(tourney_id: i32, game_id: i32, to_update: String, how_to_change: String, conn: crate::TournamentDbConn) -> Result<String, String> {
    let mut updated_row= false;
    match to_update.as_str() {
        "team_batting" => updated_row = diesel::update(games.filter(games::dsl::id.eq(game_id)))
            .set(team_batting.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).is_ok(),
        "team_1_batter" => updated_row = diesel::update(games.filter(games::dsl::id.eq(game_id)))
            .set(team_1_batter.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).is_ok(),
        "team_2_batter" => updated_row = diesel::update(games.filter(games::dsl::id.eq(game_id)))
            .set(team_2_batter.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).is_ok(),
        "inning" => updated_row = diesel::update(games.filter(games::dsl::id.eq(game_id)))
            .set(inning.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).is_ok(),
        "score" => updated_row = diesel::update(games.filter(games::dsl::id.eq(game_id)))
            .set(score.eq(how_to_change.as_str()))
            .execute(&conn.0).is_ok(),
        "batter" => updated_row = diesel::update(games.filter(games::dsl::id.eq(game_id)))
            .set(batter.eq(how_to_change.as_str()))
            .execute(&conn.0).is_ok(),
        "strike" => updated_row = diesel::update(games.filter(games::dsl::id.eq(game_id)))
            .set(strikes.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).is_ok(),
        "ball" => updated_row = diesel::update(games.filter(games::dsl::id.eq(game_id)))
            .set(balls.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).is_ok(),
        "out" => updated_row = diesel::update(games.filter(games::dsl::id.eq(game_id)))
            .set(outs.eq(how_to_change.parse::<i32>().unwrap()))
            .execute(&conn.0).is_ok(),
        _ => {}
    }
    if updated_row {
        Ok(serde_json::json!({"success": true,
                              "to_update": to_update,
                              "how_to_change": how_to_change}).to_string())
    }else{
        Err(serde_json::json!({"success": false,
                              "to_update": to_update,
                              "how_to_change": how_to_change}).to_string())
    }
}
