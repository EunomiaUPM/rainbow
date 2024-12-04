
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

start-static-server:
	cd ./test/data-servers/static-parquet-server \
	cargo run \
		1236

build-container:
	docker build \
		--progress plain \
		-t caparicioesd/rainbow:latest \
		-f deployment/Dockerfile \
		.

push-container:
	docker push caparicioesd/rainbow:latest