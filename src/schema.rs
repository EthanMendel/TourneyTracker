table! {
    games (id) {
        id -> Integer,
        game_level -> Integer,
        tournament_id -> Integer,
        team_1_id -> Integer,
        team_2_id -> Integer,
        team_batting -> Nullable<Integer>,
        inning -> Nullable<Integer>,
        score -> Nullable<Varchar>,
        batter -> Nullable<Integer>,
        strikes -> Nullable<Integer>,
        balls -> Nullable<Integer>,
        catcher -> Nullable<Integer>,
        pitcher -> Nullable<Integer>,
        base_1 -> Nullable<Integer>,
        base_2 -> Nullable<Integer>,
        short_stop -> Nullable<Integer>,
        base_3 -> Nullable<Integer>,
        right_field -> Nullable<Integer>,
        center_field -> Nullable<Integer>,
        left_field -> Nullable<Integer>,
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

table! {
    tournaments_teams (id) {
        id -> Integer,
        tournament_id -> Integer,
        team_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    games,
    players,
    teams,
    tournaments,
    tournaments_teams,
);
