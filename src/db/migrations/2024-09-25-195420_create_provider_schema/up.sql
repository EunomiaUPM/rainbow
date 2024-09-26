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
	FOREIGN KEY ("transfer_process_id") REFERENCES "transfer_processes"("provider_pid")
);

CREATE TABLE IF NOT EXISTS  "transfer_message_fields"(
	"id" UUID NOT NULL PRIMARY KEY,
	"transfer_message_id" UUID NOT NULL,
	"key" VARCHAR NOT NULL,
	"value" VARCHAR NOT NULL,
	"parent" UUID,
	FOREIGN KEY ("transfer_message_id") REFERENCES "transfer_messages"("id")
);