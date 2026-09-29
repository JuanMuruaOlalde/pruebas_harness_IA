use super::edificio::IdentificadorDeAscensor;
use super::fecha_y_hora::FechaYHora;
use super::planta::Planta;

/// Por qué se ha movido un ascensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotivoDelMovimiento {
    AtenderUnaLlamada,
    LlevarAlUsuarioASuDestino,
    ReubicarUnAscensorLibre,
}

/// Desplazamiento de un ascensor de una planta a otra, con el momento en que se produjo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Movimiento {
    pub ascensor: IdentificadorDeAscensor,
    pub planta_de_origen: Planta,
    pub planta_de_destino: Planta,
    pub fecha_y_hora: FechaYHora,
    pub motivo: MotivoDelMovimiento,
}
