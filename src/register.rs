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
use crate::schema::teams;
use crate::db::Team;
use crate::db::game_level::GameLevel::Semifinal;


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
        if (i % 2 == 0) {
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
    if (needExtraAdd){
        println!("{:?}", game);
        game_vec.push(game);
    }
    context.insert("games",serde_json::json!(game_vec));
    if diesel::insert_into(games).values(&game_vec).execute(&conn.0).is_ok() {//success
        Ok(Template::render("showTournament", context))
    } else {
        Err(Template::render("makeBracketFailure", &game_vec))
    }    
    
}