-- Your SQL goes here
CREATE TABLE users (
	id 			BIGSERIAL PRIMARY KEY,
	name		VARCHAR(20)		NOT NULL UNIQUE,
	hair_color	VARCHAR(10)		NOT NULL DEFAULT 'BK',
	created			TIMESTAMP		NOT NULL DEFAULT (now()),
	expiration_time TIMESTAMP		NOT NULL
);
