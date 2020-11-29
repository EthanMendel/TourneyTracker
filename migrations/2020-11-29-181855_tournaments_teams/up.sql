CREATE TABLE tournaments_teams (
    id INT(11) NOT NULL AUTO_INCREMENT,
    tournament_id INT(11) NOT NULL,
    team_id INT(11) NOT NULL,
    PRIMARY KEY (id)
);
