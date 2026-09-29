use crate::dominio::control_de_trafico::{ControlDeTrafico, ErrorDeControlDeTrafico};
use crate::dominio::edificio::{ErrorDeEdificio, IdentificadorDeAscensor};
use crate::dominio::planta::Planta;

/// Lo que la interfaz cuenta al usuario tras pulsar un botón de llamada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aviso {
    AscensorEnviado {
        ascensor: IdentificadorDeAscensor,
        planta_de_la_llamada: Planta,
    },
    NingunAscensorLibre,
    NoSePudoAtenderLaLlamada(ErrorDeControlDeTrafico),
}

impl Aviso {
    pub fn como_texto(&self) -> String {
        match self {
            Aviso::AscensorEnviado {
                ascensor,
                planta_de_la_llamada,
            } => format!(
                "El ascensor {} ha ido a la planta {}",
                ascensor.numero(),
                planta_de_la_llamada.numero()
            ),
            Aviso::NingunAscensorLibre => "No hay ningún ascensor libre".to_string(),
            Aviso::NoSePudoAtenderLaLlamada(error) => {
                format!("No se pudo atender la llamada: {error:?}")
            }
        }
    }
}

/// Estado y comportamiento de la interfaz, independientes de la librería gráfica.
pub struct PanelDelSimulador {
    control_de_trafico: ControlDeTrafico,
    ultimo_aviso: Option<Aviso>,
}

impl PanelDelSimulador {
    pub fn new(control_de_trafico: ControlDeTrafico) -> Self {
        Self {
            control_de_trafico,
            ultimo_aviso: None,
        }
    }

    pub fn control_de_trafico(&self) -> &ControlDeTrafico {
        &self.control_de_trafico
    }

    /// Plantas del edificio, de la más alta a la más baja.
    pub fn plantas_de_la_mas_alta_a_la_mas_baja(&self) -> Vec<Planta> {
        let edificio = self.control_de_trafico.edificio();
        (edificio.planta_mas_baja().numero()..=edificio.planta_mas_alta().numero())
            .rev()
            .map(Planta::con_numero)
            .collect()
    }

    pub fn ascensores_en_la_planta(&self, planta: Planta) -> Vec<IdentificadorDeAscensor> {
        let edificio = self.control_de_trafico.edificio();
        edificio
            .identificadores_de_ascensores()
            .into_iter()
            .filter(|&ascensor| edificio.planta_del_ascensor(ascensor) == Ok(planta))
            .collect()
    }

    pub fn ultimo_aviso(&self) -> Option<Aviso> {
        self.ultimo_aviso
    }

    pub fn pulsar_el_boton_de_llamada_en_la_planta(&mut self, planta_de_la_llamada: Planta) {
        let resultado_de_la_llamada = self
            .control_de_trafico
            .atender_llamada_desde_la_planta(planta_de_la_llamada);
        let aviso = match resultado_de_la_llamada {
            Ok(ascensor_enviado) => Aviso::AscensorEnviado {
                ascensor: ascensor_enviado,
                planta_de_la_llamada,
            },
            Err(ErrorDeControlDeTrafico::Edificio(ErrorDeEdificio::NingunAscensorLibre)) => {
                Aviso::NingunAscensorLibre
            }
            Err(error) => Aviso::NoSePudoAtenderLaLlamada(error),
        };
        self.ultimo_aviso = Some(aviso);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptadores::historico_en_memoria::HistoricoEnMemoria;
    use crate::adaptadores::reloj_simulado::RelojSimulado;
    use crate::dominio::edificio::Edificio;
    use crate::dominio::fecha_y_hora::FechaYHora;
    use crate::dominio::historico_de_movimientos::{ErrorDeHistorico, HistoricoDeMovimientos};
    use crate::dominio::movimiento::Movimiento;

    struct HistoricoQueSiempreFalla;

    impl HistoricoDeMovimientos for HistoricoQueSiempreFalla {
        fn registrar(&mut self, _movimiento: Movimiento) -> Result<(), ErrorDeHistorico> {
            Err(ErrorDeHistorico::NoSePudoAccederAlAlmacenamiento)
        }

        fn movimientos(&self) -> Result<Vec<Movimiento>, ErrorDeHistorico> {
            Ok(vec![])
        }
    }

    fn planta(numero: i8) -> Planta {
        Planta::con_numero(numero)
    }

    fn ascensor(numero: usize) -> IdentificadorDeAscensor {
        IdentificadorDeAscensor::con_numero(numero)
    }

    fn panel_con_el_edificio_y_el_historico(
        edificio: Edificio,
        historico: Box<dyn HistoricoDeMovimientos>,
    ) -> PanelDelSimulador {
        let reloj = RelojSimulado::con_la_hora(FechaYHora::con(2026, 9, 29, 10, 0, 0).unwrap());
        PanelDelSimulador::new(ControlDeTrafico::new(edificio, Box::new(reloj), historico))
    }

    fn panel_con_el_edificio(edificio: Edificio) -> PanelDelSimulador {
        panel_con_el_edificio_y_el_historico(edificio, Box::new(HistoricoEnMemoria::new()))
    }

    fn panel_recien_arrancado() -> PanelDelSimulador {
        panel_con_el_edificio(Edificio::new())
    }

    #[test]
    fn la_interfaz_muestra_todas_las_plantas_del_edificio() {
        let panel = panel_recien_arrancado();
        assert_eq!(
            panel.plantas_de_la_mas_alta_a_la_mas_baja().len(),
            panel.control_de_trafico().edificio().numero_de_plantas()
        );
    }

    #[test]
    fn la_interfaz_muestra_las_plantas_ordenadas_de_la_mas_alta_a_la_mas_baja() {
        let panel = panel_recien_arrancado();
        let edificio = panel.control_de_trafico().edificio();
        let plantas = panel.plantas_de_la_mas_alta_a_la_mas_baja();
        assert_eq!(plantas.first(), Some(&edificio.planta_mas_alta()));
        assert_eq!(plantas.last(), Some(&edificio.planta_mas_baja()));
        assert!(
            plantas
                .windows(2)
                .all(|plantas_consecutivas| plantas_consecutivas[0] > plantas_consecutivas[1])
        );
    }

    #[test]
    fn al_arrancar_la_interfaz_muestra_todos_los_ascensores_en_la_planta_de_acceso_principal() {
        let panel = panel_recien_arrancado();
        assert_eq!(
            panel.ascensores_en_la_planta(Planta::DE_ACCESO_PRINCIPAL),
            vec![ascensor(0), ascensor(1), ascensor(2)]
        );
    }

    #[test]
    fn la_interfaz_muestra_cada_ascensor_del_edificio_en_una_sola_planta() {
        let mut panel = panel_recien_arrancado();
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(4));
        let ascensores_mostrados_en_todas_las_plantas: Vec<IdentificadorDeAscensor> = panel
            .plantas_de_la_mas_alta_a_la_mas_baja()
            .into_iter()
            .flat_map(|planta| panel.ascensores_en_la_planta(planta))
            .collect();
        for ascensor in panel
            .control_de_trafico()
            .edificio()
            .identificadores_de_ascensores()
        {
            let veces_que_se_muestra_el_ascensor = ascensores_mostrados_en_todas_las_plantas
                .iter()
                .filter(|&&ascensor_mostrado| ascensor_mostrado == ascensor)
                .count();
            assert_eq!(veces_que_se_muestra_el_ascensor, 1);
        }
    }

    #[test]
    fn la_interfaz_muestra_cada_ascensor_en_la_planta_en_la_que_esta() {
        let mut panel = panel_recien_arrancado();
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(-1));
        let edificio = panel.control_de_trafico().edificio();
        for ascensor in edificio.identificadores_de_ascensores() {
            let planta_en_la_que_esta_el_ascensor = edificio.planta_del_ascensor(ascensor).unwrap();
            assert!(
                panel
                    .ascensores_en_la_planta(planta_en_la_que_esta_el_ascensor)
                    .contains(&ascensor)
            );
        }
    }

    #[test]
    fn la_interfaz_no_muestra_ningun_ascensor_en_una_planta_en_la_que_no_hay_ninguno() {
        let panel = panel_recien_arrancado();
        assert!(panel.ascensores_en_la_planta(planta(5)).is_empty());
    }

    #[test]
    fn pulsar_el_boton_de_llamada_de_una_planta_envia_a_ella_el_ascensor_libre_mas_cercano() {
        let mut edificio = Edificio::new();
        edificio
            .mover_ascensor_a_la_planta(ascensor(2), planta(5))
            .unwrap();
        let mut panel = panel_con_el_edificio(edificio);
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(4));
        assert_eq!(panel.ascensores_en_la_planta(planta(4)), vec![ascensor(2)]);
        assert_eq!(
            panel
                .control_de_trafico()
                .edificio()
                .esta_libre(ascensor(2)),
            Ok(false)
        );
    }

    #[test]
    fn tras_pulsar_el_boton_de_llamada_la_interfaz_muestra_el_ascensor_enviado_en_la_planta_de_la_llamada()
     {
        let mut panel = panel_recien_arrancado();
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(6));
        assert_eq!(panel.ascensores_en_la_planta(planta(6)).len(), 1);
        assert_eq!(
            panel
                .ascensores_en_la_planta(Planta::DE_ACCESO_PRINCIPAL)
                .len(),
            2
        );
    }

    #[test]
    fn se_puede_pulsar_el_boton_de_llamada_de_cualquier_planta_del_edificio() {
        let plantas = panel_recien_arrancado().plantas_de_la_mas_alta_a_la_mas_baja();
        for planta_de_la_llamada in plantas {
            let mut panel = panel_recien_arrancado();
            panel.pulsar_el_boton_de_llamada_en_la_planta(planta_de_la_llamada);
            assert!(matches!(
                panel.ultimo_aviso(),
                Some(Aviso::AscensorEnviado { .. })
            ));
            assert!(
                !panel
                    .ascensores_en_la_planta(planta_de_la_llamada)
                    .is_empty()
            );
        }
    }

    #[test]
    fn pulsar_el_boton_de_llamada_registra_la_llamada_en_el_historico() {
        let mut panel = panel_recien_arrancado();
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(2));
        let movimientos = panel
            .control_de_trafico()
            .historico()
            .movimientos()
            .unwrap();
        assert_eq!(movimientos.len(), 1);
        assert_eq!(movimientos[0].planta_de_destino, planta(2));
    }

    #[test]
    fn al_arrancar_la_interfaz_no_muestra_ningun_aviso() {
        assert_eq!(panel_recien_arrancado().ultimo_aviso(), None);
    }

    #[test]
    fn tras_pulsar_el_boton_de_llamada_la_interfaz_avisa_de_que_ascensor_se_ha_enviado() {
        let mut panel = panel_recien_arrancado();
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(3));
        let ascensor_enviado = panel.ascensores_en_la_planta(planta(3))[0];
        assert_eq!(
            panel.ultimo_aviso(),
            Some(Aviso::AscensorEnviado {
                ascensor: ascensor_enviado,
                planta_de_la_llamada: planta(3)
            })
        );
    }

    #[test]
    fn cada_pulsacion_del_boton_de_llamada_sustituye_el_aviso_anterior() {
        let mut panel = panel_recien_arrancado();
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(3));
        let primer_aviso = panel.ultimo_aviso();
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(5));
        assert_ne!(panel.ultimo_aviso(), primer_aviso);
        assert!(matches!(
            panel.ultimo_aviso(),
            Some(Aviso::AscensorEnviado {
                planta_de_la_llamada,
                ..
            }) if planta_de_la_llamada == planta(5)
        ));
    }

    #[test]
    fn pulsar_el_boton_de_llamada_sin_ascensores_libres_muestra_un_aviso_de_que_no_hay_ninguno_libre()
     {
        let mut panel = panel_recien_arrancado();
        let numero_de_ascensores = panel.control_de_trafico().edificio().numero_de_ascensores();
        for _ in 0..numero_de_ascensores {
            panel.pulsar_el_boton_de_llamada_en_la_planta(planta(1));
        }
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(4));
        assert_eq!(panel.ultimo_aviso(), Some(Aviso::NingunAscensorLibre));
        assert!(!panel.ultimo_aviso().unwrap().como_texto().is_empty());
    }

    #[test]
    fn si_falla_el_registro_de_la_llamada_en_el_historico_la_interfaz_muestra_un_aviso() {
        let mut panel = panel_con_el_edificio_y_el_historico(
            Edificio::new(),
            Box::new(HistoricoQueSiempreFalla),
        );
        panel.pulsar_el_boton_de_llamada_en_la_planta(planta(2));
        assert_eq!(
            panel.ultimo_aviso(),
            Some(Aviso::NoSePudoAtenderLaLlamada(
                ErrorDeControlDeTrafico::Historico(
                    ErrorDeHistorico::NoSePudoAccederAlAlmacenamiento
                )
            ))
        );
    }
}
