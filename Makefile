# Define variables
DOCKER_USERNAME ?= caparicioesd
VERSION ?= latest

#
#
#
# Transfer Microservice
#
#
build-transfer:
	docker build \
		--progress plain \
		-t $(DOCKER_USERNAME)/rainbow-transfer:$(VERSION) \
		--build-arg APP_NAME=rainbow_transfer \
		-f deployment/Dockerfile \
		.

push-transfer:
	docker push $(DOCKER_USERNAME)/rainbow-transfer:$(VERSION)


#
#
#
# Contract negotiation Microservice
#
#
build-contracts:
	docker build \
		--progress plain \
		-t $(DOCKER_USERNAME)/rainbow-contracts:$(VERSION) \
		--build-arg APP_NAME=rainbow_contracts \
		-f deployment/Dockerfile \
		.

push-contracts:
	docker push $(DOCKER_USERNAME)/rainbow-contracts:$(VERSION)


#
#
#
# Rainbow Catalog Microservice
#
#
build-catalog:
	docker build \
		--progress plain \
		-t $(DOCKER_USERNAME)/rainbow-catalog:$(VERSION) \
		--build-arg APP_NAME=rainbow_catalog \
		-f deployment/Dockerfile \
		.

push-catalog:
	docker push $(DOCKER_USERNAME)/rainbow-catalog:$(VERSION)


#
#
#
# Rainbow Auth
#
#
build-auth:
	docker build \
		--progress plain \
		-t $(DOCKER_USERNAME)/rainbow-auth:$(VERSION) \
		--build-arg APP_NAME=rainbow_auth \
		-f deployment/Dockerfile \
		.

push-auth:
	docker push $(DOCKER_USERNAME)/rainbow-auth:$(VERSION)


#
#
#
# Rainbow Core as Monolith
#
#
build-core:
	docker build \
		--progress plain \
		-t $(DOCKER_USERNAME)/rainbow:$(VERSION) \
		--build-arg APP_NAME=rainbow_core \
		-f deployment/Dockerfile \
		.

push-core:
	docker push $(DOCKER_USERNAME)/rainbow:$(VERSION)


#
#
#
# General
#
#
build-containers: build-core build-catalog build-contracts build-transfer build-auth

push-containers: push-core push-catalog push-contracts push-transfer push-auth

.PHONY: build-ƒtransfer push-transfer build-contracts push-contracts build-catalog push-catalog build-auth push-auth build-core push-core build-containers push-containers