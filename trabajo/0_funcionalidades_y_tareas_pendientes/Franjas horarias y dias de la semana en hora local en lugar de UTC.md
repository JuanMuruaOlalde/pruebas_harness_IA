# Franjas horarias y días de la semana en hora local, en lugar de UTC

Detectado al revisar "Un control de trafico basico" (2026-09-29). Hace falta que Juan decida si se cambia.

## Situación actual

`RelojDelSistema` (`src/adaptadores/reloj_del_sistema.rs`) da la fecha y hora en UTC. `FechaYHora` no tiene zona horaria. Por eso el día de la semana y la franja horaria que usa la optimización de la posición de los ascensores libres se calculan en UTC.

Consecuencias en un edificio con horario peninsular español (UTC+1 en invierno y UTC+2 en verano):

- La franja horaria de una misma hora local cambia con el cambio de horario. Durante los 30 días posteriores a cada cambio, el "último mes" mezcla llamadas de dos franjas locales distintas.
- El día de la semana cambia a las 00:00 UTC, es decir, a la 01:00 o las 02:00 hora local. Las llamadas de primera hora de la madrugada cuentan como del día anterior.
- El archivo del histórico muestra horas UTC, que no coinciden con las que vive el usuario.

## Propuesta

Decidir si el simulador trabaja en hora local, sea la del sistema o una zona horaria configurable del edificio. Si es así:

- Añadir un reloj (o ampliar `RelojDelSistema`) que dé la hora local, con o sin dependencias externas (por ejemplo, el crate `chrono`).
- Decidir cómo se guarda la hora en el histórico permanente: local, o UTC con la conversión a local para calcular día de la semana y franja.
