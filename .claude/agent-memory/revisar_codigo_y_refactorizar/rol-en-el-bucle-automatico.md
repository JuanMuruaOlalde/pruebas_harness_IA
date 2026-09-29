---
name: rol-en-el-bucle-automatico
description: Cómo encaja este agente (revisión y refactorización) en el bucle automático de programación y qué entregar
metadata:
  type: project
---

En el bucle automático, el orquestador me invoca como paso de revisión. Antes, el subagente `programar_codigo` implementa la funcionalidad y el orquestador pasa `cargo fmt`. El código llega sin commitear, y la especificación está en `trabajo/2_en_curso/<funcionalidad>.md`.

**Why:** el orquestador espera un informe final con lo revisado, los cambios, los archivos tocados, los resultados de `cargo test` y `cargo clippy`, y las observaciones. Los problemas bloqueantes van en `trabajo/2_en_curso/PROBLEMA_*.md`.
**How to apply:** comprobar primero el estado de partida (`cargo test`, `cargo clippy --all-targets`, `cargo fmt --check`), refactorizar sin tocar el comportamiento y volver a verificarlo todo. No hacer commit: se encarga el orquestador. Las propuestas que cambian el comportamiento o la API pública (por ejemplo, implementar `Display` en los errores) van como pendiente en `trabajo/0_funcionalidades_y_tareas_pendientes/`, no en el código. Ver [[supuestos-de-dominio-aceptados]].

El clasificador del modo automático a veces no da veredicto sobre Bash durante varios intentos seguidos, y al décimo se corta el turno. Cuando pasa, conviene dejar escrito antes `trabajo/2_en_curso/PROBLEMA_al_refactorizar.md` con los cambios pendientes de verificar, y borrarlo si luego la verificación pasa. Los comandos cortos, de una sola orden, suelen pasar antes. Si la verificación ya se hizo, no hay que insistir con comprobaciones redundantes.

Al crear un pendiente no hay que listar ni leer `trabajo/0_funcionalidades_y_tareas_pendientes/`, porque CLAUDE.md lo prohíbe. Basta con crear el archivo con Write y un nombre descriptivo: Write falla si ya existe un archivo que no se ha leído, así que no se sobrescribe nada. Cada pendiente puede entrar en la cola del bucle, así que conviene indicar si hace falta que Juan decida antes.
