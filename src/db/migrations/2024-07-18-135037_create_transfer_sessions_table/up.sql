-- Your SQL goes here
CREATE TABLE "transfer_sessions"(
	"id" UUID NOT NULL PRIMARY KEY,
	"provider_pid" UUID NOT NULL,
	"consumer_pid" UUID NOT NULL,
	"state" VARCHAR NOT NULL,
	"created_at" DATE NOT NULL,
	"updated_at" DATE
);

