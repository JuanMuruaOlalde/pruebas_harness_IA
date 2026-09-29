use std::fs::OpenOptions;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

use crate::dominio::edificio::IdentificadorDeAscensor;
use crate::dominio::fecha_y_hora::FechaYHora;
use crate::dominio::historico_de_movimientos::{ErrorDeHistorico, HistoricoDeMovimientos};
use crate::dominio::movimiento::{MotivoDelMovimiento, Movimiento};
use crate::dominio::planta::Planta;

const SEPARADOR_DE_CAMPOS: &str = ";";

/// Histórico permanente: cada movimiento se añade como una línea al final de un archivo de texto
/// y nunca se borra ninguno. El archivo se crea al registrar el primer movimiento.
///
/// Formato de cada línea:
/// `ascensor;planta de origen;planta de destino;AAAA-MM-DD HH:MM:SS;motivo`
pub struct HistoricoEnArchivo {
    ruta_del_archivo: PathBuf,
}

impl HistoricoEnArchivo {
    /// Abre el histórico guardado en el archivo. Si el archivo no existe, empieza vacío.
    /// Si existe pero su contenido no es válido, devuelve error.
    pub fn abrir(ruta_del_archivo: &Path) -> Result<Self, ErrorDeHistorico> {
        let historico = Self {
            ruta_del_archivo: ruta_del_archivo.to_path_buf(),
        };
        historico.movimientos()?;
        Ok(historico)
    }
}

impl HistoricoDeMovimientos for HistoricoEnArchivo {
    fn registrar(&mut self, movimiento: Movimiento) -> Result<(), ErrorDeHistorico> {
        let mut archivo = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.ruta_del_archivo)
            .map_err(|_| ErrorDeHistorico::NoSePudoAccederAlAlmacenamiento)?;
        writeln!(archivo, "{}", movimiento_como_linea(&movimiento))
            .map_err(|_| ErrorDeHistorico::NoSePudoAccederAlAlmacenamiento)
    }

    fn movimientos(&self) -> Result<Vec<Movimiento>, ErrorDeHistorico> {
        let contenido = match std::fs::read_to_string(&self.ruta_del_archivo) {
            Ok(contenido) => contenido,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(vec![]),
            Err(error) if error.kind() == ErrorKind::InvalidData => {
                return Err(ErrorDeHistorico::ContenidoNoValido);
            }
            Err(_) => return Err(ErrorDeHistorico::NoSePudoAccederAlAlmacenamiento),
        };
        contenido
            .lines()
            .map(|linea| movimiento_desde_linea(linea).ok_or(ErrorDeHistorico::ContenidoNoValido))
            .collect()
    }
}

fn movimiento_como_linea(movimiento: &Movimiento) -> String {
    [
        movimiento.ascensor.numero().to_string(),
        movimiento.planta_de_origen.numero().to_string(),
        movimiento.planta_de_destino.numero().to_string(),
        fecha_y_hora_como_texto(movimiento.fecha_y_hora),
        motivo_como_texto(movimiento.motivo).to_string(),
    ]
    .join(SEPARADOR_DE_CAMPOS)
}

fn movimiento_desde_linea(linea: &str) -> Option<Movimiento> {
    let campos: Vec<&str> = linea.split(SEPARADOR_DE_CAMPOS).collect();
    let [
        texto_del_ascensor,
        texto_de_la_planta_de_origen,
        texto_de_la_planta_de_destino,
        texto_de_la_fecha_y_hora,
        texto_del_motivo,
    ] = campos[..]
    else {
        return None;
    };
    Some(Movimiento {
        ascensor: IdentificadorDeAscensor::con_numero(texto_del_ascensor.parse().ok()?),
        planta_de_origen: Planta::con_numero(texto_de_la_planta_de_origen.parse().ok()?),
        planta_de_destino: Planta::con_numero(texto_de_la_planta_de_destino.parse().ok()?),
        fecha_y_hora: fecha_y_hora_desde_texto(texto_de_la_fecha_y_hora)?,
        motivo: motivo_desde_texto(texto_del_motivo)?,
    })
}

/// Formato `AAAA-MM-DD HH:MM:SS`.
fn fecha_y_hora_como_texto(fecha_y_hora: FechaYHora) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        fecha_y_hora.anio(),
        fecha_y_hora.mes(),
        fecha_y_hora.dia(),
        fecha_y_hora.hora(),
        fecha_y_hora.minuto(),
        fecha_y_hora.segundo(),
    )
}

fn fecha_y_hora_desde_texto(texto: &str) -> Option<FechaYHora> {
    let (texto_de_la_fecha, texto_de_la_hora) = texto.split_once(' ')?;
    let [anio, mes, dia] = texto_de_la_fecha.split('-').collect::<Vec<_>>()[..] else {
        return None;
    };
    let [horas, minutos, segundos] = texto_de_la_hora.split(':').collect::<Vec<_>>()[..] else {
        return None;
    };
    FechaYHora::con(
        anio.parse().ok()?,
        mes.parse().ok()?,
        dia.parse().ok()?,
        horas.parse().ok()?,
        minutos.parse().ok()?,
        segundos.parse().ok()?,
    )
    .ok()
}

fn motivo_como_texto(motivo: MotivoDelMovimiento) -> &'static str {
    match motivo {
        MotivoDelMovimiento::AtenderUnaLlamada => "atender_una_llamada",
        MotivoDelMovimiento::LlevarAlUsuarioASuDestino => "llevar_al_usuario_a_su_destino",
        MotivoDelMovimiento::ReubicarUnAscensorLibre => "reubicar_un_ascensor_libre",
    }
}

fn motivo_desde_texto(texto: &str) -> Option<MotivoDelMovimiento> {
    match texto {
        "atender_una_llamada" => Some(MotivoDelMovimiento::AtenderUnaLlamada),
        "llevar_al_usuario_a_su_destino" => Some(MotivoDelMovimiento::LlevarAlUsuarioASuDestino),
        "reubicar_un_ascensor_libre" => Some(MotivoDelMovimiento::ReubicarUnAscensorLibre),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Ruta única bajo el directorio temporal; el archivo se borra al soltar el valor.
    struct ArchivoTemporal(PathBuf);

    impl ArchivoTemporal {
        fn nuevo() -> Self {
            static CONTADOR: AtomicUsize = AtomicUsize::new(0);
            let numero = CONTADOR.fetch_add(1, Ordering::SeqCst);
            let nombre = format!("historico_ascensores_{}_{}.txt", std::process::id(), numero);
            Self(std::env::temp_dir().join(nombre))
        }

        fn ruta(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for ArchivoTemporal {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    fn movimiento(
        ascensor: usize,
        origen: i8,
        destino: i8,
        fecha_y_hora: FechaYHora,
        motivo: MotivoDelMovimiento,
    ) -> Movimiento {
        Movimiento {
            ascensor: IdentificadorDeAscensor::con_numero(ascensor),
            planta_de_origen: Planta::con_numero(origen),
            planta_de_destino: Planta::con_numero(destino),
            fecha_y_hora,
            motivo,
        }
    }

    fn un_movimiento() -> Movimiento {
        movimiento(
            1,
            0,
            3,
            FechaYHora::con(2026, 9, 29, 10, 15, 30).unwrap(),
            MotivoDelMovimiento::AtenderUnaLlamada,
        )
    }

    #[test]
    fn un_historico_en_archivo_que_aun_no_existe_empieza_vacio() {
        let archivo = ArchivoTemporal::nuevo();
        let historico = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        assert_eq!(historico.movimientos(), Ok(vec![]));
    }

    #[test]
    fn el_historico_en_archivo_conserva_los_movimientos_al_volver_a_abrirlo() {
        let archivo = ArchivoTemporal::nuevo();
        let mut historico = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        historico.registrar(un_movimiento()).unwrap();
        drop(historico);
        let reabierto = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        assert_eq!(reabierto.movimientos(), Ok(vec![un_movimiento()]));
    }

    #[test]
    fn el_historico_en_archivo_anade_los_movimientos_nuevos_sin_perder_los_anteriores() {
        let archivo = ArchivoTemporal::nuevo();
        let mut historico = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        historico.registrar(un_movimiento()).unwrap();
        drop(historico);
        let mut reabierto = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        let otro = movimiento(
            2,
            3,
            -2,
            FechaYHora::con(2026, 9, 29, 11, 0, 0).unwrap(),
            MotivoDelMovimiento::LlevarAlUsuarioASuDestino,
        );
        reabierto.registrar(otro).unwrap();
        assert_eq!(reabierto.movimientos(), Ok(vec![un_movimiento(), otro]));
        let vuelto_a_abrir = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        assert_eq!(
            vuelto_a_abrir.movimientos(),
            Ok(vec![un_movimiento(), otro])
        );
    }

    #[test]
    fn el_historico_en_archivo_conserva_todos_los_datos_de_cada_movimiento() {
        let archivo = ArchivoTemporal::nuevo();
        let mut historico = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        let movimientos = [
            movimiento(
                0,
                -2,
                7,
                FechaYHora::con(2024, 2, 29, 23, 59, 59).unwrap(),
                MotivoDelMovimiento::AtenderUnaLlamada,
            ),
            movimiento(
                2,
                7,
                -2,
                FechaYHora::con(2027, 1, 1, 0, 0, 0).unwrap(),
                MotivoDelMovimiento::LlevarAlUsuarioASuDestino,
            ),
            movimiento(
                1,
                4,
                4,
                FechaYHora::con(2026, 12, 31, 12, 30, 5).unwrap(),
                MotivoDelMovimiento::ReubicarUnAscensorLibre,
            ),
        ];
        for movimiento in movimientos {
            historico.registrar(movimiento).unwrap();
        }
        let reabierto = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        assert_eq!(reabierto.movimientos(), Ok(movimientos.to_vec()));
    }

    #[test]
    fn el_historico_en_archivo_conserva_los_movimientos_de_hace_mas_de_un_mes() {
        let archivo = ArchivoTemporal::nuevo();
        let mut historico = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        let antiguo = movimiento(
            0,
            0,
            1,
            FechaYHora::con(2020, 1, 1, 8, 0, 0).unwrap(),
            MotivoDelMovimiento::AtenderUnaLlamada,
        );
        historico.registrar(antiguo).unwrap();
        historico.registrar(un_movimiento()).unwrap();
        let reabierto = HistoricoEnArchivo::abrir(archivo.ruta()).unwrap();
        assert_eq!(reabierto.movimientos(), Ok(vec![antiguo, un_movimiento()]));
    }

    #[test]
    fn abrir_un_historico_en_archivo_con_contenido_no_valido_devuelve_error() {
        let archivo = ArchivoTemporal::nuevo();
        std::fs::write(archivo.ruta(), "esto no es un movimiento\n").unwrap();
        assert_eq!(
            HistoricoEnArchivo::abrir(archivo.ruta()).err(),
            Some(ErrorDeHistorico::ContenidoNoValido)
        );
    }
}
