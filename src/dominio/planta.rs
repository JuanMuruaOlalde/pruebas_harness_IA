/// Número de planta: 0 es la planta principal de acceso, positivos por encima y negativos por debajo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Planta(i8);

impl Planta {
    /// Planta principal por donde se accede al edificio.
    pub const DE_ACCESO_PRINCIPAL: Planta = Planta(0);

    pub fn con_numero(numero: i8) -> Self {
        Self(numero)
    }

    pub fn numero(self) -> i8 {
        self.0
    }
}
