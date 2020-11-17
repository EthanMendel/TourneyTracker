table! {
    games (id) {
        id -> Integer,
        team_1_id -> Integer,
        team_2_id -> Integer,
        inning -> Nullable<Integer>,
        score -> Nullable<Varchar>,
        batter -> Nullable<Integer>,
        strikes -> Nullable<Integer>,
        balls -> Nullable<Integer>,
    }
}

table! {
    players (id) {
        id -> Integer,
        name -> Varchar,
        number -> Integer,
        team_id -> Integer,
    }
}

table! {
    teams (id) {
        id -> Integer,
        name -> Varchar,
        record -> Varchar,
    }
}

table! {
    tournaments (id) {
        id -> Integer,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    games,
    players,
    teams,
    tournaments,
);
