CREATE TABLE games (
    id INT(11) NOT NULL AUTO_INCREMENT,
    tournament_id INT(11) NOT NULL,
    team_1_id INT(11) NOT NULL,
    team_2_id INT(11) NOT NULL,
    inning INT(11),
    score VARCHAR(250),
    batter INT(11),
    strikes INT(11),
    balls INT(11),
    PRIMARY KEY (id)
);
