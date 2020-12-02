CREATE TABLE teams (
    id INT(11) NOT NULL AUTO_INCREMENT,
    tournament_id INT(11) NOT NULL,
    `name` VARCHAR(250) NOT NULL,
    record VARCHAR(11) NOT NULL,
    pitcher VARCHAR(250),
    catcher VARCHAR(250),
    base_1 VARCHAR(250),
    base_2 VARCHAR(250),
    short_stop VARCHAR(250),
    base_3 VARCHAR(250),
    right_field VARCHAR(250),
    center_field VARCHAR(250),
    left_field VARCHAR(250),
    PRIMARY KEY (id)
);
