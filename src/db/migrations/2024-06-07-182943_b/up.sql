-- Your SQL goes here
CREATE TABLE "protocol_versions"(
	"id" UUID NOT NULL PRIMARY KEY,
	"path" VARCHAR NOT NULL
);

CREATE TABLE "users"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"name" VARCHAR NOT NULL,
	"email" VARCHAR NOT NULL,
	"lol" VARCHAR NOT NULL
);

