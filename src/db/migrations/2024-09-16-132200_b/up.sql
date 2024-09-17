-- Your SQL goes here

ALTER TABLE "transfer_sessions" DROP COLUMN "created_at";
ALTER TABLE "transfer_sessions" DROP COLUMN "updated_at";
ALTER TABLE "transfer_sessions" ADD COLUMN "created_at" TIMESTAMP NOT NULL;
ALTER TABLE "transfer_sessions" ADD COLUMN "updated_at" TIMESTAMP;

