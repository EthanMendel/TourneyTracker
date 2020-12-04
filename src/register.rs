use rocket_contrib::templates::Template;
use std::collections::HashMap;
use rocket::request::Form;
use crate::db::{ InsertableTournament, InsertableTeam, InsertableGame };
use crate::schema::tournaments::dsl::*;
use crate::schema::tournaments;
use crate::schema::teams::dsl::*;
use diesel::prelude::*;
use crate::db::Tournament;
use crate::db::Game;
use crate::schema::games::dsl::*;
use crate::schema::games;
use crate::schema::teams;
use crate::db::Team;
use crate::db::game_level::GameLevel::Semifinal;
use crate::db::game_level::GameLevel::Final;


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
        Ok(Template::render("registerTournamentSuccess", &data.0))
    } else {
        Err(Template::render("registerTournamentFailure", &data.0))
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
    let team = InsertableTeam {
        tournament_id: Some(tourney_id),
        record: Some("0-0".to_string()),
        ..data.0
    };
    if diesel::insert_into(teams).values(&team).execute(&conn.0).is_ok() {//success
        Ok(Template::render("RegisterTeamSuccess", &team))
    } else {
        Err(Template::render("RegisterTeamFailure", &team))
    }
}

#[post("/registerGames?<tourney_id>")]
pub fn register_game_post(tourney_id: i32, conn: crate::TournamentDbConn) -> Result<Template, Template> {
    let mut context = HashMap::new();
    let tourney = tournaments.filter(tournaments::dsl::id.eq(tourney_id)).first::<Tournament>(&conn.0).unwrap();
    context.insert("tournament",serde_json::json!(tourney));
    let tourney_teams = teams.filter(teams::dsl::tournament_id.eq(tourney_id)).load::<Team>(&conn.0).unwrap();
    context.insert("teams",serde_json::json!(tourney_teams));
    let mut game_vec = Vec::new();
    let mut game = InsertableGame{
        game_level: Semifinal,
        tournament_id: tourney_id,
        team_1_id: 0,
        team_2_id: 0,
        team_batting: 0,
        team_1_batter: 0,
        team_2_batter: 0,
        inning: 1,
        score: "0-0".to_string(),
        batter: "".to_string(),
        strikes: 0,
        balls: 0,
        outs: 0
    };
    let mut needExtraAdd = false;
    for (i, t) in tourney_teams.iter().enumerate(){
        println!("{:?}", i);
        if i % 2 == 0 {
            needExtraAdd = true;
            game.team_1_id = t.id;
            game.team_batting = t.id;
        }else{
            needExtraAdd = false;
            game.team_2_id = t.id;
            println!("{:?}", game);
            game_vec.push(game);
            game = InsertableGame{
                game_level: Semifinal,
                tournament_id: tourney_id,
                team_1_id: 0,
                team_2_id: 0,
                team_batting: 0,
                team_1_batter: 0,
                team_2_batter: 0,
                inning: 1,
                score: "0-0".to_string(),
                batter: "".to_string(),
                strikes: 0,
                balls: 0,
                outs: 0
            };
        }
    }
    if needExtraAdd {
        println!("{:?}", game);
        game_vec.push(game);
    }
    if game_vec.len() == 1 {
        game_vec[0].game_level = Final;
    }
    context.insert("games",serde_json::json!(game_vec));
    if diesel::insert_into(games).values(&game_vec).execute(&conn.0).is_ok() {//success
        Ok(Template::render("showTournament", context))
    } else {
        Err(Template::render("makeBracketFailure", &game_vec))
    }    
}

#[post("/gameFinsihed?<game_id>&<tourney_id>")]
pub fn game_finished(game_id: i32, tourney_id: i32, conn: crate::TournamentDbConn) -> Result<String, String> {
    let tourney_game = games.filter(games::dsl::game_level.eq(Final)).first::<Game>(&conn.0);
    if tourney_game.is_err() {
        make_final(game_id,tourney_id,conn)
    }else{
        add_to_final(game_id,tourney_id,conn)
    }
}



fn make_final(game_id: i32, tourney_id: i32, conn: crate::TournamentDbConn) -> Result<String, String> {
    let tourney_game = games.filter(games::dsl::id.eq(game_id)).first::<Game>(&conn.0).unwrap();
    let win_1_id;
    let scores: Vec<&str> = tourney_game.score.split("-").collect();
    if scores[0] > scores[1] {
        win_1_id = tourney_game.team_1_id;
    }else{
        win_1_id = tourney_game.team_2_id;
    }
    let new_game = InsertableGame{
        game_level: Final,
        tournament_id: tourney_id,
        team_1_id: win_1_id,
        team_2_id: 0,
        team_batting: 0,
        team_1_batter: 0,
        team_2_batter: 0,
        inning: 1,
        score: "0-0".to_string(),
        batter: "".to_string(),
        strikes: 0,
        balls: 0,
        outs: 0
    };
    if diesel::insert_into(games).values(&new_game).execute(&conn.0).is_ok() {//success
        Ok(serde_json::json!({"success": true,
                              "msg": format!("Game {} finished. Made Final", game_id),
                            }).to_string())
    } else {
        Err(serde_json::json!({ "success": true,
                                "msg": format!("Game {} finished. Failed to Make Final", game_id),
                            }).to_string())
    }    
}

fn add_to_final(game_id: i32, tourney_id: i32, conn: crate::TournamentDbConn) -> Result<String, String> {
    let tourney_game = games.filter(games::dsl::id.eq(game_id)).first::<Game>(&conn.0).unwrap();
    let win_2_id;
    let scores: Vec<&str> = tourney_game.score.split("-").collect();
    if scores[0] > scores[1] {
        win_2_id = tourney_game.team_1_id;
    }else{
        win_2_id = tourney_game.team_2_id;
    }
    let updated_row = diesel::update(games.filter(games::dsl::game_level.eq(Final)))
            .set(team_2_id.eq(win_2_id))
            .execute(&conn.0).is_ok();
    if updated_row {//success
        Ok(serde_json::json!({"success": true,
                                "msg": format!("Game {} finished. Added to Final", game_id),
                            }).to_string())
    } else {
        Err(serde_json::json!({ "success": true,
                                "msg": format!("Game {} finished. Failed to Add to Final", game_id),
                            }).to_string())
    }    
          
}