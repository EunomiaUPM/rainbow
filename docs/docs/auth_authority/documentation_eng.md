# Rainbow Identity System

## Table of Contents
- [Overview](#overview)
- [Consumer](#consumer)
- [Provider](#provider)
- [Authority](#authority)
- [Wallet](#wallet)
- [Tests](#tests)
- [Summary](#summary)

## Overview

This document provides the documentation to understand the authentication system developed for Rainbow.

## Consumer

Interacts with the rest of the "rainbow_consumer" services, as defined by Carlos.  
It has its own database (Postgres) (currently uses the shared Consumer DB, to be migrated to its own DB later).

### Deployment

- **Deployment Type**
    - _Monolith_
  ```bash
  cd rainbow-core
  ```
    - _Individual_
  ```bash
  cd rainbow-auth
  ```
- **Initialization**
    - _Database_
  ```bash
  cd deployment
  docker-compose up
  ```
    - _Setup DB_
  ```bash
  cargo run consumer setup --env-file ../static/envs/.env.consumer.core
  ```
    - _Start_
  ```bash
  cargo run consumer start --env-file ../static/envs/.env.consumer.core
  # If files are modified, it recompiles in real time
  cargo watch -x "run consumer start --env-file ../static/envs/.env.consumer.core"
  ```
- **Dependencies**
  ```bash
  /rainbow-auth/ssi-auth/consumer/... # Root
  /rainbow-auth/ssi-auth/common/...   # Common auth modules dependencies
  /rainbow-db/auth_consumer/...       # DB dependencies
  /rainbow-common/...                 # Project-wide common dependencies
  ```
- **Swagger**  
  API specification available at: [http://127.0.0.1:1100/api/v1/auth/openapi](http://127.0.0.1:1100/api/v1/auth/openapi)

- **Postman**  
  Import tests from:
  ```bash
  /statics/specs/postman/auth/Consumer...
  ```

## Provider

Interacts with the rest of the "rainbow_provider" services, as defined by Carlos.  
It has its own database (Postgres) (currently uses the shared Provider DB, to be migrated to its own DB later).

### Deployment

- **Deployment Type**
    - _Monolith_
  ```bash
  cd rainbow-core
  ```
    - _Individual_
  ```bash
  cd rainbow-auth
  ```
- **Initialization**
    - _Database_
  ```bash
  cd deployment
  docker-compose up
  ```
    - _Setup DB_
  ```bash
  cargo run provider setup --env-file ../static/envs/.env.provider.core
  ```
    - _Start_
  ```bash
  cargo run provider start --env-file ../static/envs/.env.provider.core
  # If files are modified, it recompiles in real time
  cargo watch -x "run provider start --env-file ../static/envs/.env.provider.core"
  ```
- **Dependencies**
  ```bash
  /rainbow-auth/ssi-auth/provider/... # Root
  /rainbow-auth/ssi-auth/common/...   # Common auth modules dependencies
  /rainbow-db/auth_provider/...       # DB dependencies
  /rainbow-common/...                 # Project-wide common dependencies
  ```
- **Swagger**  
  API specification available at: [http://127.0.0.1:1200/api/v1/auth/openapi](http://127.0.0.1:1200/api/v1/auth/openapi)

- **Postman**  
  Import tests from:
  ```bash
  /statics/specs/postman/auth/Provider...
  ```

## Authority

This is a unique service.  
It has its own database (Postgres) and a GUI (not yet implemented).

### Deployment

- **Initialization**
    - _Database_
  ```bash
  cd deployment
  docker-compose up
  ```
    - _Setup DB_
  ```bash
  cd rainbow-authority
  cargo run setup
  ```
    - _Start_
  ```bash
  cargo run start
  # If files are modified, it recompiles in real time
  cargo watch -x "run start"
  ```
- **Dependencies**
  ```bash
  /rainbow-authority/...   # Root
  ```
- **Swagger**  
  API specification available at: [http://127.0.0.1:1500/api/v1/authority/openapi](http://127.0.0.1:1500/api/v1/authority/openapi)

- **Postman**  
  Import tests from:
  ```bash
  /statics/specs/postman/auth/Authority...
  ```

## Wallet

This is a service already created and defined by WaltID in [WaltId Docs](https://docs.walt.id/community-stack/home).  
Currently running locally with Docker Compose, though public APIs can also be used.  
It interacts with the previous three entities — for Consumer and Provider, only from the **Auth** module.  
There is only **one such service for all entities**, each authenticating with its own username and password.  
Interaction happens via HTTP requests to routes defined in environment variables.

> **Note:** Since **AUTH** and **AUTHORITY** currently run as a monolith and Wallet runs via docker-compose, some routes are hardcoded to replace "127.0.0.1" with "host.docker.internal". This must change in a microservices architecture.

- _Initialization_
  ```bash
  git clone https://github.com/walt-id/waltid-identity.git
  cd waltid-identity/docker-compose
  docker-compose up
  ```

## Tests

In the folder containing all Postman files, there is a collection of all tests which should always work.

## Summary

### Deployment

- **Deployment Type**
    - _Monolith_
  ```bash
  cd rainbow-core
  ```
    - _Individual_
  ```bash
  cd rainbow-auth
  ```
    - _Authority_
  ```bash
  cd rainbow-authority
  ```
- **Initialization**
    - _Database_
  ```bash
  cd deployment
  docker-compose up
  ```

- _Wallet_
  ```bash
  git clone https://github.com/walt-id/waltid-identity.git
  cd waltid-identity/docker-compose
  docker-compose up
  ```

- _Services_
  ```bash
  cargo run $entity_name $start_or_setup --env-file $env_file_path
  # If files are modified, it recompiles in real time
  # For authority these commands are different
  # The --env-file option is optional
  cargo watch -x "run $entity_name $start_or_setup --env-file $env_file_path"
  ```

- **Swagger**
    - Consumer API spec: [http://127.0.0.1:1100/api/v1/auth/openapi](http://127.0.0.1:1100/api/v1/auth/openapi)
    - Provider API spec: [http://127.0.0.1:1200/api/v1/auth/openapi](http://127.0.0.1:1200/api/v1/auth/openapi)
    - Authority API spec: [http://127.0.0.1:1500/api/v1/auth/openapi](http://127.0.0.1:1500/api/v1/auth/openapi)

- **Postman**  
  Collections located at:
  ```bash
  /statics/specs/postman/auth/...
  ```
