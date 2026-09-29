use std::cell::Cell;
use std::rc::Rc;

use crate::dominio::fecha_y_hora::FechaYHora;
use crate::dominio::reloj::Reloj;

/// Reloj cuya hora se fija a mano. Sus copias comparten la misma hora, de modo que
/// quien lo entregó a otro componente puede seguir cambiándola.
#[derive(Clone)]
pub struct RelojSimulado {
    hora_actual: Rc<Cell<FechaYHora>>,
}

impl RelojSimulado {
    pub fn con_la_hora(fecha_y_hora: FechaYHora) -> Self {
        Self {
            hora_actual: Rc::new(Cell::new(fecha_y_hora)),
        }
    }

    pub fn fijar_la_hora(&self, fecha_y_hora: FechaYHora) {
        self.hora_actual.set(fecha_y_hora);
    }
}

impl Reloj for RelojSimulado {
    fn ahora(&self) -> FechaYHora {
        self.hora_actual.get()
    }
}
