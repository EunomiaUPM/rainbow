-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "dataset_catalogs" (
	"dataset_id" UUID NOT NULL PRIMARY KEY,
	"dataset_endpoint" VARCHAR NOT NULL
);