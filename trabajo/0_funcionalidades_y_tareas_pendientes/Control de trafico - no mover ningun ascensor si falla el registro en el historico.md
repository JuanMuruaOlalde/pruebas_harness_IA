# Control de tráfico: no mover ningún ascensor si falla el registro en el histórico

Detectado al revisar "Un control de trafico basico" (2026-09-29). Hace falta que Juan decida el comportamiento esperado antes de implementarlo.

## Situación actual

En `ControlDeTrafico` (`src/dominio/control_de_trafico.rs`), las operaciones primero cambian el `Edificio` y después registran el movimiento en el histórico:

- `atender_llamada_desde_la_planta`: si el registro falla, devuelve `ErrorDeControlDeTrafico::Historico`, pero el ascensor ya se ha movido a la planta de la llamada y está ocupado.
- `llevar_al_usuario_a_su_planta_de_destino`: si el registro falla, devuelve error, pero el ascensor ya está en la planta de destino y ha quedado libre.
- `optimizar_posicion_de_ascensores_libres`: si falla el registro de una reubicación, las anteriores ya se han hecho y registrado, y el ascensor de la que falla ya se ha movido. El resultado es una optimización a medias.

La especificación no contempla este caso. Lo que sí acordamos es que "si hay error, no se mueve, ocupa ni registra nada", pero pensando en errores del edificio (planta inexistente, ningún ascensor libre...).

## Propuesta

Decidir una de estas opciones:

1. Mantener lo actual y documentarlo: el edificio refleja la realidad física (el ascensor se ha movido) aunque el histórico no lo recoja.
2. Operación "todo o nada": si el registro falla, el edificio queda como estaba. Por ejemplo, calculando primero qué ascensor se mueve (una consulta en `Edificio`), registrando después y aplicando el movimiento solo si el registro va bien. Otra posibilidad es deshacer el movimiento si el registro falla.
3. En la optimización, decidir también si una reubicación fallida debe deshacer las anteriores o si basta con parar en la que falla.

Haría falta añadir tests con un histórico de prueba que falle al registrar.
