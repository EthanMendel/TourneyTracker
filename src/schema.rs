table! {
    games (id) {
        id -> Integer,
        game_level -> Integer,
        tournament_id -> Integer,
        team_1_id -> Integer,
        team_2_id -> Integer,
        team_batting -> Integer,
        team_1_batter -> Integer,
        team_2_batter -> Integer,
        inning -> Integer,
        score -> Varchar,
        batter -> Varchar,
        strikes -> Integer,
        balls -> Integer,
        outs -> Integer,
    }
}

table! {
    teams (id) {
        id -> Integer,
        tournament_id -> Integer,
        name -> Varchar,
        record -> Varchar,
        pitcher -> Nullable<Varchar>,
        catcher -> Nullable<Varchar>,
        base_1 -> Nullable<Varchar>,
        base_2 -> Nullable<Varchar>,
        short_stop -> Nullable<Varchar>,
        base_3 -> Nullable<Varchar>,
        right_field -> Nullable<Varchar>,
        center_field -> Nullable<Varchar>,
        left_field -> Nullable<Varchar>,
    }
}

table! {
    tournaments (id) {
        id -> Integer,
        name -> Varchar,
    }
}

table! {
    tournaments_teams (id) {
        id -> Integer,
        tournament_id -> Integer,
        team_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    games,
    teams,
    tournaments,
    tournaments_teams,
);
