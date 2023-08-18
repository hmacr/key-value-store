-- Add up migration script here
CREATE TABLE IF NOT EXISTS store (
	id VARCHAR(36) NOT NULL PRIMARY KEY,
	name VARCHAR(50) NOT NULL,
	data TEXT NOT NULL
);