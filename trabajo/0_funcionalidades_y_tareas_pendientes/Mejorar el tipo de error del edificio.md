# Mejorar el tipo de error del edificio

Surgido en la revisión de "Estructura basica del edificio y ascensores" (2026-09-29).

`ErrorDeEdificio` (en `src/dominio/edificio.rs`) es ahora un enum simple sin más. Propuestas:

1. **Implementar `std::fmt::Display` y `std::error::Error`** para `ErrorDeEdificio`. Es lo idiomático en Rust y hará falta cuando haya una interfaz de usuario que muestre los errores (o para propagarlos con `?` hacia `Box<dyn Error>`). Supone decidir el texto de cada mensaje.

2. **Decidir qué error prevalece cuando fallan ascensor y planta a la vez.** Hoy, `mover_ascensor_a_la_planta` con un ascensor inexistente y una planta inexistente devuelve `AscensorInexistente`, porque comprueba primero el ascensor. Ningún test fija este comportamiento ni la especificación lo menciona. Habría que confirmarlo (y añadir un test) o cambiarlo; por ejemplo, incluir en el error qué ascensor o qué planta no existen.
