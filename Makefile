
BINARY_NAME=rainbow
RELEASE_DIR=target/release
RELEASE_BIN_DIR=bin
VERSION=0_1
SCHEMA_NAME=b

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

create-migration:
	export $(cat $(PWD)/.env | xargs)
	diesel migration generate \
		--diff-schema $(SCHEMA_NAME) \
		--database-url $(DATABASE_URL)

run-migration:
	diesel migration run \
		--database-url $(DATABASE_URL)

start-static-server:
	cargo run \
		--manifest-path ./test/data-servers/static-parquet-server/Cargo.toml \
		1236