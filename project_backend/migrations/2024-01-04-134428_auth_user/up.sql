-- Your SQL goes here
CREATE TABLE auth_users (
    user_id UUID PRIMARY KEY,
	username VARCHAR(64) NOT NULL UNIQUE,
	fullname VARCHAR(128) NOT NULL,
	email VARCHAR(128) NOT NULL UNIQUE,
	password VARCHAR(512) NOT NULL
);