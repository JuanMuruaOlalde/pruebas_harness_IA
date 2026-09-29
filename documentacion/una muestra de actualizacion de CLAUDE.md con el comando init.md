Se ha lanzado el comando `/init`

# El resumen del agente principal que gestiona la sesión

Ya existía un `CLAUDE.md`, así que no lo he rehecho: he rellenado las secciones que estaban vacías y he añadido algunas líneas. No he cambiado nada de lo que ya estaba escrito. Son 40 líneas nuevas en [CLAUDE.md](CLAUDE.md), sin commitear.

- **Propósito:** una línea que explica que el harness (`.claude/` y `trabajo/`) también forma parte del experimento, como dice el `README.md`, y no solo el código.
- **Comandos habituales:**
  - `cargo clippy --all-targets`, para revisar también los tests.
  - Cómo ejecutar solo algunos tests. He comprobado que `cargo test dominio::edificio::` ejecuta los 44 tests de ese módulo.
  - Qué hace `cargo run`: abre la ventana egui, escribe en `historico_de_movimientos.txt` y no arranca si ese archivo tiene alguna línea mal formada.
- **Mapa de la arquitectura actual:**
  - `main.rs` es donde se ensambla todo.
  - El dominio no depende de nada externo y tiene los puertos `Reloj` y `HistoricoDeMovimientos`.
  - `ControlDeTrafico` es la puerta de entrada al dominio.
  - Límites fijos del `Edificio`: plantas de la -2 a la 7 y 3 ascensores.
  - Adaptadores de entrada y de salida, incluido el reparto entre `PanelDelSimulador` (se prueba sin egui) y `VentanaDelSimulador`.
  - Dónde están los tests y qué dobles de test usan.
- **Flujo de trabajo:** para qué sirve cada carpeta de `trabajo/`, los seis pasos de `/programacion_en_bucle_automatico`, los dos agentes de análisis que quedan fuera del bucle y las restricciones de `git` en `settings.json`.
- **Estilo:** todo va en español, con identificadores sin tildes ni eñes (`anio`, `Miercoles`), y los términos del dominio salen del glosario.

No hay reglas de Cursor ni de Copilot, ni configuración de Codex o Gemini que importar.

También he actualizado mi memoria con los pendientes de la revisión del harness. Muchos ya están resueltos. Siguen abiertos:
- el deny de lectura sobre `documentacion/`;
- el hook que compruebe las directrices antes de `git commit`;
- algunas erratas en `programar_codigo.md`;
- los dos detalles de [analizar_codigo_y_determinar_estructura.md:13](.claude/agents/analizar_codigo_y_determinar_estructura.md#L13): la comilla invertida suelta y el guion antes de `.md`.

# CLAUDE.md antes de /init

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

Allá donde sea conveniente:
- Implementar una arquitectura Hexagonal (Ports&Adapters).
- Seguir los principios SOLID.
- Seguir la metodología DDD (Domain Driven Design). El glosario de dominio está en el archivo `src/glosario_de_dominio.md`.

### Mapa de la arquitectura actual


## Flujo de trabajo




## Límites a respetar

No modificar la carpeta `trabajo/directrices/`. Sí leer su contenido.

No modificar la carpeta `documentacion/`. No leer su contenido; excepto cuando sea necesario para hacer un commit.

No leer el contenido de la carpeta `trabajo/0_funcionalidades_y_tareas_pendientes`. No modificar su contenido; excepto para añadir nuevos archivos.

Ignorar totalmente la carpeta `zz - trozos de codigo descartados - guardados por si acaso`. No leer ni modifcar su contenido.


## Estilo al escribir código

Dedicar atención a la nomenclatura. Poner nombres descriptivos, aunque resulten largos. El código se ha de poder leer con facilidad y quedando claro qué representa cada variable, función, estructura,...



# CLAUDE.md después de /init

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Propósito del proyecto

Es una prueba para experimentar y practicar programación en un entorno agéntico.

Vamos a implementar un pequeño simulador de ascensores.

El harness (`.claude/agents/`, `.claude/skills/`, `.claude/settings.json` y la carpeta `trabajo/`) también es objeto del experimento, no solo el código: el `README.md` los presenta como muestra de cómo explicitar directrices para agentes IA.


## Comandos habituales

```bash

cargo fmt
cargo clippy
cargo build
cargo run
cargo test

```

- `cargo clippy --all-targets`: revisa también los tests. El bucle automático exige que no haya ningún warning.
- `cargo test <parte_del_nombre>`: ejecuta solo los tests cuyo nombre la contiene. Por ejemplo, `cargo test dominio::edificio::` ejecuta solo los tests de ese módulo.
- `cargo run`: abre una ventana egui y añade cada movimiento a `historico_de_movimientos.txt`, en el directorio desde el que se lanza (está en `.gitignore`). Si ese archivo tiene alguna línea mal formada, el programa no arranca.


## Arquitectura

Allá donde sea conveniente:
- Implementar una arquitectura Hexagonal (Ports&Adapters).
- Seguir los principios SOLID.
- Seguir la metodología DDD (Domain Driven Design). El glosario de dominio está en el archivo `src/glosario_de_dominio.md`.

### Mapa de la arquitectura actual

Un crate `pruebas_harness` con una biblioteca (`src/lib.rs`) y un binario (`src/main.rs`). `main.rs` es donde se ensambla todo: crea `ControlDeTrafico` con `Edificio`, `RelojDelSistema` y `HistoricoEnArchivo`, y abre la ventana.

- `src/dominio/`: no depende de los adaptadores ni de ningún crate externo. Tiene dos puertos de salida (traits): `Reloj` y `HistoricoDeMovimientos`.
  - `ControlDeTrafico` es la puerta de entrada al dominio. Contiene el `Edificio` y recibe el reloj y el histórico como `Box<dyn …>`. Atiende llamadas, lleva al usuario a su destino, registra cada movimiento y optimiza la posición de los ascensores libres.
  - `Edificio` fija en constantes las plantas (de la -2 a la 7) y el número de ascensores (3). Los movimientos son instantáneos.
  - `FechaYHora` implementa a mano el calendario gregoriano (algoritmo de H. Hinnant), sin zona horaria.
  - Cada módulo tiene su propio enum de error. `ErrorDeControlDeTrafico` envuelve los de edificio e histórico mediante `From`.
- `src/adaptadores/`:
  - De salida: `historico_en_archivo` (una línea de texto por movimiento: `ascensor;origen;destino;AAAA-MM-DD HH:MM:SS;motivo`), `historico_en_memoria`, `reloj_del_sistema` (en UTC) y `reloj_simulado`.
  - De entrada: `interfaz_grafica/`. `PanelDelSimulador` contiene el estado y la lógica de la interfaz y se prueba sin egui. `VentanaDelSimulador` solo dibuja con egui y pasa las pulsaciones al panel. `eframe` es la única dependencia externa.
- Tests: no hay carpeta `tests/`. Cada archivo tiene su propio módulo `#[cfg(test)] mod tests`. Los tests del dominio usan `HistoricoEnMemoria` y `RelojSimulado` como dobles de test.
- Los análisis de la arquitectura, con los problemas conocidos, se guardan en `trabajo/analisis/`.


## Flujo de trabajo

Las funcionalidades avanzan por las carpetas de `trabajo/`:

- `0_funcionalidades_y_tareas_pendientes/`: ideas y tareas pendientes. Los agentes añaden aquí lo que detectan fuera de su encargo.
- `1_listo_para_implementar/`: la cola de funcionalidades para el bucle automático.
- `2_en_curso/`: la funcionalidad en curso y su `lista_de_tests.txt`, con una signatura de test por línea. Si aparece un `PROBLEMA_*.md` o un `FALLAN_VALIDACIONES.md`, el bucle se detiene.
- `3_historico/`: las funcionalidades terminadas y los informes `realizado_AAAAMMDDTHHMMSS.md` de cada ejecución del bucle.
- `directrices/`: las reglas que `validar_codigo_y_commitearlo` comprueba antes de cada commit: nada de secretos, archivos de más de 10 MB ni ejecutables binarios.

La skill `/programacion_en_bucle_automatico` solo se lanza a mano. La coordina la conversación principal, porque los subagentes no pueden lanzar otros subagentes (`CLAUDE_CODE_MAX_SUBAGENT_SPAWN_DEPTH=1`). Pasos de cada vuelta:

1. `evaluar_y_preparar_trabajo`
2. El usuario confirma la funcionalidad elegida y su lista de tests.
3. `programar_codigo`: TDD, un test por cada línea de la lista.
4. `cargo fmt`
5. `revisar_codigo_y_refactorizar`
6. `validar_codigo_y_commitearlo`: si todo está bien, pasa `2_en_curso/` a `3_historico/` y hace el commit.

Fuera del bucle hay dos agentes de análisis, `analizar_codigo_y_determinar_estructura` y `buscar_costuras_y_extraer_interfaces`, que escriben sus informes en `trabajo/analisis/`.

En `.claude/settings.json`, `git commit` pide confirmación y `git push` está prohibido.


## Límites a respetar

No modificar la carpeta `trabajo/directrices/`. Sí leer su contenido.

No modificar la carpeta `documentacion/`. No leer su contenido; excepto cuando sea necesario para hacer un commit.

No leer el contenido de la carpeta `trabajo/0_funcionalidades_y_tareas_pendientes`. No modificar su contenido; excepto para añadir nuevos archivos.

Ignorar totalmente la carpeta `zz - trozos de codigo descartados - guardados por si acaso`. No leer ni modifcar su contenido.


## Estilo al escribir código

Dedicar atención a la nomenclatura. Poner nombres descriptivos, aunque resulten largos. El código se ha de poder leer con facilidad y quedando claro qué representa cada variable, función, estructura,...

Todo va en español: identificadores, nombres de tests, comentarios y mensajes. En los identificadores se escribe sin tildes ni eñes (`anio`, `Miercoles`). Los términos del dominio son los del glosario; para usar uno que no esté, hay que acordarlo antes.



