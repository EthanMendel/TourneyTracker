CREATE TABLE games (
    id INT(11) NOT NULL AUTO_INCREMENT,
    game_level INT(11) NOT NULL,
    tournament_id INT(11) NOT NULL,
    team_1_id INT(11) NOT NULL,
    team_2_id INT(11) NOT NULL,
    team_batting INT(11) NOT NULL,
    team_1_batter INT(11) NOT NULL,
    team_2_batter INT(11) NOT NULL,
    inning INT(11) NOT NULL,
    score VARCHAR(250) NOT NULL,
    batter VARCHAR(250) NOT NULL,
    strikes INT(11) NOT NULL,
    balls INT(11) NOT NULL,
    outs INT(11) NOT NULL,
    PRIMARY KEY (id)
);
