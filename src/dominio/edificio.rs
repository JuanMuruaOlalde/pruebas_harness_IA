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

    pub fn numero(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorDeEdificio {
    AscensorInexistente,
    PlantaInexistente,
    NingunAscensorLibre,
    AscensorNoOcupado,
}

/// Estado de un ascensor dentro del edificio: dónde está y si está libre u ocupado.
#[derive(Debug, Clone, Copy)]
struct Ascensor {
    planta: Planta,
    esta_libre: bool,
}

impl Ascensor {
    const LIBRE_EN_LA_PLANTA_DE_ACCESO_PRINCIPAL: Ascensor = Ascensor {
        planta: Planta::DE_ACCESO_PRINCIPAL,
        esta_libre: true,
    };
}

/// Edificio con sus plantas y sus ascensores.
///
/// Cada ascensor puede ir a cualquier planta del edificio, independientemente de los demás.
/// El movimiento es instantáneo: al moverlo, el ascensor queda ya en la planta de destino.
pub struct Edificio {
    ascensores: [Ascensor; NUMERO_DE_ASCENSORES],
}

impl Default for Edificio {
    fn default() -> Self {
        Self::new()
    }
}

impl Edificio {
    /// Crea el edificio con todos sus ascensores libres en la planta de acceso principal.
    pub fn new() -> Self {
        Self {
            ascensores: [Ascensor::LIBRE_EN_LA_PLANTA_DE_ACCESO_PRINCIPAL; NUMERO_DE_ASCENSORES],
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
        Ok(self.ascensores[indice_del_ascensor].planta)
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
        self.ascensores[indice_del_ascensor].planta = planta_de_destino;
        Ok(())
    }

    pub fn esta_libre(
        &self,
        identificador_del_ascensor: IdentificadorDeAscensor,
    ) -> Result<bool, ErrorDeEdificio> {
        let indice_del_ascensor = self.indice_del_ascensor(identificador_del_ascensor)?;
        Ok(self.ascensores[indice_del_ascensor].esta_libre)
    }

    /// Envía a la planta de la llamada el ascensor libre más cercano; a igual distancia, el de
    /// identificador más bajo. El ascensor enviado queda ocupado.
    ///
    /// Si la planta no existe o no hay ningún ascensor libre, devuelve error y nada cambia.
    pub fn enviar_ascensor_libre_mas_cercano_a_la_planta(
        &mut self,
        planta_de_la_llamada: Planta,
    ) -> Result<IdentificadorDeAscensor, ErrorDeEdificio> {
        if !self.incluye_la_planta(planta_de_la_llamada) {
            return Err(ErrorDeEdificio::PlantaInexistente);
        }
        // `min_by_key` devuelve el primero de los empatados: el de identificador más bajo.
        let indice_del_ascensor_mas_cercano = self
            .ascensores
            .iter()
            .enumerate()
            .filter(|(_, ascensor)| ascensor.esta_libre)
            .min_by_key(|(_, ascensor)| ascensor.planta.distancia_hasta(planta_de_la_llamada))
            .map(|(indice_del_ascensor, _)| indice_del_ascensor)
            .ok_or(ErrorDeEdificio::NingunAscensorLibre)?;
        let ascensor_enviado = &mut self.ascensores[indice_del_ascensor_mas_cercano];
        ascensor_enviado.planta = planta_de_la_llamada;
        ascensor_enviado.esta_libre = false;
        Ok(IdentificadorDeAscensor::con_numero(
            indice_del_ascensor_mas_cercano,
        ))
    }

    /// Lleva al usuario a su planta de destino con el ascensor ocupado, que queda libre allí.
    ///
    /// Si el ascensor no existe, está libre o la planta no existe, devuelve error y nada cambia.
    pub fn llevar_al_usuario_a_su_planta_de_destino(
        &mut self,
        identificador_del_ascensor: IdentificadorDeAscensor,
        planta_de_destino: Planta,
    ) -> Result<(), ErrorDeEdificio> {
        let indice_del_ascensor = self.indice_del_ascensor(identificador_del_ascensor)?;
        if self.ascensores[indice_del_ascensor].esta_libre {
            return Err(ErrorDeEdificio::AscensorNoOcupado);
        }
        self.mover_ascensor_a_la_planta(identificador_del_ascensor, planta_de_destino)?;
        self.ascensores[indice_del_ascensor].esta_libre = true;
        Ok(())
    }

    /// Índice del ascensor en `ascensores`, o error si el edificio no tiene ese ascensor.
    fn indice_del_ascensor(
        &self,
        identificador_del_ascensor: IdentificadorDeAscensor,
    ) -> Result<usize, ErrorDeEdificio> {
        let indice_del_ascensor = identificador_del_ascensor.numero();
        if indice_del_ascensor < self.ascensores.len() {
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

    #[test]
    fn al_crear_el_edificio_todos_los_ascensores_estan_libres() {
        let edificio = Edificio::new();
        for identificador in edificio.identificadores_de_ascensores() {
            assert_eq!(edificio.esta_libre(identificador), Ok(true));
        }
    }

    #[test]
    fn consultar_si_esta_libre_un_ascensor_inexistente_devuelve_error() {
        assert_eq!(
            Edificio::new().esta_libre(ascensor(3)),
            Err(ErrorDeEdificio::AscensorInexistente)
        );
    }

    fn edificio_con_ascensores_en(plantas: [i8; 3]) -> Edificio {
        let mut edificio = Edificio::new();
        for (indice, numero) in plantas.iter().enumerate() {
            edificio
                .mover_ascensor_a_la_planta(ascensor(indice), planta(*numero))
                .unwrap();
        }
        edificio
    }

    #[test]
    fn una_llamada_desde_una_planta_envia_a_esa_planta_el_ascensor_libre_mas_cercano() {
        let mut edificio = edificio_con_ascensores_en([0, 5, -2]);
        let enviado = edificio
            .enviar_ascensor_libre_mas_cercano_a_la_planta(planta(4))
            .unwrap();
        assert_eq!(edificio.planta_del_ascensor(enviado), Ok(planta(4)));
        assert_eq!(edificio.planta_del_ascensor(ascensor(1)), Ok(planta(4)));
    }

    #[test]
    fn una_llamada_devuelve_el_identificador_del_ascensor_enviado() {
        let mut edificio = edificio_con_ascensores_en([0, 5, -2]);
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(-2)),
            Ok(ascensor(2))
        );
    }

    #[test]
    fn si_hay_un_ascensor_libre_en_la_planta_de_la_llamada_es_el_que_la_atiende() {
        let mut edificio = edificio_con_ascensores_en([3, 6, 2]);
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(6)),
            Ok(ascensor(1))
        );
    }

    #[test]
    fn el_ascensor_mas_cercano_puede_estar_por_encima_de_la_planta_de_la_llamada() {
        let mut edificio = edificio_con_ascensores_en([-2, 4, 7]);
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(3)),
            Ok(ascensor(1))
        );
    }

    #[test]
    fn el_ascensor_mas_cercano_puede_estar_por_debajo_de_la_planta_de_la_llamada() {
        let mut edificio = edificio_con_ascensores_en([-2, 1, 7]);
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(2)),
            Ok(ascensor(1))
        );
    }

    #[test]
    fn a_igual_distancia_se_envia_el_ascensor_libre_con_el_identificador_mas_bajo() {
        let mut edificio = edificio_con_ascensores_en([6, 2, 2]);
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(4)),
            Ok(ascensor(0))
        );
        let mut edificio = edificio_con_ascensores_en([7, 5, 3]);
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(4)),
            Ok(ascensor(1))
        );
    }

    #[test]
    fn el_ascensor_enviado_a_una_llamada_queda_ocupado() {
        let mut edificio = Edificio::new();
        let enviado = edificio
            .enviar_ascensor_libre_mas_cercano_a_la_planta(planta(3))
            .unwrap();
        assert_eq!(edificio.esta_libre(enviado), Ok(false));
    }

    #[test]
    fn un_ascensor_ocupado_no_se_envia_aunque_sea_el_mas_cercano() {
        let mut edificio = edificio_con_ascensores_en([4, 0, 0]);
        edificio
            .enviar_ascensor_libre_mas_cercano_a_la_planta(planta(4))
            .unwrap();
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(4)),
            Ok(ascensor(1))
        );
    }

    #[test]
    fn una_llamada_no_mueve_los_demas_ascensores() {
        let mut edificio = edificio_con_ascensores_en([0, 5, -2]);
        edificio
            .enviar_ascensor_libre_mas_cercano_a_la_planta(planta(4))
            .unwrap();
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(planta(0)));
        assert_eq!(edificio.planta_del_ascensor(ascensor(2)), Ok(planta(-2)));
        assert_eq!(edificio.esta_libre(ascensor(0)), Ok(true));
        assert_eq!(edificio.esta_libre(ascensor(2)), Ok(true));
    }

    #[test]
    fn una_llamada_sin_ascensores_libres_devuelve_error() {
        let mut edificio = Edificio::new();
        for _ in 0..3 {
            edificio
                .enviar_ascensor_libre_mas_cercano_a_la_planta(planta(1))
                .unwrap();
        }
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(1)),
            Err(ErrorDeEdificio::NingunAscensorLibre)
        );
    }

    #[test]
    fn una_llamada_desde_una_planta_inexistente_devuelve_error() {
        let mut edificio = Edificio::new();
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(8)),
            Err(ErrorDeEdificio::PlantaInexistente)
        );
    }

    #[test]
    fn una_llamada_que_devuelve_error_no_mueve_ni_ocupa_ningun_ascensor() {
        let mut edificio = edificio_con_ascensores_en([1, 2, 3]);
        let _ = edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(-3));
        for (indice, numero) in [1, 2, 3].iter().enumerate() {
            assert_eq!(
                edificio.planta_del_ascensor(ascensor(indice)),
                Ok(planta(*numero))
            );
            assert_eq!(edificio.esta_libre(ascensor(indice)), Ok(true));
        }
    }

    fn edificio_con_un_ascensor_ocupado_en_la_planta(numero_de_planta: i8) -> Edificio {
        let mut edificio = Edificio::new();
        edificio
            .enviar_ascensor_libre_mas_cercano_a_la_planta(planta(numero_de_planta))
            .unwrap();
        edificio
    }

    #[test]
    fn llevar_al_usuario_a_su_planta_de_destino_deja_el_ascensor_en_esa_planta() {
        let mut edificio = edificio_con_un_ascensor_ocupado_en_la_planta(2);
        assert_eq!(
            edificio.llevar_al_usuario_a_su_planta_de_destino(ascensor(0), planta(-1)),
            Ok(())
        );
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(planta(-1)));
    }

    #[test]
    fn al_llevar_al_usuario_a_su_planta_de_destino_el_ascensor_queda_libre() {
        let mut edificio = edificio_con_un_ascensor_ocupado_en_la_planta(2);
        edificio
            .llevar_al_usuario_a_su_planta_de_destino(ascensor(0), planta(5))
            .unwrap();
        assert_eq!(edificio.esta_libre(ascensor(0)), Ok(true));
    }

    #[test]
    fn un_ascensor_que_ha_llevado_al_usuario_a_su_destino_puede_atender_otra_llamada() {
        let mut edificio = edificio_con_ascensores_en([0, 0, 0]);
        for _ in 0..3 {
            edificio
                .enviar_ascensor_libre_mas_cercano_a_la_planta(planta(1))
                .unwrap();
        }
        edificio
            .llevar_al_usuario_a_su_planta_de_destino(ascensor(1), planta(6))
            .unwrap();
        assert_eq!(
            edificio.enviar_ascensor_libre_mas_cercano_a_la_planta(planta(7)),
            Ok(ascensor(1))
        );
    }

    #[test]
    fn llevar_al_usuario_a_su_destino_con_un_ascensor_libre_devuelve_error() {
        let mut edificio = Edificio::new();
        assert_eq!(
            edificio.llevar_al_usuario_a_su_planta_de_destino(ascensor(0), planta(3)),
            Err(ErrorDeEdificio::AscensorNoOcupado)
        );
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(planta(0)));
    }

    #[test]
    fn llevar_al_usuario_a_una_planta_inexistente_devuelve_error_y_el_ascensor_sigue_ocupado_en_su_planta()
     {
        let mut edificio = edificio_con_un_ascensor_ocupado_en_la_planta(2);
        assert_eq!(
            edificio.llevar_al_usuario_a_su_planta_de_destino(ascensor(0), planta(8)),
            Err(ErrorDeEdificio::PlantaInexistente)
        );
        assert_eq!(edificio.planta_del_ascensor(ascensor(0)), Ok(planta(2)));
        assert_eq!(edificio.esta_libre(ascensor(0)), Ok(false));
    }

    #[test]
    fn llevar_al_usuario_a_su_destino_con_un_ascensor_inexistente_devuelve_error() {
        let mut edificio = Edificio::new();
        assert_eq!(
            edificio.llevar_al_usuario_a_su_planta_de_destino(ascensor(3), planta(1)),
            Err(ErrorDeEdificio::AscensorInexistente)
        );
    }
}
