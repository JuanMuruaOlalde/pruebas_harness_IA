---
name: supuestos-de-dominio-aceptados
description: Supuestos del simulador de ascensores aceptados por Juan (plantas -2..7, arranque en 0, movimiento instantáneo, libre/ocupado, reglas del control de tráfico) y términos provisionales fuera del glosario
metadata:
  type: project
---

Supuestos aceptados por Juan el 2026-09-29 al preparar "Estructura basica del edificio y ascensores":
1. Plantas de la -2 a la 7 (10 en total). La 0 es la planta de acceso principal (`Planta::DE_ACCESO_PRINCIPAL`).
2. Al crear el edificio, los 3 ascensores están libres en la planta 0.
3. El movimiento es instantáneo: no se modela el tiempo ni el estado "en movimiento".

Supuestos añadidos el 2026-09-29 con "Un control de trafico basico":
- Cada ascensor está libre u ocupado. A una llamada acude el libre más cercano (a igual distancia, el de identificador más bajo), que queda ocupado. "Llevar al usuario a su planta de destino" lo deja libre.
- Cualquier error del edificio implica que no se mueve, ocupa ni registra nada. El caso de fallo al registrar en el histórico no está decidido: hay un pendiente abierto.
- Histórico permanente = archivo en el que nunca se borran movimientos. "Último mes" = 30 días con ambos límites incluidos; franjas de una hora. La hora es UTC; hay un pendiente abierto sobre usar la hora local.

Términos que NO están en `src/glosario_de_dominio.md` y son provisionales: identificador de ascensor, control de tráfico, libre, ocupado, llamada, botón de llamada, histórico, movimiento, planta de destino, motivo del movimiento, reubicación, fecha y hora, franja horaria, reloj.

**Why:** el glosario obliga a acordar con Juan cualquier término nuevo antes de usarlo. Los supuestos son decisiones de alcance, no limitaciones técnicas.
**How to apply:** al refactorizar, no introducir tiempos ni estados nuevos (sería funcionalidad nueva). No renombrar ni ampliar términos de dominio sin acuerdo; si hace falta, dejarlo como pendiente en `trabajo/0_funcionalidades_y_tareas_pendientes/`. Comprobar primero si el glosario ya incorpora el término, porque este recuerdo puede estar desfasado.
