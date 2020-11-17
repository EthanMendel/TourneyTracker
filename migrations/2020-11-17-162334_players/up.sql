CREATE TABLE players (
    id INT(11) NOT NULL AUTO_INCREMENT,
    `name` VARCHAR(250) NOT NULL,
    `number` INT(11) NOT NULL,
    team_id INT(11) NOT NULL,
    PRIMARY KEY (id)
);
