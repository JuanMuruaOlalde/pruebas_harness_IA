---
name: rol-en-el-bucle-automatico
description: Cómo encaja este agente (revisión y refactorización) en el bucle automático de programación y qué entregar
metadata:
  type: project
---

En el bucle automático, el orquestador me invoca como paso de revisión. Antes, el subagente `programar_codigo` implementa la funcionalidad y el orquestador pasa `cargo fmt`. El código llega sin commitear, y la especificación está en `trabajo/2_en_curso/<funcionalidad>.md`.

**Why:** el orquestador espera un informe final con lo revisado, los cambios, los archivos tocados, los resultados de `cargo test` y `cargo clippy`, y las observaciones. Los problemas bloqueantes van en `trabajo/2_en_curso/PROBLEMA_*.md`.
**How to apply:** comprobar primero el estado de partida (`cargo test`, `cargo clippy --all-targets`, `cargo fmt --check`), refactorizar sin tocar el comportamiento y volver a verificarlo todo. No hacer commit: se encarga el orquestador. Las propuestas que cambian el comportamiento o la API pública (por ejemplo, implementar `Display` en los errores) van como pendiente en `trabajo/0_funcionalidades_y_tareas_pendientes/`, no en el código. Ver [[supuestos-de-dominio-aceptados]].
