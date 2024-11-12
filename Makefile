
BINARY_NAME=rainbow
RELEASE_DIR=target/release
RELEASE_BIN_DIR=bin
VERSION=0_1
PROVIDER_DATABASE_URL=postgresql://ds-protocol-provider:ds-protocol-provider@localhost:5433/ds-protocol-provider
CONSUMER_DATABASE_URL=postgresql://ds-protocol:ds-protocol@localhost:5434/ds-protocol

all:
	build

build:
	cargo build --release
	mkdir -p $(RELEASE_BIN_DIR)
	cp $(RELEASE_DIR)/$(BINARY_NAME) $(RELEASE_BIN_DIR)/$(BINARY_NAME)-$(VERSION)

clean:
	cargo clean
	rm -rf $(RELEASE_BIN_DIR)

dev:
	docker compose -f ./deployment/docker-compose.dev.yaml up -d

build-container:
	echo "container..."

create-provider-migration:
	export $(cat $(PWD)/.env | xargs)
	diesel migration generate \
		--diff-schema provider \
		--database-url $(PROVIDER_DATABASE_URL) \
		--schema-key provider

run-provider-migration:
	diesel migration run \
		--database-url $(PROVIDER_DATABASE_URL) \
		--migration-dir ./rainbow-core/src/db/provider_migrations

create-consumer-migration:
	export $(cat $(PWD)/.env | xargs)
	diesel migration generate \
		--diff-schema consumer \
		--database-url $(CONSUMER_DATABASE_URL) \
		--schema-key consumer

run-consumer-migration:
	diesel migration run \
		--database-url $(CONSUMER_DATABASE_URL)
		--schema-key consumer

run-migration:
	diesel migration run \
		--database-url $(DATABASE_URL)

start-static-server:
	cd ./test/data-servers/static-parquet-server \
	cargo run \
		1236