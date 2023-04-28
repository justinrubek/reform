-- Your SQL goes here
CREATE TABLE users (
	id SERIAL NOT NULL PRIMARY KEY,
	email TEXT NOT NULL,
	hash TEXT NOT NULL
);
