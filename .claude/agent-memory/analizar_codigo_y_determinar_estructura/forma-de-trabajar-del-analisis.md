---
name: forma-de-trabajar-del-analisis
description: Detalles prácticos para ejecutar este agente en este repo: carpetas vetadas, nombre del archivo de salida, comprobaciones que se pueden lanzar sin modificar nada
metadata:
  type: project
---

- La única salida es `trabajo/analisis/vista_general_del_codigo-AAAAMMDDTHHMMSS-.md`. La plantilla lleva un guion antes de `.md` y se respeta tal cual. Además se actualiza esta memoria. No se toca el código.
- No hay que listar `trabajo/` de forma recursiva: con `ls trabajo/*` salen los nombres de los archivos de `0_funcionalidades_y_tareas_pendientes/`, que `CLAUDE.md` pide no leer. Los pendientes ya vienen resumidos en los `realizado_*.md` de `trabajo/3_historico/`.
- Estas comprobaciones funcionan sin pedir permisos y no modifican nada: `cargo fmt --check`, `cargo clippy --all-targets` y `cargo test` (compila en segundos, porque `target/` ya existe).
- El entorno no tiene pantalla, así que no se puede verificar la ventana egui con `cargo run`.

**Why:** son restricciones del encargo y de `CLAUDE.md`, y peculiaridades del entorno que se descubrieron en el primer análisis (2026-09-29).
**How to apply:** tenerlas en cuenta desde el principio de cada análisis. Ver [[contexto-y-fuentes-de-decisiones]].
