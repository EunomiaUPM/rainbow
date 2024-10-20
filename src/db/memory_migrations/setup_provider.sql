INSTALL json;
LOAD json;
CREATE SCHEMA IF NOT EXISTS provider;
CREATE TABLE IF NOT EXISTS provider.transfer_processes (
    provider_pid VARCHAR NOT NULL PRIMARY KEY,
    consumer_pid VARCHAR NOT NULL,
    state VARCHAR NOT NULL,
    created_at TIME NOT NULL,
    updated_at TIME
);
CREATE TABLE IF NOT EXISTS provider.transfer_messages (
    id VARCHAR NOT NULL PRIMARY KEY,
    transfer_process_id VARCHAR NOT NULL REFERENCES provider.transfer_processes(provider_pid),
    created_at TIME NOT NULL,
    message_type VARCHAR NOT NULL,
    content JSON
);
CREATE TABLE IF NOT EXISTS provider.data_plane_processes (
    data_plane_id VARCHAR NOT NULL PRIMARY KEY,
    transfer_process_id VARCHAR NOT NULL REFERENCES provider.transfer_processes(provider_pid),
    agreement_id VARCHAR NOT NULL,
    created_at TIME NOT NULL,
    updated_at TIME,
    state BOOLEAN NOT NULL
);
CREATE TABLE IF NOT EXISTS provider.dataset_catalogs (
    dataset_id VARCHAR NOT NULL PRIMARY KEY,
    dataset_endpoint VARCHAR NOT NULL
);
CREATE TABLE IF NOT EXISTS provider.contract_agreements (
    agreement_id VARCHAR NOT NULL PRIMARY KEY,
    dataset_id VARCHAR NOT NULL
);
