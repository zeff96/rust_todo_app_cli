-- Your SQL goes here

CREATE TABLE todos(
    id INT GENERATED ALWAYS AS IDENTITY,
    description VARCHAR(255) NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY(id)
);