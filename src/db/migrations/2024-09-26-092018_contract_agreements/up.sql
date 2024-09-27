-- Your SQL goes here
CREATE TABLE IF NOT EXISTS  "contract_agreements" (
	"agreement_id" UUID NOT NULL PRIMARY KEY,
	"dataset_id" UUID NOT NULL
);