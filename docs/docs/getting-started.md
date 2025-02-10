---
sidebar_position: 2
title: Getting Started
---

# Getting started

### Plain old docker

To get started simply, we recommend that you work with the docker container.

```bash
docker pull caparicioesd/rainbow
```

And then you can initialize it with:

```bash
docker run caparicioesd/rainbow:latest provider start
```

If you want to see the different rainbow options:

```bash
docker run caparicioesd/rainbow:latest provider -h  

# Options:
  # --host-url <HOST_URL>                  
  # --host-port <HOST_PORT>                
  # --db-type <DB_TYPE>                    
  # --db-url <DB_URL>                      
  # --db-port <DB_PORT>                    
  # --db-user <DB_USER>                    
  # --db-password <DB_PASSWORD>  
```

The database that Rainbow works with is Postgres. To initialize the database migrations:

```bash
docker run caparicioesd/rainbow:latest provider \
  --db-port <DB_PORT> \
  --db-user <DB_USER> \                    
  --db-password <DB_PASSWORD> \           
  --db-database <DB_DATABASE> \
  setup
```

### Docker compose

In case you want to integrate Rainbow in a more automated way, with docker-compose, you can start server instances,
databases, migrations. In `/deployments/docker-compose.testing.yaml` you have a good example of this.
