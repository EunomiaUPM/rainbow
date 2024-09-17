-- This file should undo anything in `up.sql`

ALTER TABLE "transfer_sessions" DROP COLUMN "created_at";
ALTER TABLE "transfer_sessions" DROP COLUMN "updated_at";
ALTER TABLE "transfer_sessions" ADD COLUMN "created_at" DATE NOT NULL;
ALTER TABLE "transfer_sessions" ADD COLUMN "updated_at" DATE;

