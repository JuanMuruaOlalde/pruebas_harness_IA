use std::path::Path;
use std::process::ExitCode;

use pruebas_harness::adaptadores::historico_en_archivo::HistoricoEnArchivo;
use pruebas_harness::adaptadores::interfaz_grafica::panel_del_simulador::PanelDelSimulador;
use pruebas_harness::adaptadores::interfaz_grafica::ventana_del_simulador::{
    TITULO_DE_LA_VENTANA, VentanaDelSimulador,
};
use pruebas_harness::adaptadores::reloj_del_sistema::RelojDelSistema;
use pruebas_harness::dominio::control_de_trafico::ControlDeTrafico;
use pruebas_harness::dominio::edificio::Edificio;

const RUTA_DEL_ARCHIVO_DEL_HISTORICO: &str = "historico_de_movimientos.txt";

fn main() -> ExitCode {
    let historico = match HistoricoEnArchivo::abrir(Path::new(RUTA_DEL_ARCHIVO_DEL_HISTORICO)) {
        Ok(historico) => historico,
        Err(error) => {
            eprintln!(
                "No se pudo abrir el histórico de movimientos ({RUTA_DEL_ARCHIVO_DEL_HISTORICO}): {error:?}"
            );
            return ExitCode::FAILURE;
        }
    };
    let control_de_trafico = ControlDeTrafico::new(
        Edificio::new(),
        Box::new(RelojDelSistema),
        Box::new(historico),
    );
    abrir_la_ventana_del_simulador(control_de_trafico)
}

/// Abre la ventana y no vuelve hasta que el usuario la cierra.
fn abrir_la_ventana_del_simulador(control_de_trafico: ControlDeTrafico) -> ExitCode {
    let ventana = VentanaDelSimulador::new(PanelDelSimulador::new(control_de_trafico));
    let resultado_de_la_ventana = eframe::run_native(
        TITULO_DE_LA_VENTANA,
        eframe::NativeOptions::default(),
        Box::new(|_contexto_de_creacion| Ok(Box::new(ventana))),
    );
    match resultado_de_la_ventana {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("No se pudo abrir la ventana: {error}");
            ExitCode::FAILURE
        }
    }
}
