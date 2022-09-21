-- Your SQL goes here

CREATE TABLE projects (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    directory TEXT NOT NULL,
    start DATE NOT NULL,
    end DATE
);

CREATE TABLE aircrafts (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    registration TEXT
);

CREATE TABLE pilots (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    registration TEXT
);

CREATE TABLE locations (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    kml TEXT NOT NULL,
    project_id INTEGER NOT NULL
);

CREATE TABLE flights (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    datasets INTEGER NOT NULL,
    location_id INTEGER NOT NULL,
    aircraft_id INTEGER NOT NULL,
    pilot_id INTEGER NOT NULL,
    start TIMESTAMP NOT NULL,
    end TIMESTAMP NOT NULL,
    speed_ms INTEGER NOT NULL,
    height_m INTEGER NOT NULL
);

CREATE TABLE datasets (
    id INTEGER NOT NULL PRIMARY KEY,
    flight_number INTEGER NOT NULL,
    flight_id INTEGER NOT NULL,
    start TIMESTAMP NOT NULL,
    end TIMESTAMP NOT NULL
);
