-- Add up migration script here
CREATE TABLE IF NOT EXISTS users (
	id VARCHAR(36) PRIMARY KEY NOT NULL,
	name VARCHAR(36) NOT NULL,
	password_hash CHAR(64) NOT NULL,
	password_salt CHAR(32) NOT NULL
);