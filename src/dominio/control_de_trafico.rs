use std::cmp::Reverse;
use std::collections::BTreeMap;

use super::edificio::{Edificio, ErrorDeEdificio, IdentificadorDeAscensor};
use super::fecha_y_hora::FechaYHora;
use super::historico_de_movimientos::{ErrorDeHistorico, HistoricoDeMovimientos};
use super::movimiento::{MotivoDelMovimiento, Movimiento};
use super::planta::Planta;
use super::reloj::Reloj;

const LOS_ASCENSORES_DEL_PROPIO_EDIFICIO_SIEMPRE_EXISTEN: &str =
    "los identificadores del propio edificio siempre existen";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorDeControlDeTrafico {
    Edificio(ErrorDeEdificio),
    Historico(ErrorDeHistorico),
}

impl From<ErrorDeEdificio> for ErrorDeControlDeTrafico {
    fn from(error: ErrorDeEdificio) -> Self {
        Self::Edificio(error)
    }
}

impl From<ErrorDeHistorico> for ErrorDeControlDeTrafico {
    fn from(error: ErrorDeHistorico) -> Self {
        Self::Historico(error)
    }
}

/// Decide qué ascensor atiende cada llamada y conserva el histórico de movimientos.
pub struct ControlDeTrafico {
    edificio: Edificio,
    reloj: Box<dyn Reloj>,
    historico: Box<dyn HistoricoDeMovimientos>,
}

impl ControlDeTrafico {
    pub fn new(
        edificio: Edificio,
        reloj: Box<dyn Reloj>,
        historico: Box<dyn HistoricoDeMovimientos>,
    ) -> Self {
        Self {
            edificio,
            reloj,
            historico,
        }
    }

    pub fn edificio(&self) -> &Edificio {
        &self.edificio
    }

    pub fn historico(&self) -> &dyn HistoricoDeMovimientos {
        self.historico.as_ref()
    }

    /// Un usuario pulsa el botón de llamada en una planta: se envía el ascensor libre más cercano.
    pub fn atender_llamada_desde_la_planta(
        &mut self,
        planta_de_la_llamada: Planta,
    ) -> Result<IdentificadorDeAscensor, ErrorDeControlDeTrafico> {
        let planta_de_cada_ascensor_antes_de_la_llamada = self.planta_de_cada_ascensor();
        let ascensor_enviado = self
            .edificio
            .enviar_ascensor_libre_mas_cercano_a_la_planta(planta_de_la_llamada)?;
        let planta_de_origen = planta_de_cada_ascensor_antes_de_la_llamada
            .into_iter()
            .find_map(|(ascensor, planta)| (ascensor == ascensor_enviado).then_some(planta))
            .expect(LOS_ASCENSORES_DEL_PROPIO_EDIFICIO_SIEMPRE_EXISTEN);
        self.registrar_movimiento(
            ascensor_enviado,
            planta_de_origen,
            planta_de_la_llamada,
            MotivoDelMovimiento::AtenderUnaLlamada,
        )?;
        Ok(ascensor_enviado)
    }

    /// Lleva al usuario a su planta de destino con el ascensor ocupado, que queda libre.
    pub fn llevar_al_usuario_a_su_planta_de_destino(
        &mut self,
        identificador_del_ascensor: IdentificadorDeAscensor,
        planta_de_destino: Planta,
    ) -> Result<(), ErrorDeControlDeTrafico> {
        let planta_de_origen = self
            .edificio
            .planta_del_ascensor(identificador_del_ascensor)?;
        self.edificio.llevar_al_usuario_a_su_planta_de_destino(
            identificador_del_ascensor,
            planta_de_destino,
        )?;
        self.registrar_movimiento(
            identificador_del_ascensor,
            planta_de_origen,
            planta_de_destino,
            MotivoDelMovimiento::LlevarAlUsuarioASuDestino,
        )?;
        Ok(())
    }

    /// Reubica los ascensores libres en las plantas desde las que más se ha llamado, en el mismo
    /// día de la semana y la misma franja horaria durante el último mes, para agilizar las
    /// próximas llamadas. Los ascensores ocupados no se mueven.
    pub fn optimizar_posicion_de_ascensores_libres(
        &mut self,
    ) -> Result<(), ErrorDeControlDeTrafico> {
        let mut ascensores_libres_sin_asignar = self.ascensores_libres();
        let plantas_elegidas =
            self.plantas_con_mas_llamadas(ascensores_libres_sin_asignar.len())?;
        let plantas_elegidas_sin_ascensor = self
            .dejar_en_su_sitio_los_ascensores_libres_que_ya_estan_en_una_planta_elegida(
                plantas_elegidas,
                &mut ascensores_libres_sin_asignar,
            );
        for planta_de_destino in plantas_elegidas_sin_ascensor {
            let ascensor_a_reubicar = self.retirar_el_ascensor_mas_cercano_a_la_planta(
                &mut ascensores_libres_sin_asignar,
                planta_de_destino,
            );
            self.reubicar_ascensor_libre(ascensor_a_reubicar, planta_de_destino)?;
        }
        Ok(())
    }

    /// Cada planta elegida en la que ya hay algún ascensor libre se queda con uno de ellos, que se
    /// retira de `ascensores_libres_sin_asignar`. Devuelve las plantas elegidas que siguen sin
    /// ascensor.
    fn dejar_en_su_sitio_los_ascensores_libres_que_ya_estan_en_una_planta_elegida(
        &self,
        plantas_elegidas: Vec<Planta>,
        ascensores_libres_sin_asignar: &mut Vec<IdentificadorDeAscensor>,
    ) -> Vec<Planta> {
        let mut plantas_elegidas_sin_ascensor = vec![];
        for planta_elegida in plantas_elegidas {
            let posicion_del_ascensor_ya_presente = ascensores_libres_sin_asignar
                .iter()
                .position(|&ascensor| self.planta_actual_del_ascensor(ascensor) == planta_elegida);
            match posicion_del_ascensor_ya_presente {
                Some(posicion) => {
                    ascensores_libres_sin_asignar.remove(posicion);
                }
                None => plantas_elegidas_sin_ascensor.push(planta_elegida),
            }
        }
        plantas_elegidas_sin_ascensor
    }

    /// Retira de los candidatos el ascensor más cercano a la planta de destino y lo devuelve.
    /// A igual distancia, el que aparece antes entre los candidatos.
    fn retirar_el_ascensor_mas_cercano_a_la_planta(
        &self,
        ascensores_candidatos: &mut Vec<IdentificadorDeAscensor>,
        planta_de_destino: Planta,
    ) -> IdentificadorDeAscensor {
        let posicion_del_mas_cercano = ascensores_candidatos
            .iter()
            .enumerate()
            .min_by_key(|&(_, &ascensor)| {
                self.planta_actual_del_ascensor(ascensor)
                    .distancia_hasta(planta_de_destino)
            })
            .map(|(posicion, _)| posicion)
            .expect("hay tantos ascensores libres como plantas elegidas");
        ascensores_candidatos.remove(posicion_del_mas_cercano)
    }

    /// Mueve un ascensor libre a la planta de destino, donde sigue libre, y lo registra.
    fn reubicar_ascensor_libre(
        &mut self,
        ascensor_a_reubicar: IdentificadorDeAscensor,
        planta_de_destino: Planta,
    ) -> Result<(), ErrorDeControlDeTrafico> {
        let planta_de_origen = self.planta_actual_del_ascensor(ascensor_a_reubicar);
        self.edificio
            .mover_ascensor_a_la_planta(ascensor_a_reubicar, planta_de_destino)?;
        self.registrar_movimiento(
            ascensor_a_reubicar,
            planta_de_origen,
            planta_de_destino,
            MotivoDelMovimiento::ReubicarUnAscensorLibre,
        )?;
        Ok(())
    }

    /// Como máximo `numero_maximo_de_plantas` plantas, de más a menos llamadas en el mismo día de
    /// la semana y la misma franja horaria del último mes. Solo cuentan las plantas con alguna
    /// llamada. A igual número de llamadas, gana la más cercana a la planta de acceso principal
    /// y, si aún hay empate, la de número más bajo.
    fn plantas_con_mas_llamadas(
        &self,
        numero_maximo_de_plantas: usize,
    ) -> Result<Vec<Planta>, ErrorDeHistorico> {
        let mut plantas_con_su_numero_de_llamadas: Vec<(Planta, usize)> = self
            .numero_de_llamadas_por_planta_en_la_franja_horaria_actual()?
            .into_iter()
            .collect();
        plantas_con_su_numero_de_llamadas.sort_by_key(|&(planta, numero_de_llamadas)| {
            (
                Reverse(numero_de_llamadas),
                planta.distancia_hasta(Planta::DE_ACCESO_PRINCIPAL),
                planta.numero(),
            )
        });
        Ok(plantas_con_su_numero_de_llamadas
            .into_iter()
            .take(numero_maximo_de_plantas)
            .map(|(planta, _)| planta)
            .collect())
    }

    /// Llamadas atendidas desde cada planta en el mismo día de la semana y la misma franja
    /// horaria que ahora, durante el último mes. Solo aparecen las plantas con alguna llamada.
    fn numero_de_llamadas_por_planta_en_la_franja_horaria_actual(
        &self,
    ) -> Result<BTreeMap<Planta, usize>, ErrorDeHistorico> {
        let ahora = self.reloj.ahora();
        let mut numero_de_llamadas_por_planta: BTreeMap<Planta, usize> = BTreeMap::new();
        for movimiento in self.historico.movimientos()? {
            if es_una_llamada_del_mismo_dia_de_la_semana_y_franja_horaria_en_el_ultimo_mes(
                &movimiento,
                ahora,
            ) {
                *numero_de_llamadas_por_planta
                    .entry(movimiento.planta_de_destino)
                    .or_default() += 1;
            }
        }
        Ok(numero_de_llamadas_por_planta)
    }

    fn ascensores_libres(&self) -> Vec<IdentificadorDeAscensor> {
        self.edificio
            .identificadores_de_ascensores()
            .into_iter()
            .filter(|&ascensor| {
                self.edificio
                    .esta_libre(ascensor)
                    .expect(LOS_ASCENSORES_DEL_PROPIO_EDIFICIO_SIEMPRE_EXISTEN)
            })
            .collect()
    }

    fn planta_de_cada_ascensor(&self) -> Vec<(IdentificadorDeAscensor, Planta)> {
        self.edificio
            .identificadores_de_ascensores()
            .into_iter()
            .map(|ascensor| (ascensor, self.planta_actual_del_ascensor(ascensor)))
            .collect()
    }

    /// Planta de uno de los ascensores del propio edificio.
    fn planta_actual_del_ascensor(&self, ascensor: IdentificadorDeAscensor) -> Planta {
        self.edificio
            .planta_del_ascensor(ascensor)
            .expect(LOS_ASCENSORES_DEL_PROPIO_EDIFICIO_SIEMPRE_EXISTEN)
    }

    fn registrar_movimiento(
        &mut self,
        ascensor: IdentificadorDeAscensor,
        planta_de_origen: Planta,
        planta_de_destino: Planta,
        motivo: MotivoDelMovimiento,
    ) -> Result<(), ErrorDeHistorico> {
        self.historico.registrar(Movimiento {
            ascensor,
            planta_de_origen,
            planta_de_destino,
            fecha_y_hora: self.reloj.ahora(),
            motivo,
        })
    }
}

/// Indica si el movimiento se hizo para atender una llamada en el mismo día de la semana y la
/// misma franja horaria que `ahora`, durante el último mes.
fn es_una_llamada_del_mismo_dia_de_la_semana_y_franja_horaria_en_el_ultimo_mes(
    movimiento: &Movimiento,
    ahora: FechaYHora,
) -> bool {
    let momento_del_movimiento = movimiento.fecha_y_hora;
    movimiento.motivo == MotivoDelMovimiento::AtenderUnaLlamada
        && momento_del_movimiento.dia_de_la_semana() == ahora.dia_de_la_semana()
        && momento_del_movimiento.franja_horaria() == ahora.franja_horaria()
        && momento_del_movimiento.pertenece_al_ultimo_mes_visto_desde(ahora)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptadores::historico_en_memoria::HistoricoEnMemoria;
    use crate::adaptadores::reloj_simulado::RelojSimulado;

    fn planta(numero: i8) -> Planta {
        Planta::con_numero(numero)
    }

    fn ascensor(numero: usize) -> IdentificadorDeAscensor {
        IdentificadorDeAscensor::con_numero(numero)
    }

    fn fecha_y_hora(anio: i32, mes: u8, dia: u8, hora: u8) -> FechaYHora {
        FechaYHora::con(anio, mes, dia, hora, 0, 0).unwrap()
    }

    fn el_martes_a_las_diez() -> FechaYHora {
        fecha_y_hora(2026, 9, 29, 10)
    }

    fn control_de_trafico_con_reloj() -> (ControlDeTrafico, RelojSimulado) {
        let reloj = RelojSimulado::con_la_hora(el_martes_a_las_diez());
        let control = ControlDeTrafico::new(
            Edificio::new(),
            Box::new(reloj.clone()),
            Box::new(HistoricoEnMemoria::new()),
        );
        (control, reloj)
    }

    fn movimientos_registrados(control: &ControlDeTrafico) -> Vec<Movimiento> {
        control.historico().movimientos().unwrap()
    }

    #[test]
    fn atender_una_llamada_registra_un_movimiento_en_el_historico() {
        let (mut control, _) = control_de_trafico_con_reloj();
        control.atender_llamada_desde_la_planta(planta(3)).unwrap();
        assert_eq!(movimientos_registrados(&control).len(), 1);
    }

    #[test]
    fn el_movimiento_registrado_indica_el_ascensor_y_sus_plantas_de_origen_y_destino() {
        let (mut control, _) = control_de_trafico_con_reloj();
        control.atender_llamada_desde_la_planta(planta(3)).unwrap();
        let movimiento = movimientos_registrados(&control)[0];
        assert_eq!(movimiento.ascensor, ascensor(0));
        assert_eq!(movimiento.planta_de_origen, planta(0));
        assert_eq!(movimiento.planta_de_destino, planta(3));
    }

    #[test]
    fn el_movimiento_registrado_lleva_la_fecha_y_hora_que_marca_el_reloj() {
        let (mut control, reloj) = control_de_trafico_con_reloj();
        control.atender_llamada_desde_la_planta(planta(3)).unwrap();
        reloj.fijar_la_hora(fecha_y_hora(2026, 9, 30, 18));
        control.atender_llamada_desde_la_planta(planta(5)).unwrap();
        let movimientos = movimientos_registrados(&control);
        assert_eq!(movimientos[0].fecha_y_hora, el_martes_a_las_diez());
        assert_eq!(movimientos[1].fecha_y_hora, fecha_y_hora(2026, 9, 30, 18));
    }

    #[test]
    fn el_movimiento_para_atender_una_llamada_se_registra_con_ese_motivo() {
        let (mut control, _) = control_de_trafico_con_reloj();
        control.atender_llamada_desde_la_planta(planta(3)).unwrap();
        assert_eq!(
            movimientos_registrados(&control)[0].motivo,
            MotivoDelMovimiento::AtenderUnaLlamada
        );
    }

    #[test]
    fn atender_una_llamada_con_un_ascensor_que_ya_esta_en_esa_planta_tambien_se_registra() {
        let (mut control, _) = control_de_trafico_con_reloj();
        control.atender_llamada_desde_la_planta(planta(0)).unwrap();
        let movimiento = movimientos_registrados(&control)[0];
        assert_eq!(movimiento.planta_de_origen, planta(0));
        assert_eq!(movimiento.planta_de_destino, planta(0));
    }

    #[test]
    fn llevar_al_usuario_a_su_destino_registra_un_movimiento_con_ese_motivo() {
        let (mut control, _) = control_de_trafico_con_reloj();
        let enviado = control.atender_llamada_desde_la_planta(planta(3)).unwrap();
        control
            .llevar_al_usuario_a_su_planta_de_destino(enviado, planta(-1))
            .unwrap();
        let movimientos = movimientos_registrados(&control);
        assert_eq!(movimientos.len(), 2);
        assert_eq!(movimientos[1].ascensor, enviado);
        assert_eq!(movimientos[1].planta_de_origen, planta(3));
        assert_eq!(movimientos[1].planta_de_destino, planta(-1));
        assert_eq!(
            movimientos[1].motivo,
            MotivoDelMovimiento::LlevarAlUsuarioASuDestino
        );
    }

    #[test]
    fn las_operaciones_que_devuelven_error_no_registran_ningun_movimiento() {
        let (mut control, _) = control_de_trafico_con_reloj();
        assert!(control.atender_llamada_desde_la_planta(planta(9)).is_err());
        assert!(
            control
                .llevar_al_usuario_a_su_planta_de_destino(ascensor(0), planta(1))
                .is_err()
        );
        assert!(movimientos_registrados(&control).is_empty());
    }

    #[test]
    fn el_historico_conserva_los_movimientos_en_el_orden_en_que_se_produjeron() {
        let (mut control, _) = control_de_trafico_con_reloj();
        let primero = control.atender_llamada_desde_la_planta(planta(2)).unwrap();
        let segundo = control.atender_llamada_desde_la_planta(planta(5)).unwrap();
        control
            .llevar_al_usuario_a_su_planta_de_destino(primero, planta(7))
            .unwrap();
        let destinos: Vec<Planta> = movimientos_registrados(&control)
            .iter()
            .map(|movimiento| movimiento.planta_de_destino)
            .collect();
        assert_eq!(destinos, vec![planta(2), planta(5), planta(7)]);
        assert_ne!(primero, segundo);
    }

    // Optimización de la posición de los ascensores libres.
    // El reloj marca el martes 29 de septiembre de 2026 a las 10:00. Los martes del último mes son
    // el 1, 8, 15, 22 y 29 de septiembre; el 25 de agosto (también martes) queda a 35 días.

    fn llamada_desde(numero_de_planta: i8, fecha_y_hora: FechaYHora) -> Movimiento {
        movimiento_con_motivo(
            numero_de_planta,
            fecha_y_hora,
            MotivoDelMovimiento::AtenderUnaLlamada,
        )
    }

    fn movimiento_con_motivo(
        numero_de_planta: i8,
        fecha_y_hora: FechaYHora,
        motivo: MotivoDelMovimiento,
    ) -> Movimiento {
        Movimiento {
            ascensor: ascensor(0),
            planta_de_origen: planta(0),
            planta_de_destino: planta(numero_de_planta),
            fecha_y_hora,
            motivo,
        }
    }

    /// `veces` llamadas en martes distintos del último mes, a las 10:30.
    fn llamadas_de_un_martes_a_las_diez_desde(
        numero_de_planta: i8,
        veces: usize,
    ) -> Vec<Movimiento> {
        [1, 8, 15, 22]
            .iter()
            .take(veces)
            .map(|dia| {
                llamada_desde(
                    numero_de_planta,
                    FechaYHora::con(2026, 9, *dia, 10, 30, 0).unwrap(),
                )
            })
            .collect()
    }

    fn control_con_historico_y_ascensores(
        movimientos_previos: Vec<Movimiento>,
        plantas_de_los_ascensores: [i8; 3],
        ascensores_ocupados: &[usize],
    ) -> ControlDeTrafico {
        let mut edificio = Edificio::new();
        for indice_ocupado in ascensores_ocupados {
            // Para ocupar justo ese ascensor, se deja como único libre en la planta 0.
            for identificador in edificio.identificadores_de_ascensores() {
                let numero_de_planta = if identificador == ascensor(*indice_ocupado) {
                    0
                } else {
                    -2
                };
                edificio
                    .mover_ascensor_a_la_planta(identificador, planta(numero_de_planta))
                    .unwrap();
            }
            let enviado = edificio
                .enviar_ascensor_libre_mas_cercano_a_la_planta(planta(0))
                .unwrap();
            assert_eq!(enviado, ascensor(*indice_ocupado));
        }
        for (indice, numero) in plantas_de_los_ascensores.iter().enumerate() {
            edificio
                .mover_ascensor_a_la_planta(ascensor(indice), planta(*numero))
                .unwrap();
        }
        let mut historico = HistoricoEnMemoria::new();
        for movimiento in movimientos_previos {
            historico.registrar(movimiento).unwrap();
        }
        ControlDeTrafico::new(
            edificio,
            Box::new(RelojSimulado::con_la_hora(el_martes_a_las_diez())),
            Box::new(historico),
        )
    }

    fn plantas_actuales(control: &ControlDeTrafico) -> Vec<Planta> {
        control
            .edificio()
            .identificadores_de_ascensores()
            .into_iter()
            .map(|identificador| {
                control
                    .edificio()
                    .planta_del_ascensor(identificador)
                    .unwrap()
            })
            .collect()
    }

    fn plantas(numeros: [i8; 3]) -> Vec<Planta> {
        numeros.iter().map(|numero| planta(*numero)).collect()
    }

    #[test]
    fn sin_llamadas_en_el_historico_la_optimizacion_no_mueve_ningun_ascensor() {
        let mut control = control_con_historico_y_ascensores(vec![], [1, 2, 3], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([1, 2, 3]));
        assert!(movimientos_registrados(&control).is_empty());
    }

    #[test]
    fn la_optimizacion_lleva_un_ascensor_libre_a_la_planta_con_mas_llamadas() {
        let mut control = control_con_historico_y_ascensores(
            llamadas_de_un_martes_a_las_diez_desde(5, 3),
            [0, 0, 0],
            &[],
        );
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([5, 0, 0]));
    }

    #[test]
    fn la_optimizacion_reparte_los_ascensores_libres_entre_las_plantas_con_mas_llamadas() {
        let mut historico = llamadas_de_un_martes_a_las_diez_desde(5, 3);
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 2));
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(-1, 1));
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(7, 1));
        let mut control = control_con_historico_y_ascensores(historico, [0, 0, 0], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([5, 2, -1]));
    }

    #[test]
    fn a_igual_numero_de_llamadas_la_optimizacion_prefiere_la_planta_mas_cercana_a_la_de_acceso_principal()
     {
        let mut historico = llamadas_de_un_martes_a_las_diez_desde(3, 1);
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(-1, 1));
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(6, 1));
        let mut control = control_con_historico_y_ascensores(historico, [7, 7, 7], &[1, 2]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([-1, 7, 7]));

        // A igual distancia de la planta 0, gana la de número más bajo.
        let mut historico = llamadas_de_un_martes_a_las_diez_desde(1, 1);
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(-1, 1));
        let mut control = control_con_historico_y_ascensores(historico, [7, 7, 7], &[1, 2]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([-1, 7, 7]));
    }

    #[test]
    fn la_optimizacion_no_mueve_un_ascensor_libre_que_ya_esta_en_una_de_las_plantas_elegidas() {
        let mut historico = llamadas_de_un_martes_a_las_diez_desde(5, 2);
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 1));
        let previos = historico.len();
        let mut control = control_con_historico_y_ascensores(historico, [5, 0, 0], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([5, 2, 0]));
        let nuevos = movimientos_registrados(&control).split_off(previos);
        assert_eq!(nuevos.len(), 1);
        assert_eq!(nuevos[0].ascensor, ascensor(1));
    }

    #[test]
    fn si_varios_ascensores_libres_estan_en_una_planta_elegida_solo_uno_se_queda_en_ella() {
        let mut historico = llamadas_de_un_martes_a_las_diez_desde(5, 3);
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 2));
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(-1, 1));
        let mut control = control_con_historico_y_ascensores(historico, [5, 5, 5], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([5, 2, -1]));
    }

    #[test]
    fn la_optimizacion_envia_a_cada_planta_elegida_el_ascensor_libre_mas_cercano_de_los_que_quedan()
    {
        let mut historico = llamadas_de_un_martes_a_las_diez_desde(4, 3);
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(6, 2));
        let mut control = control_con_historico_y_ascensores(historico, [7, -2, 3], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([6, -2, 4]));
    }

    #[test]
    fn con_mas_ascensores_libres_que_plantas_con_llamadas_los_sobrantes_no_se_mueven() {
        let mut control = control_con_historico_y_ascensores(
            llamadas_de_un_martes_a_las_diez_desde(3, 2),
            [0, 1, -2],
            &[],
        );
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([0, 3, -2]));
    }

    #[test]
    fn la_optimizacion_no_mueve_los_ascensores_ocupados() {
        let mut historico = llamadas_de_un_martes_a_las_diez_desde(5, 2);
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 1));
        let mut control = control_con_historico_y_ascensores(historico, [0, 0, 0], &[1]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([5, 0, 2]));
    }

    #[test]
    fn la_optimizacion_solo_cuenta_las_llamadas_del_mismo_dia_de_la_semana() {
        let un_lunes = |dia| FechaYHora::con(2026, 9, dia, 10, 30, 0).unwrap();
        let mut historico: Vec<Movimiento> = [7, 14, 21]
            .iter()
            .map(|dia| llamada_desde(5, un_lunes(*dia)))
            .collect();
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 1));
        let mut control = control_con_historico_y_ascensores(historico, [0, 0, 0], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([2, 0, 0]));
    }

    #[test]
    fn la_optimizacion_solo_cuenta_las_llamadas_de_la_misma_franja_horaria() {
        let a_las_once = |dia| FechaYHora::con(2026, 9, dia, 11, 0, 0).unwrap();
        let mut historico: Vec<Movimiento> = [1, 8, 15]
            .iter()
            .map(|dia| llamada_desde(5, a_las_once(*dia)))
            .collect();
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 1));
        let mut control = control_con_historico_y_ascensores(historico, [0, 0, 0], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([2, 0, 0]));
    }

    #[test]
    fn la_optimizacion_ignora_las_llamadas_de_hace_mas_de_treinta_dias() {
        let hace_treinta_y_cinco_dias = FechaYHora::con(2026, 8, 25, 10, 30, 0).unwrap();
        let mut historico: Vec<Movimiento> = (0..3)
            .map(|_| llamada_desde(5, hace_treinta_y_cinco_dias))
            .collect();
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 1));
        let mut control = control_con_historico_y_ascensores(historico, [0, 0, 0], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([2, 0, 0]));
    }

    #[test]
    fn la_optimizacion_solo_cuenta_los_movimientos_hechos_para_atender_llamadas() {
        let fecha_y_hora = FechaYHora::con(2026, 9, 22, 10, 30, 0).unwrap();
        let mut historico = vec![];
        for _ in 0..3 {
            historico.push(movimiento_con_motivo(
                5,
                fecha_y_hora,
                MotivoDelMovimiento::LlevarAlUsuarioASuDestino,
            ));
            historico.push(movimiento_con_motivo(
                5,
                fecha_y_hora,
                MotivoDelMovimiento::ReubicarUnAscensorLibre,
            ));
        }
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 1));
        let mut control = control_con_historico_y_ascensores(historico, [0, 0, 0], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        assert_eq!(plantas_actuales(&control), plantas([2, 0, 0]));
    }

    #[test]
    fn la_optimizacion_registra_cada_reubicacion_en_el_historico_con_ese_motivo() {
        let mut historico = llamadas_de_un_martes_a_las_diez_desde(5, 2);
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 1));
        let previos = historico.len();
        let mut control = control_con_historico_y_ascensores(historico, [0, 0, 0], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        let nuevos = movimientos_registrados(&control).split_off(previos);
        let esperados = vec![
            Movimiento {
                ascensor: ascensor(0),
                planta_de_origen: planta(0),
                planta_de_destino: planta(5),
                fecha_y_hora: el_martes_a_las_diez(),
                motivo: MotivoDelMovimiento::ReubicarUnAscensorLibre,
            },
            Movimiento {
                ascensor: ascensor(1),
                planta_de_origen: planta(0),
                planta_de_destino: planta(2),
                fecha_y_hora: el_martes_a_las_diez(),
                motivo: MotivoDelMovimiento::ReubicarUnAscensorLibre,
            },
        ];
        assert_eq!(nuevos, esperados);
    }

    #[test]
    fn los_ascensores_reubicados_por_la_optimizacion_siguen_libres() {
        let mut historico = llamadas_de_un_martes_a_las_diez_desde(5, 2);
        historico.extend(llamadas_de_un_martes_a_las_diez_desde(2, 1));
        let mut control = control_con_historico_y_ascensores(historico, [0, 0, 0], &[]);
        control.optimizar_posicion_de_ascensores_libres().unwrap();
        for identificador in control.edificio().identificadores_de_ascensores() {
            assert_eq!(control.edificio().esta_libre(identificador), Ok(true));
        }
    }
}
