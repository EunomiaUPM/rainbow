-- Your SQL goes here
CREATE TABLE IF NOT EXISTS  "transfer_callbacks" (
	"id" UUID NOT NULL PRIMARY KEY,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP
);