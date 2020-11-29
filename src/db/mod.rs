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

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Tournament)]
pub struct Game {
    pub id: i32,
    pub tournament_id: i32,
    pub team_1_id: i32,
    pub team_2_id: i32,
    pub team_batting: i32,
    pub inning: i32,
    pub score: String,
    pub batter: i32,
    pub strikes: i32,
    pub balls: i32,
    pub catcher: i32,
    pub pitcher: i32,
    pub base_1: i32,
    pub base_2: i32,
    pub short_stop: i32,
    pub base_3: i32,
    pub right_field: i32,
    pub center_field: i32,
    pub left_field: i32
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub record: String,
}

#[derive(Insertable, Debug, PartialEq, Clone, Serialize, FromForm)]
#[table_name = "teams"]
pub struct InsertableTeam {
    pub name: String,
    pub record: String,
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

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Team)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub number: i32,
    pub team_id: i32,
}
