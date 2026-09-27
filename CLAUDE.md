# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Propósito del proyecto

Es una prueba para experimentar y practicar programación en un entorno agéntico.

Vamos a implementar un pequeño simulador de ascensores.


## Comandos habituales

```bash

cargo fmt
cargo clippy
cargo build
cargo run
cargo test

```


## Arquitectura

Allá donde sea posible:
- Implementar una arquitectura Hexagonal (Ports&Adapters).
- Seguir los principios SOLID.
- Seguir la metodología DDD (Domain Driven Design). El glosario de dominio está en el archivo `src/glosario_de_dominio.md`.


## Flujo de trabajo




## Límites a respetar

No modificar la carpeta `directrices/`. Sí leer su contenido.

No modificar la carpeta `documentacion/`. No leer su contenido; excepto cuando sea necesario para hacer un commit.

No leer el contenido de la carpeta `trabajo/0_funcionalidades_y_tareas_pendientes`. No modificar su contenido; excepto para añadir nuevos archivos.

Ignorar totalmente la carpeta `zz - trozos de codigo descartados - guardados por si acaso`. No leer ni modifcar su contenido.


## Estilo al escribir código

Dedicar atención a la nomenclatura. Poner nombres descriptivos, aunque resulten largos. El código se ha de poder leer con facilidad y quedando claro qué representa cada variable, función, estructura,...



