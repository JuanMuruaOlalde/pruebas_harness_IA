use std::time::{SystemTime, UNIX_EPOCH};

use crate::dominio::fecha_y_hora::FechaYHora;
use crate::dominio::reloj::Reloj;

/// Reloj real del ordenador, en hora UTC.
pub struct RelojDelSistema;

impl Reloj for RelojDelSistema {
    fn ahora(&self) -> FechaYHora {
        let segundos_desde_1970 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(tiempo_desde_1970) => tiempo_desde_1970.as_secs() as i64,
            // La hora del sistema es anterior a 1970: el error indica cuánto falta hasta 1970.
            Err(tiempo_hasta_1970) => -(tiempo_hasta_1970.duration().as_secs() as i64),
        };
        FechaYHora::desde_segundos_desde_1970(segundos_desde_1970)
    }
}
