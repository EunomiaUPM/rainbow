# Sistema de Identidad del Rainbow

## Table of Contents
- [Overview](#overview)
- [Consumer](#consumer)
- [Provider](#provider)
- [Authority](#authority)
- [Wallet](#wallet)
- [Tests](#tests)
- [Resumen](#resumen)


## Overview

Este documento muestra la documentacion para comprender el sistema de autenticación desarrollado para el Rainbow

## Consumer

Interactua con el resto de servicios de "rainbow_consumer", estos son los definidos por Carlos. <br>
Tiene una DB propia (postgress) (de momento usa la comun del Consumer, trasladar a DB propia en individual) <br>

### Despliegue

- **Tipo de despligue**
    - <u>_Monolito_</u>
  ```bash 
  cd rainbow-core
  ```
    - <u>_Individual_</u>
  ```bash 
  cd rainbow-auth
  ```
- **Inicializacion**
    - <u>_Base de Datos_</u>
  ```bash
  cd deployment
  docker-compose up
  ```
    - <u>_Setup BD_</u>
  ```bash
  cargo run consumer setup --env-file ../static/envs/.env.consumer.core  
  ```
    - <u>_Start_</u>
  ```bash
  cargo run consumer start --env-file ../static/envs/.env.consumer.core
  # En caso de ser modificados archivos, este recompila en tiempo real 
  cargo watch -x "run consumer start --env-file ../static/envs/.env.consumer.core" 
  ```
- **Dependencias**
  ```bash
  /rainbow-auth/ssi-auth/consumer/... # Raiz
  /rainbow-auth/ssi-auth/common/...   # Dependencias comunes entre modulos auth
  /rainbow-db/auth_consumer/...       # Dependencias DB
  /rainbow-common/...                 # Dependencias comunes de todo el proyecto
  ```
- **Swagger** <br>
  Especificacion de la API
  en: [http://127.0.0.1:1100/api/v1/auth/openapi](http://127.0.0.1:1100/api/v1/auth/openapi) <br> <br>

- **Postman** <br>
  Importar los tests cuya ruta es:
  ```bash
  /statics/specs/postman/auth/Consumer...
  ```

## Provider

Interactua con el resto de servicios de "rainbow_provider", estos son los definidos por Carlos. <br>
Tiene una DB propia (postgress) (de momento usa la comun del Provider, trasladar a DB propia en individual)

### Despliegue

- **Tipo de despligue**
    - <u>_Monolito_</u>
  ```bash 
  cd rainbow-core
  ```
    - <u>_Individual_</u>
  ```bash 
  cd rainbow-auth
  ```
- **Inicializacion**
    - <u>_Base de Datos_</u>
  ```bash
  cd deployment
  docker-compose up
  ```
    - <u>_Setup BD_</u>
  ```bash
  cargo run provider setup --env-file ../static/envs/.env.provider.core  
  ```
    - <u>_Start_</u>
  ```bash
  cargo run provider start --env-file ../static/envs/.env.provider.core
  # En caso de ser modificados archivos, este recompila en tiempo real 
  cargo watch -x "run provider start --env-file ../static/envs/.env.provider.core" 
  ```
- **Dependencias**
  ```bash
  /rainbow-auth/ssi-auth/provider/... # Raiz
  /rainbow-auth/ssi-auth/common/...   # Dependencias comunes entre modulos auth
  /rainbow-db/auth_provider/...       # Dependencias DB
  /rainbow-common/...                 # Dependencias comunes de todo el proyecto
  ```
- **Swagger** <br>
  Especificacion de la API
  en: [http://127.0.0.1:1200/api/v1/auth/openapi](http://127.0.0.1:1200/api/v1/auth/openapi) <br> <br>

- **Postman** <br>
  Importar los tests cuya ruta es:
  ```bash
  /statics/specs/postman/auth/Provider...
  ```

## Authority

Es un servicio único. <br>
Tiene una DB propia (postgress) y una GUI (aun no implementada)

### Despliegue

- **Inicializacion**
    - <u>_Base de Datos_</u>
  ```bash
  cd deployment
  docker-compose up
  ```
    - <u>_Setup BD_</u>
  ```bash
  cd rainbow-authority
  cargo run setup  
  ```
    - <u>_Start_</u>
  ```bash
  cargo run start 
  # En caso de ser modificados archivos, este recompila en tiempo real 
  cargo watch -x "run start" 
  ```
- **Dependencias**
  ```bash
  /rainbow-authority/...   # Raiz
  ```
- **Swagger** <br>
  Especificacion de la API
  en: [http://127.0.0.1:1500/api/v1/authority/openapi](http://127.0.0.1:1500/api/v1/authority/openapi) <br> <br>

- **Postman** <br>
  Importar los tests cuya ruta es:
  ```bash
  /statics/specs/postman/auth/Authority...
  ```

## Wallet

Es un servicio ya creado y definido por waltid en [WaltId Docs](https://docs.walt.id/community-stack/home). <br>
Ahora mismo está levantado con Docker-Compose de manera local, aunque se pueden utilizar sus APIs públicas en vez de las
locales. Estas se encuentran en la documentación anterior.
Se relaciona, con las 3 entidades anteriores, en el caso de Consumer y Provider solo desde el módulo de **Auth**. <br>
Solo existe **un servicio asi para todas** las entidades, cada una luego se autentica con su usuario y contraseña. La
manera de interactuar es mediante peticiones http a las rutas definidas en variables de entorno.

> **Nota:** Como actualmente el modulo **AUTH** y **AUTHORITY** se levantan como monolito, y la wallet mediante
> docker-compose. Hay ciertas rutas hardcodeadas dentro del proyecto para sustituir "127.0.0.1" por "
> host.docker.internal", en una arquitectura de microservicios eso tiene que cambiar.

- <u>_Inicializacion_</u>
  ```bash
  git clone https://github.com/walt-id/waltid-identity.git
  cd waltid-identity/docker-compose
  docker-compose up
  ```

## Tests

En la carpeta que contiene todos los postman de esta parte, hay una coleccion de todos los test, que deben funcionar
siempre.

## Resumen

### Despliegue

- **Tipo de despligue**
    - <u>_Monolito_</u>
  ```bash 
  cd rainbow-core 
  ```
    - <u>_Individual_</u>
  ```bash 
  cd rainbow-auth
  ```
    - <u>_Authority_</u>
  ```bash 
  cd rainbow-authority
  ```
- **Inicializacion**
    - <u>_Base de Datos_</u>
  ```bash
  cd deployment
  docker-compose up
  ```

- <u>_Wallet_</u>
  ```bash
  git clone https://github.com/walt-id/waltid-identity.git
  cd waltid-identity/docker-compose
  docker-compose up
  ```

- <u>_Servicios_</u>
  ```bash 
  cargo run $nombre_entidad $start_o_setup --env-file $ruta_archivo_env 
  # En caso de ser modificados archivos, este recompila en tiempo real
  # Para la autoridad no son estos 
  # La opcion --env-file es opcional, se puede omitir junto con la ruta, en ese caso se emplean valores por defecto 
  cargo watch -x "run $nombre_entidad $start_o_setup --env-file $ruta_archivo_env
  ```

- **Swagger** <br>
    - Consumer API spec: [http://127.0.0.1:1100/api/v1/auth/openapi](http://127.0.0.1:1100/api/v1/auth/openapi)
    - Provider API spec: [http://127.0.0.1:1200/api/v1/auth/openapi](http://127.0.0.1:1200/api/v1/auth/openapi)
    - Authority API spec: [http://127.0.0.1:1500/api/v1/auth/openapi](http://127.0.0.1:1500/api/v1/auth/openapi)

- **Postman** <br>
  Las colleciones se encuentran en:
  ```bash
  /statics/specs/postman/auth/...
  ```
