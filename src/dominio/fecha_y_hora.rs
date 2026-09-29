const SEGUNDOS_POR_MINUTO: i64 = 60;
const SEGUNDOS_POR_HORA: i64 = 60 * SEGUNDOS_POR_MINUTO;
const SEGUNDOS_POR_DIA: i64 = 24 * SEGUNDOS_POR_HORA;
const DIAS_DEL_ULTIMO_MES: i64 = 30;
/// El calendario gregoriano se repite cada 400 años (una «era» en el algoritmo de H. Hinnant).
const DIAS_POR_ERA_DE_400_ANIOS: i64 = 146_097;
/// El algoritmo de H. Hinnant cuenta los días desde el 1 de marzo del año 0.
const DIAS_DESDE_EL_1_DE_MARZO_DEL_ANIO_0_HASTA_1970: i64 = 719_468;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiaDeLaSemana {
    Lunes,
    Martes,
    Miercoles,
    Jueves,
    Viernes,
    Sabado,
    Domingo,
}

/// Cada una de las 24 franjas de una hora en que se divide el día. La franja 0 va de 00:00 a 00:59.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FranjaHoraria(u8);

impl FranjaHoraria {
    pub fn numero(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorDeFechaYHora {
    FechaYHoraInexistente,
}

/// Fecha y hora (calendario gregoriano, sin zona horaria) con precisión de segundos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FechaYHora {
    anio: i32,
    mes: u8,
    dia: u8,
    hora: u8,
    minuto: u8,
    segundo: u8,
}

impl FechaYHora {
    pub fn con(
        anio: i32,
        mes: u8,
        dia: u8,
        hora: u8,
        minuto: u8,
        segundo: u8,
    ) -> Result<Self, ErrorDeFechaYHora> {
        let es_valida = (1..=12).contains(&mes)
            && (1..=dias_del_mes(anio, mes)).contains(&dia)
            && hora < 24
            && minuto < 60
            && segundo < 60;
        if !es_valida {
            return Err(ErrorDeFechaYHora::FechaYHoraInexistente);
        }
        Ok(Self {
            anio,
            mes,
            dia,
            hora,
            minuto,
            segundo,
        })
    }

    /// Crea la fecha y hora a partir de los segundos transcurridos desde el 1 de enero de 1970 00:00:00.
    pub fn desde_segundos_desde_1970(segundos_desde_1970: i64) -> Self {
        let dias_desde_1970 = segundos_desde_1970.div_euclid(SEGUNDOS_POR_DIA);
        let segundos_del_dia = segundos_desde_1970.rem_euclid(SEGUNDOS_POR_DIA);
        // Algoritmo de calendario civil desde días (H. Hinnant), con años que empiezan en marzo.
        let dias_desde_el_1_de_marzo_del_anio_0 =
            dias_desde_1970 + DIAS_DESDE_EL_1_DE_MARZO_DEL_ANIO_0_HASTA_1970;
        let era = dias_desde_el_1_de_marzo_del_anio_0.div_euclid(DIAS_POR_ERA_DE_400_ANIOS);
        let dia_de_la_era =
            dias_desde_el_1_de_marzo_del_anio_0.rem_euclid(DIAS_POR_ERA_DE_400_ANIOS);
        let anio_de_la_era = (dia_de_la_era - dia_de_la_era / 1460 + dia_de_la_era / 36_524
            - dia_de_la_era / 146_096)
            / 365;
        let dia_del_anio =
            dia_de_la_era - (365 * anio_de_la_era + anio_de_la_era / 4 - anio_de_la_era / 100);
        let mes_desde_marzo = (5 * dia_del_anio + 2) / 153;
        let dia = dia_del_anio - (153 * mes_desde_marzo + 2) / 5 + 1;
        let mes = if mes_desde_marzo < 10 {
            mes_desde_marzo + 3
        } else {
            mes_desde_marzo - 9
        };
        let anio = anio_de_la_era + era * 400 + i64::from(mes <= 2);
        Self {
            anio: anio as i32,
            mes: mes as u8,
            dia: dia as u8,
            hora: (segundos_del_dia / SEGUNDOS_POR_HORA) as u8,
            minuto: (segundos_del_dia % SEGUNDOS_POR_HORA / SEGUNDOS_POR_MINUTO) as u8,
            segundo: (segundos_del_dia % SEGUNDOS_POR_MINUTO) as u8,
        }
    }

    pub fn anio(&self) -> i32 {
        self.anio
    }

    pub fn mes(&self) -> u8 {
        self.mes
    }

    pub fn dia(&self) -> u8 {
        self.dia
    }

    pub fn hora(&self) -> u8 {
        self.hora
    }

    pub fn minuto(&self) -> u8 {
        self.minuto
    }

    pub fn segundo(&self) -> u8 {
        self.segundo
    }

    pub fn dia_de_la_semana(&self) -> DiaDeLaSemana {
        // El 1 de enero de 1970 (día 0) fue jueves.
        const DIAS_DE_LA_SEMANA_EMPEZANDO_POR_EL_JUEVES: [DiaDeLaSemana; 7] = [
            DiaDeLaSemana::Jueves,
            DiaDeLaSemana::Viernes,
            DiaDeLaSemana::Sabado,
            DiaDeLaSemana::Domingo,
            DiaDeLaSemana::Lunes,
            DiaDeLaSemana::Martes,
            DiaDeLaSemana::Miercoles,
        ];
        DIAS_DE_LA_SEMANA_EMPEZANDO_POR_EL_JUEVES[self.dias_desde_1970().rem_euclid(7) as usize]
    }

    pub fn franja_horaria(&self) -> FranjaHoraria {
        FranjaHoraria(self.hora)
    }

    /// Indica si esta fecha y hora está en los 30 días anteriores a `ahora`, ambos límites incluidos.
    pub fn pertenece_al_ultimo_mes_visto_desde(&self, ahora: FechaYHora) -> bool {
        let segundos_transcurridos = ahora.segundos_desde_1970() - self.segundos_desde_1970();
        (0..=DIAS_DEL_ULTIMO_MES * SEGUNDOS_POR_DIA).contains(&segundos_transcurridos)
    }

    fn dias_desde_1970(&self) -> i64 {
        // Algoritmo de días desde el calendario civil (H. Hinnant), con años que empiezan en marzo.
        let anio = i64::from(self.anio) - i64::from(self.mes <= 2);
        let era = anio.div_euclid(400);
        let anio_de_la_era = anio.rem_euclid(400);
        let mes_desde_marzo = (i64::from(self.mes) + 9) % 12;
        let dia_del_anio = (153 * mes_desde_marzo + 2) / 5 + i64::from(self.dia) - 1;
        let dia_de_la_era =
            anio_de_la_era * 365 + anio_de_la_era / 4 - anio_de_la_era / 100 + dia_del_anio;
        era * DIAS_POR_ERA_DE_400_ANIOS + dia_de_la_era
            - DIAS_DESDE_EL_1_DE_MARZO_DEL_ANIO_0_HASTA_1970
    }

    fn segundos_desde_1970(&self) -> i64 {
        self.dias_desde_1970() * SEGUNDOS_POR_DIA
            + i64::from(self.hora) * SEGUNDOS_POR_HORA
            + i64::from(self.minuto) * SEGUNDOS_POR_MINUTO
            + i64::from(self.segundo)
    }
}

fn es_bisiesto(anio: i32) -> bool {
    (anio % 4 == 0 && anio % 100 != 0) || anio % 400 == 0
}

fn dias_del_mes(anio: i32, mes: u8) -> u8 {
    match mes {
        2 if es_bisiesto(anio) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fecha_y_hora(anio: i32, mes: u8, dia: u8, hora: u8, minuto: u8) -> FechaYHora {
        FechaYHora::con(anio, mes, dia, hora, minuto, 0).unwrap()
    }

    #[test]
    fn una_fecha_y_hora_conoce_su_dia_de_la_semana() {
        assert_eq!(
            fecha_y_hora(2026, 9, 29, 10, 0).dia_de_la_semana(),
            DiaDeLaSemana::Martes
        );
        assert_eq!(
            fecha_y_hora(1970, 1, 1, 0, 0).dia_de_la_semana(),
            DiaDeLaSemana::Jueves
        );
        assert_eq!(
            fecha_y_hora(2024, 2, 29, 23, 59).dia_de_la_semana(),
            DiaDeLaSemana::Jueves
        );
        assert_eq!(
            fecha_y_hora(2026, 10, 4, 12, 0).dia_de_la_semana(),
            DiaDeLaSemana::Domingo
        );
    }

    #[test]
    fn una_fecha_y_hora_conoce_su_franja_horaria() {
        assert_eq!(fecha_y_hora(2026, 9, 29, 0, 0).franja_horaria().numero(), 0);
        assert_eq!(
            fecha_y_hora(2026, 9, 29, 8, 59).franja_horaria().numero(),
            8
        );
        assert_eq!(fecha_y_hora(2026, 9, 29, 9, 0).franja_horaria().numero(), 9);
        assert_eq!(
            fecha_y_hora(2026, 9, 29, 23, 59).franja_horaria().numero(),
            23
        );
    }

    #[test]
    fn crear_una_fecha_y_hora_inexistente_devuelve_error() {
        let error = Err(ErrorDeFechaYHora::FechaYHoraInexistente);
        assert_eq!(FechaYHora::con(2026, 13, 1, 0, 0, 0), error);
        assert_eq!(FechaYHora::con(2026, 0, 1, 0, 0, 0), error);
        assert_eq!(FechaYHora::con(2026, 2, 29, 0, 0, 0), error);
        assert_eq!(FechaYHora::con(2026, 4, 31, 0, 0, 0), error);
        assert_eq!(FechaYHora::con(2026, 4, 0, 0, 0, 0), error);
        assert_eq!(FechaYHora::con(2026, 4, 1, 24, 0, 0), error);
        assert_eq!(FechaYHora::con(2026, 4, 1, 0, 60, 0), error);
        assert_eq!(FechaYHora::con(2026, 4, 1, 0, 0, 60), error);
        assert!(FechaYHora::con(2024, 2, 29, 0, 0, 0).is_ok());
    }

    #[test]
    fn una_fecha_y_hora_de_hace_menos_de_treinta_dias_pertenece_al_ultimo_mes() {
        let ahora = fecha_y_hora(2026, 9, 29, 10, 0);
        assert!(fecha_y_hora(2026, 9, 1, 10, 0).pertenece_al_ultimo_mes_visto_desde(ahora));
        assert!(ahora.pertenece_al_ultimo_mes_visto_desde(ahora));
    }

    #[test]
    fn una_fecha_y_hora_de_hace_exactamente_treinta_dias_pertenece_al_ultimo_mes() {
        let ahora = fecha_y_hora(2026, 9, 29, 10, 0);
        assert!(fecha_y_hora(2026, 8, 30, 10, 0).pertenece_al_ultimo_mes_visto_desde(ahora));
    }

    #[test]
    fn una_fecha_y_hora_de_hace_mas_de_treinta_dias_no_pertenece_al_ultimo_mes() {
        let ahora = fecha_y_hora(2026, 9, 29, 10, 0);
        assert!(!fecha_y_hora(2026, 8, 30, 9, 59).pertenece_al_ultimo_mes_visto_desde(ahora));
        assert!(!fecha_y_hora(2026, 7, 1, 10, 0).pertenece_al_ultimo_mes_visto_desde(ahora));
    }

    #[test]
    fn una_fecha_y_hora_de_diciembre_puede_pertenecer_al_ultimo_mes_visto_desde_enero() {
        let ahora = fecha_y_hora(2027, 1, 5, 10, 0);
        assert!(fecha_y_hora(2026, 12, 20, 10, 0).pertenece_al_ultimo_mes_visto_desde(ahora));
        assert!(!fecha_y_hora(2026, 12, 5, 9, 0).pertenece_al_ultimo_mes_visto_desde(ahora));
    }

    #[test]
    fn una_fecha_y_hora_se_puede_crear_a_partir_de_los_segundos_transcurridos_desde_1970() {
        assert_eq!(
            FechaYHora::desde_segundos_desde_1970(0),
            fecha_y_hora(1970, 1, 1, 0, 0)
        );
        assert_eq!(
            FechaYHora::desde_segundos_desde_1970(1_782_724_845),
            FechaYHora::con(2026, 6, 29, 9, 20, 45).unwrap()
        );
        assert_eq!(
            FechaYHora::desde_segundos_desde_1970(-1),
            FechaYHora::con(1969, 12, 31, 23, 59, 59).unwrap()
        );
        let fecha = FechaYHora::con(2024, 2, 29, 23, 59, 59).unwrap();
        assert_eq!(
            FechaYHora::desde_segundos_desde_1970(fecha.segundos_desde_1970()),
            fecha
        );
    }
}
