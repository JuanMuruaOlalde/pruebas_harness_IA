use super::planta::Planta;

const NUMERO_DE_PLANTA_MAS_BAJA: i8 = -2;
const NUMERO_DE_PLANTA_MAS_ALTA: i8 = 7;
const NUMERO_DE_ASCENSORES: usize = 3;

/// Identifica a cada ascensor del edificio. Los ascensores se numeran desde 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdentificadorDeAscensor(usize);

impl IdentificadorDeAscensor {
    pub fn con_numero(numero: usize) -> Self {
        Self(numero)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorDeEdificio {
    AscensorInexistente,
    PlantaInexistente,
}

/// Edificio con sus plantas y sus ascensores.
///
/// Cada ascensor puede ir a cualquier planta del edificio, independientemente de los demás.
/// El movimiento es instantáneo: al moverlo, el ascensor queda ya en la planta de destino.
pub struct Edificio {
    planta_de_cada_ascensor: [Planta; NUMERO_DE_ASCENSORES],
}

impl Default for Edificio {
    fn default() -> Self {
        Self::new()
    }
}

impl Edificio {
    /// Crea el edificio con todos sus ascensores en la planta de acceso principal.
    pub fn new() -> Self {
        Self {
            planta_de_cada_ascensor: [Planta::DE_ACCESO_PRINCIPAL; NUMERO_DE_ASCENSORES],
        }
    }

    pub fn numero_de_plantas(&self) -> usize {
        (NUMERO_DE_PLANTA_MAS_ALTA - NUMERO_DE_PLANTA_MAS_BAJA + 1) as usize
    }

    pub fn planta_mas_baja(&self) -> Planta {
        Planta::con_numero(NUMERO_DE_PLANTA_MAS_BAJA)
    }

    pub fn planta_mas_alta(&self) -> Planta {
        Planta::con_numero(NUMERO_DE_PLANTA_MAS_ALTA)
    }

    pub fn incluye_la_planta(&self, planta: Planta) -> bool {
        (NUMERO_DE_PLANTA_MAS_BAJA..=NUMERO_DE_PLANTA_MAS_ALTA).contains(&planta.numero())
    }

    pub fn numero_de_ascensores(&self) -> usize {
        NUMERO_DE_ASCENSORES
    }

    pub fn identificadores_de_ascensores(&self) -> Vec<IdentificadorDeAscensor> {
        (0..NUMERO_DE_ASCENSORES)
            .map(IdentificadorDeAscensor::con_numero)
            .collect()
    }

    pub fn planta_del_ascensor(
        &self,
        identificador_del_ascensor: IdentificadorDeAscensor,
    ) -> Result<Planta, ErrorDeEdificio> {
        let indice_del_ascensor = self.indice_del_ascensor(identificador_del_ascensor)?;
        Ok(self.planta_de_cada_ascensor[indice_del_ascensor])
    }

    /// Mueve el ascensor a la planta de destino.
    ///
    /// Si el ascensor o la planta no existen en el edificio, devuelve error
    /// y ningún ascensor cambia de planta.
    pub fn mover_ascensor_a_la_planta(
        &mut self,
        identificador_del_ascensor: IdentificadorDeAscensor,
        planta_de_destino: Planta,
    ) -> Result<(), ErrorDeEdificio> {
        let indice_del_ascensor = self.indice_del_ascensor(identificador_del_ascensor)?;
        if !self.incluye_la_planta(planta_de_destino) {
            return Err(ErrorDeEdificio::PlantaInexistente);
        }
        self.planta_de_cada_ascensor[indice_del_ascensor] = planta_de_destino;
        Ok(())
    }

    /// Índice del ascensor en `planta_de_cada_ascensor`, o error si el edificio no tiene ese ascensor.
    fn indice_del_ascensor(
        &self,
        identificador_del_ascensor: IdentificadorDeAscensor,
    ) -> Result<usize, ErrorDeEdificio> {
        let indice_del_ascensor = identificador_del_ascensor.0;
        if indice_del_ascensor < self.planta_de_cada_ascensor.len() {
            Ok(indice_del_ascensor)
        } else {
            Err(ErrorDeEdificio::AscensorInexistente)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn planta(numero: i8) -> Planta {
        Planta::con_numero(numero)
    }

    fn ascensor(numero: usize) -> IdentificadorDeAscensor {
        IdentificadorDeAscensor::con_numero(numero)
    }

    #[test]
    fn el_edificio_tiene_diez_plantas() {
        assert_eq!(Edificio::new().numero_de_plantas(), 10);
    }

    #[test]
    fn la_planta_mas_baja_del_edificio_es_la_menos_dos() {
        assert_eq!(Edificio::new().planta_mas_baja(), planta(-2));
    }

    #[test]
    fn la_planta_mas_alta_del_edificio_es_la_siete() {
        assert_eq!(Edificio::new().planta_mas_alta(), planta(7));
    }

    #[test]
    fn el_edificio_incluye_la_planta_cero_de_acceso_principal() {
        assert!(Edificio::new().incluye_la_planta(planta(0)));
    }

    #[test]
    fn una_planta_dentro_del_rango_del_edificio_es_valida() {
        let edificio = Edificio::new();
        assert!(edificio.incluye_la_planta(planta(-1)));
        assert!(edificio.incluye_la_planta(planta(5)));
    }

    #[test]
    fn una_planta_por_encima_de_la_mas_alta_no_es_valida() {
        assert!(!Edificio::new().incluye_la_planta(planta(8)));
    }

    #[test]
    fn una_planta_por_debajo_de_la_mas_baja_no_es_valida() {
        assert!(!Edificio::new().incluye_la_planta(planta(-3)));
    }

    #[test]
    fn el_edificio_tiene_tres_ascensores() {
        let edificio = Edificio::new();
        assert_eq!(edificio.numero_de_ascensores(), 3);
        assert_eq!(edificio.identificadores_de_ascensores().len(), 3);
    }

    #[test]
    fn cada_ascensor_del_edificio_tiene_un_identificador_distinto() {
        let identificadores = Edificio::new().identificadores_de_ascensores();
        for (posicion, identificador) in identificadores.iter().enumerate() {
            assert!(!identificadores[posicion + 1..].contains(identificador));
        }
    }

    #[test]
    fn al_crear_el_edificio_todos_los_ascensores_estan_en_la_planta_cero() {
        let edificio = Edificio::new();
        for identificador in edificio.identificadores_de_ascensores() {
            assert_eq!(edificio.planta_del_ascensor(identificador), Ok(planta(0)));
        }
    }

    #[test]
    fn se_puede_consultar_la_planta_en_la_que_esta_cada_ascensor() {
        let mut edificio = Edificio::new();
        let identificadores = edificio.identificadores_de_ascensores();
        for (indice, identificador) in identificadores.iter().enumerate() {
            edificio
                .mover_ascensor_a_la_planta(*identificador, planta(indice as i8 + 1))
                .unwrap();
        }
        for (indice, identificador) in identificadores.iter().enumerate() {
            assert_eq!(
                edificio.planta_del_ascensor(*identificador),
                Ok(planta(indice as i8 + 1))
            );
        }
    }

    #[test]
    fn consultar_la_planta_de_un_ascensor_inexistente_devuelve_error() {
        assert_eq!(
            Edificio::new().planta_del_ascensor(ascensor(3)),
            Err(ErrorDeEdificio::AscensorInexistente)
        );
    }

    #[test]
    fn mover_un_ascensor_a_una_planta_superior_lo_deja_en_esa_planta() {
        let mut edificio = Edificio::new();
        assert_eq!(
            edificio.mover_ascensor_a_la_planta(ascensor(0), planta(4)),
            Ok(())
        );
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(planta(4)));
    }

    #[test]
    fn mover_un_ascensor_a_una_planta_inferior_lo_deja_en_esa_planta() {
        let mut edificio = Edificio::new();
        assert_eq!(
            edificio.mover_ascensor_a_la_planta(ascensor(1), planta(-1)),
            Ok(())
        );
        assert_eq!(edificio.planta_del_ascensor(ascensor(1)), Ok(planta(-1)));
    }

    #[test]
    fn mover_un_ascensor_a_la_planta_en_la_que_ya_esta_no_cambia_su_planta() {
        let mut edificio = Edificio::new();
        assert_eq!(
            edificio.mover_ascensor_a_la_planta(ascensor(2), planta(0)),
            Ok(())
        );
        assert_eq!(edificio.planta_del_ascensor(ascensor(2)), Ok(planta(0)));
    }

    #[test]
    fn mover_un_ascensor_a_la_planta_mas_alta_es_posible() {
        let mut edificio = Edificio::new();
        let mas_alta = edificio.planta_mas_alta();
        assert_eq!(
            edificio.mover_ascensor_a_la_planta(ascensor(0), mas_alta),
            Ok(())
        );
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(mas_alta));
    }

    #[test]
    fn mover_un_ascensor_a_la_planta_mas_baja_es_posible() {
        let mut edificio = Edificio::new();
        let mas_baja = edificio.planta_mas_baja();
        assert_eq!(
            edificio.mover_ascensor_a_la_planta(ascensor(0), mas_baja),
            Ok(())
        );
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(mas_baja));
    }

    #[test]
    fn cada_ascensor_puede_ir_a_todas_las_plantas_del_edificio() {
        let mut edificio = Edificio::new();
        for identificador in edificio.identificadores_de_ascensores() {
            for numero in -2..=7 {
                assert_eq!(
                    edificio.mover_ascensor_a_la_planta(identificador, planta(numero)),
                    Ok(())
                );
                assert_eq!(
                    edificio.planta_del_ascensor(identificador),
                    Ok(planta(numero))
                );
            }
        }
    }

    #[test]
    fn mover_un_ascensor_a_una_planta_inexistente_devuelve_error() {
        let mut edificio = Edificio::new();
        assert_eq!(
            edificio.mover_ascensor_a_la_planta(ascensor(0), planta(8)),
            Err(ErrorDeEdificio::PlantaInexistente)
        );
        assert_eq!(
            edificio.mover_ascensor_a_la_planta(ascensor(0), planta(-3)),
            Err(ErrorDeEdificio::PlantaInexistente)
        );
    }

    #[test]
    fn mover_un_ascensor_a_una_planta_inexistente_no_cambia_su_planta() {
        let mut edificio = Edificio::new();
        edificio
            .mover_ascensor_a_la_planta(ascensor(0), planta(3))
            .unwrap();
        let _ = edificio.mover_ascensor_a_la_planta(ascensor(0), planta(9));
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(planta(3)));
    }

    #[test]
    fn mover_un_ascensor_inexistente_devuelve_error() {
        let mut edificio = Edificio::new();
        assert_eq!(
            edificio.mover_ascensor_a_la_planta(ascensor(3), planta(1)),
            Err(ErrorDeEdificio::AscensorInexistente)
        );
    }

    #[test]
    fn mover_un_ascensor_no_cambia_la_planta_de_los_demas_ascensores() {
        let mut edificio = Edificio::new();
        edificio
            .mover_ascensor_a_la_planta(ascensor(1), planta(5))
            .unwrap();
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(planta(0)));
        assert_eq!(edificio.planta_del_ascensor(ascensor(2)), Ok(planta(0)));
    }

    #[test]
    fn los_tres_ascensores_pueden_estar_a_la_vez_en_plantas_distintas() {
        let mut edificio = Edificio::new();
        edificio
            .mover_ascensor_a_la_planta(ascensor(0), planta(-2))
            .unwrap();
        edificio
            .mover_ascensor_a_la_planta(ascensor(1), planta(3))
            .unwrap();
        edificio
            .mover_ascensor_a_la_planta(ascensor(2), planta(7))
            .unwrap();
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(planta(-2)));
        assert_eq!(edificio.planta_del_ascensor(ascensor(1)), Ok(planta(3)));
        assert_eq!(edificio.planta_del_ascensor(ascensor(2)), Ok(planta(7)));
    }

    #[test]
    fn varios_movimientos_sucesivos_dejan_el_ascensor_en_la_ultima_planta_indicada() {
        let mut edificio = Edificio::new();
        for numero in [5, -2, 3, 7, 1] {
            edificio
                .mover_ascensor_a_la_planta(ascensor(0), planta(numero))
                .unwrap();
        }
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(planta(1)));
    }
}
