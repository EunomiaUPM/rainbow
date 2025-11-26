# Define variables
DOCKER_USERNAME ?= quay.io/eunomia_upm
VERSION ?= 0.3.4

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
# Transfer Agent Microservice
#
#
build-transfer-agent:
	docker build \
		--progress plain \
		-t $(DOCKER_USERNAME)/rainbow-transfer-agent:$(VERSION) \
		--build-arg APP_NAME=rainbow_transfer \
		-f deployment/Dockerfile \
		.

push-transfer-agent:
	docker push $(DOCKER_USERNAME)/rainbow-transfer-agent:$(VERSION)


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
# Rainbow FE Gateway
#
#
build-fe-gateway:
	docker build \
		--progress plain \
		-t $(DOCKER_USERNAME)/rainbow_fe_gateway:$(VERSION) \
		--build-arg APP_NAME=rainbow_fe_gateway \
		-f deployment/Dockerfile \
		.

push-fe-gateway:
	docker push $(DOCKER_USERNAME)/rainbow_fe_gateway:$(VERSION)


#
#
#
# Rainbow Business Gateway
#
#
build-business-gateway:
	docker build \
		--progress plain \
		-t $(DOCKER_USERNAME)/rainbow_business_gateway:$(VERSION) \
		--build-arg APP_NAME=rainbow_business_gateway \
		-f deployment/Dockerfile \
		.

push-business-gateway:
	docker push $(DOCKER_USERNAME)/rainbow_business_gateway:$(VERSION)


#
#
#
# Rainbow Authority
#
#
build-authority:
	docker build \
		--progress plain \
		-t $(DOCKER_USERNAME)/rainbow_authority:$(VERSION) \
		--build-arg APP_NAME=rainbow_authority \
		-f deployment/Dockerfile \
		.

push-authority:
	docker push $(DOCKER_USERNAME)/rainbow_authority:$(VERSION)


#
#
#
# General
#
#
build-containers: build-core build-catalog build-contracts build-transfer build-transfer-agent build-auth build-fe-gateway build-business-gateway build-authority

push-containers: push-core push-catalog push-contracts push-transfer push-transfer-agent push-auth push-fe-gateway push-business-gateway push-authority

.PHONY: build-containers push-containers