# PT 4300 Aumento de cobertura de tests

## Table of Contents

- [Overview](#overview)
- [Realization](#realization)
- [Tools](#tools)
- [Resumen](#resumen)

## Overview

Implementación de los tests unitarios que aseguren la cobertura sobre el código fuente.

## Realization

Para la realización de los test se han mockeado los elementos necesarios, para que dichas pruebas completen su función y se ha hecho una llamada a la función a probar.
En el caso de funciones asincronas nos ayudaremos de TOKIO. 

## Tools

Tokio en Rust es un runtime asíncrono diseñado para ejecutar código basado en el modelo async/await. Es uno de los componentes más importantes para desarrollar aplicaciones concurrentes y de alto rendimiento en Rust, especialmente en entornos de red.
Este runtime:
    - Proporciona un event loop eficiente para manejar tareas asíncronas.
    - Implementa un scheduler que ejecuta múltiples tareas en paralelo usando hilos.
    - Ofrece primitivas de I/O asíncrono (TCP, UDP, HTTP, etc.) sin bloquear el hilo principal.
    - Incluye timers, canales, y utilidades para sincronización.

Para la ejecución de los test usaremos la herramienta “Cargo”. Cargo es la herramienta oficial de gestión de proyectos y dependencias en Rust. Es similar a lo que npm es para JavaScript o pip para Python, pero mucho más integrado con el compilador de Rust.

Para usar esta herramienta y lanzar todos los test de proyecto se usará en la terminal el comando:

    ```bash
    cargo test
    ```

Si quisieramos lanzar solo los test de un paquete en concreto del proyecto usaremos:

    ```bash
    cargo test -p nombre_del_paquete
    (ej. cargo test -p rainbow-auth)
    ```

Y si queremos lanzar un test en concreto usaremos:

    ```bash
    cargo test -p nombre_del_paquete nombre_del_test
    (ej. cargo test -p rainbow-auth test_wallet_register_success)
    ```

Para generar un informe que muestre la cobertura, usaremos la herramienta “LLVM”. Esta es una herramienta para proyectos Rust que permite medir la cobertura de código usando “LLVM”. Está construida sobre Cargo y utiliza la infraestructura de LLVM para generar informes detallados.
“LLVM”, nos permite:

    - Ejecutar los test y calcular qué porcentaje del código fue cubierto.
    - Genera informes en varios formatos: texto, HTML, JSON.
    - Se integra fácilmente con cargo, por lo que no necesitas configuraciones complejas.

Las ventajas de usar esta herramienta son:

    - Usa LLVM source-based code coverage, que es más preciso que métodos tradicionales.
    - Compatible con unit tests, integration tests y doctests.

El informe generado se guadara en la ruta: rainbow\target\llvm-cov\html\

Para usar “LLVM”, tendremos previamente (sino lo hemos hecho ya), que instalar la herramienta, con el comando:

    ```bash
    cargo install cargo-llvm-cov
    ```

Para generar y abrir el informe completo del proyecto, ejecutaremos el comando:

    ```bash
    cargo llvm-cov --open
    ```

Si se quiere generar el informe de un paquete en concreto del proyecto usaremos:

    ```bash
    cargo llvm-cov --open -p nombre_del_paquete
    (ej. cargo llvm-cov --open -p rainbow_auth)
    ```

## Resumen

Para ejecutar todos los test del proyecto usaremos el comando:

    ```bash
    cargo test
    ```

Para obtener un informe de cobertura de los test usaremos:

    ```bash
    cargo llvm-cov --open
    ```
