INSTALL json;
LOAD json;
CREATE SCHEMA IF NOT EXISTS consumer;
CREATE TABLE IF NOT EXISTS consumer.transfer_callbacks (
    id VARCHAR NOT NULL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    provider_pid VARCHAR,
    consumer_pid VARCHAR,
    data_address JSON
);
