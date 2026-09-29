---
name: linea-base-analisis-2026-09-29
description: Hallazgos principales del primer análisis (commit 0b8783a, 2026-09-29), como línea base para medir el avance en análisis posteriores
metadata:
  type: project
---

Es una foto fija: hay que comprobarla contra el código actual antes de repetir nada. El informe completo está en `trabajo/analisis/vista_general_del_codigo-20260929T214603-.md`.

Estado de partida: pasan los 100 tests, clippy no da avisos y el formato está limpio. Los hallazgos principales, por prioridad sugerida:
1. Las operaciones no son atómicas: `ControlDeTrafico` mueve el ascensor antes de registrar el movimiento y, si el registro falla, el estado queda incoherente. Hay un pendiente abierto.
2. La decisión de qué ascensor atiende una llamada está en `Edificio`, aunque el glosario se la asigna al control de tráfico. La lógica del "más cercano" está duplicada en `Edificio` y en `ControlDeTrafico`.
3. `Edificio::mover_ascensor_a_la_planta` es público y permite saltarse las invariantes. El estado libre/ocupado es un `bool`. La configuración (plantas -2..7, 3 ascensores) son constantes fijas.
4. `ControlDeTrafico` concentra tres responsabilidades; la optimización se puede extraer como estrategia.
5. No hay puertos de entrada ni capa de aplicación: la interfaz depende del struct concreto y de cómo se anidan sus errores.
6. La *llamada* no está modelada, así que las llamadas no atendidas no quedan registradas.

**Why:** en el siguiente análisis permite decir qué se ha corregido y qué sigue igual, sin empezar de cero.
**How to apply:** verificar cada punto en el código antes de citarlo como vigente, y actualizar o sustituir esta memoria tras cada análisis nuevo.
