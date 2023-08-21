-- Add migration script here
DROP TABLE bio;
DROP TABLE project;
DROP TABLE skill;

CREATE TABLE bio (
    id SERIAL PRIMARY KEY,
    bio_content VARCHAR(1000) NOT NULL
);

CREATE TABLE project (
    id SERIAL PRIMARY KEY,
    name VARCHAR(80) NOT NULL,
    description VARCHAR(240) NOT NULL
);

CREATE TABLE skill (
    id SERIAL PRIMARY KEY,
    name VARCHAR(80) NOT NULL
);
