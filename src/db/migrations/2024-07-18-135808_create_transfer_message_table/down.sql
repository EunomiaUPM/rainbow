-- This file should undo anything in `up.sql`
CREATE TABLE "transfer_message"(
	"id" UUID NOT NULL PRIMARY KEY,
	"transfer_session" UUID NOT NULL,
	"provider_pid" UUID NOT NULL,
	"consumer_pid" UUID NOT NULL,
	"content" JSON NOT NULL
);


DROP TABLE IF EXISTS "transfer_messages";
