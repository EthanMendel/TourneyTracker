pub mod game_level;

use crate::schema::*;
use serde::Serialize;

#[derive(Queryable, Debug, PartialEq, Clone, Serialize, Identifiable)]
pub struct Tournament {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug, PartialEq, Clone, Serialize, FromForm)]
#[table_name = "tournaments"]
pub struct InsertableTournament {
    pub name: String,
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Associations)]
#[belongs_to(Tournament)]
pub struct Game {
    pub id: i32,
    #[diesel(deserialize_as = "i32")]
    pub game_level: game_level::GameLevel,
    pub tournament_id: i32,
    pub team_1_id: i32,
    pub team_2_id: i32,
    pub team_batting: i32,
    pub team_1_batter: i32,
    pub team_2_batter: i32,
    pub inning: i32,
    pub score: String,
    pub batter: String,
    pub strikes: i32,
    pub balls: i32,//need to add outs
    pub outs: i32
}

#[derive(Insertable, Queryable, Associations, PartialEq, Debug, Serialize)]
#[table_name = "games"]
pub struct InsertableGame {
    #[diesel(deserialize_as = "i32")]
    pub game_level: game_level::GameLevel,
    pub tournament_id: i32,
    pub team_1_id: i32,
    pub team_2_id: i32,
    pub team_batting: i32,
    pub team_1_batter: i32,
    pub team_2_batter: i32,
    pub inning: i32,
    pub score: String,
    pub batter: String,
    pub strikes: i32,
    pub balls: i32,//need to add outs
    pub outs: i32
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize, Associations)]
#[belongs_to(Tournament)]
pub struct Team {
    pub id: i32,
    pub tournament_id: i32,
    pub name: String,
    pub record: String,
    pub pitcher: Option<String>,
    pub catcher: Option<String>,
    pub base_1: Option<String>,
    pub base_2: Option<String>,
    pub short_stop: Option<String>,
    pub base_3: Option<String>,
    pub right_field: Option<String>,
    pub center_field: Option<String>,
    pub left_field:Option<String>
}

#[derive(Insertable, Debug, PartialEq, Clone, Serialize, FromForm)]
#[table_name = "teams"]
pub struct InsertableTeam {
    pub name: String,
    pub record: Option<String>,
    pub tournament_id: Option<i32>,
    pub pitcher: String,
    pub catcher: String,
    pub base_1: String,
    pub base_2: String,
    pub short_stop: String,
    pub base_3: String,
    pub right_field: String,
    pub center_field: String,
    pub left_field:String
}

#[derive(Identifiable, Queryable, Debug, Associations, Serialize)]
#[belongs_to(Tournament)]
#[belongs_to(Team)]
#[table_name = "tournaments_teams"]
pub struct TournamentTeam {
    pub id: i32,
    pub tournament_id: i32,
    pub team_id: i32,
}

#[derive(Insertable, Debug, PartialEq, Clone, Serialize)]
#[table_name = "tournaments_teams"]
pub struct InsertableTournamentTeam {
    pub tournament_id: i32,
    pub team_id: i32,
}
