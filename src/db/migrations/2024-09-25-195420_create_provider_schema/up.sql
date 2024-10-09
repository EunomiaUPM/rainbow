-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "transfer_processes"(
	"provider_pid" UUID NOT NULL PRIMARY KEY,
	"consumer_pid" UUID NOT NULL,
	"state" VARCHAR NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP
);

CREATE TABLE IF NOT EXISTS "transfer_messages"(
	"id" UUID NOT NULL PRIMARY KEY,
	"transfer_process_id" UUID NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"message_type" VARCHAR NOT NULL,
	"content" JSONB NOT NULL,
	FOREIGN KEY ("transfer_process_id") REFERENCES "transfer_processes"("provider_pid")
);

CREATE TABLE IF NOT EXISTS "data_plane_processes" (
	"data_plane_id" UUID NOT NULL PRIMARY KEY,
	"transfer_process_id" UUID NOT NULL,
	"agreement_id" UUID NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP,
	"state" BOOLEAN NOT NULL,
	FOREIGN KEY ("transfer_process_id") REFERENCES "transfer_processes"("provider_pid")
);