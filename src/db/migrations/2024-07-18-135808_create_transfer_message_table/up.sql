-- Your SQL goes here
DROP TABLE IF EXISTS "transfer_message";

CREATE TABLE "transfer_messages"(
	"id" UUID NOT NULL PRIMARY KEY,
	"transfer_session" UUID NOT NULL,
	"provider_pid" UUID NOT NULL,
	"consumer_pid" UUID NOT NULL,
	"content" JSON NOT NULL,
	FOREIGN KEY ("transfer_session") REFERENCES "transfer_sessions"("id")
);

