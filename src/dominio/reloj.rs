use super::fecha_y_hora::FechaYHora;

/// Puerto que dice qué fecha y hora es en este momento.
pub trait Reloj {
    fn ahora(&self) -> FechaYHora;
}
