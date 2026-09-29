---
name: contexto-y-fuentes-de-decisiones
description: El simulador de ascensores es un vehículo para experimentar con el harness agéntico; dónde están registradas las decisiones de alcance aceptadas por Juan y cómo tratarlas en un análisis
metadata:
  type: project
---

El repositorio tiene como fin principal experimentar con agentes (el harness de `.claude/`: agentes, la skill del bucle automático y los permisos). El simulador de ascensores en Rust es el vehículo. El `README.md` habla del harness, no del simulador.

Las decisiones de alcance que Juan ha aceptado no están en el código. Están en:
- `trabajo/3_historico/realizado_*.md`: registros del bucle automático, con los supuestos, los problemas y los pendientes abiertos de cada vuelta;
- la memoria del revisor, `.claude/agent-memory/revisar_codigo_y_refactorizar/supuestos-de-dominio-aceptados.md` (ver [[supuestos-de-dominio-aceptados]]).

**Why:** así se distingue lo que es una decisión aceptada (por ejemplo, que la interfaz solo tenga el botón de llamada, la hora en UTC o el movimiento instantáneo) de lo que es un defecto.
**How to apply:** antes de analizar, leer esos registros. Las decisiones aceptadas se citan como limitaciones, no como fallos. Cualquier sugerencia que añada términos de dominio (nuevos tipos o estados) debe indicar que hay que acordarla antes con Juan, porque el glosario lo exige.
