/// Número de planta: 0 es la planta principal de acceso, positivos por encima y negativos por debajo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    /// Número de plantas que separan a esta planta de otra, sin importar el sentido.
    pub fn distancia_hasta(self, otra_planta: Planta) -> u8 {
        self.0.abs_diff(otra_planta.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planta(numero: i8) -> Planta {
        Planta::con_numero(numero)
    }

    #[test]
    fn la_distancia_entre_dos_plantas_es_el_numero_de_plantas_que_las_separan() {
        assert_eq!(planta(2).distancia_hasta(planta(5)), 3);
        assert_eq!(planta(3).distancia_hasta(planta(3)), 0);
    }

    #[test]
    fn la_distancia_entre_dos_plantas_es_la_misma_en_ambos_sentidos() {
        assert_eq!(
            planta(1).distancia_hasta(planta(6)),
            planta(6).distancia_hasta(planta(1))
        );
    }

    #[test]
    fn la_distancia_entre_una_planta_negativa_y_una_positiva_cuenta_las_plantas_intermedias() {
        assert_eq!(planta(-2).distancia_hasta(planta(7)), 9);
        assert_eq!(planta(-1).distancia_hasta(planta(1)), 2);
    }
}
