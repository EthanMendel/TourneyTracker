use crate::schema::*;

#[derive(Identifiable, Queryable, Debug, PartialEq, Clone)]
pub struct Tournament {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Tournament)]
pub struct Game {
    pub id: i32,
    pub tournament_id: i32,
    pub team_1_id: i32,
    pub team_2_id: i32,
    pub inning: i32,
    pub score: String,
    pub batter: i32,
    pub strikes: i32,
    pub balls: i32,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub record: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug)]
#[belongs_to(Team)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub number: i32,
    pub team_id: i32,
}
