---
name: supuestos-de-dominio-aceptados
description: Supuestos del modelo del simulador de ascensores aceptados por Juan (plantas -2..7, arranque en planta 0, movimiento instantáneo) y términos provisionales fuera del glosario
metadata:
  type: project
---

Supuestos aceptados por Juan el 2026-09-29 al preparar "Estructura basica del edificio y ascensores":
1. Plantas de la -2 a la 7 (10 en total). La 0 es la planta de acceso principal (`Planta::DE_ACCESO_PRINCIPAL`).
2. Al crear el edificio, los 3 ascensores están en la planta 0.
3. El movimiento es instantáneo: no se modelan estados libre/en movimiento.

"Identificador de ascensor" (`IdentificadorDeAscensor`) NO está en `src/glosario_de_dominio.md`. Es un nombre provisional, pendiente de que Juan lo acuerde.

**Why:** el glosario obliga a acordar con Juan cualquier término nuevo antes de usarlo. Los supuestos son decisiones de alcance, no limitaciones técnicas.
**How to apply:** al refactorizar, no introducir estados de movimiento ni tiempos (eso sería funcionalidad nueva). No renombrar ni ampliar términos de dominio sin acuerdo; si hace falta, dejarlo como pendiente en `trabajo/0_funcionalidades_y_tareas_pendientes/`. Comprobar primero si el glosario ya incorpora el término, porque este recuerdo puede estar desfasado.
