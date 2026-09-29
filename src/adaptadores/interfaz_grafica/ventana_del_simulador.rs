use eframe::egui;

use super::panel_del_simulador::PanelDelSimulador;
use crate::dominio::planta::Planta;

pub const TITULO_DE_LA_VENTANA: &str = "Simulador de ascensores";

/// Ventana egui del simulador: dibuja el panel y le traslada las pulsaciones del usuario.
pub struct VentanaDelSimulador {
    panel: PanelDelSimulador,
}

impl VentanaDelSimulador {
    pub fn new(panel: PanelDelSimulador) -> Self {
        Self { panel }
    }

    fn dibujar(&mut self, ui: &mut egui::Ui) {
        ui.heading(TITULO_DE_LA_VENTANA);
        // La pulsación se traslada al panel después de dibujar todas las plantas, porque mientras
        // se dibujan se está consultando el panel.
        if let Some(planta_de_la_llamada) = self.dibujar_las_plantas(ui) {
            self.panel
                .pulsar_el_boton_de_llamada_en_la_planta(planta_de_la_llamada);
        }
        ui.separator();
        self.dibujar_el_ultimo_aviso(ui);
    }

    /// Dibuja una fila por planta y devuelve la planta cuyo botón de llamada se ha pulsado, si
    /// se ha pulsado alguno.
    fn dibujar_las_plantas(&self, ui: &mut egui::Ui) -> Option<Planta> {
        let mut planta_con_el_boton_de_llamada_pulsado = None;
        for planta in self.panel.plantas_de_la_mas_alta_a_la_mas_baja() {
            if self.dibujar_la_fila_de_la_planta(ui, planta) {
                planta_con_el_boton_de_llamada_pulsado = Some(planta);
            }
        }
        planta_con_el_boton_de_llamada_pulsado
    }

    /// Dibuja la planta, su botón de llamada y los ascensores que están en ella. Devuelve si se
    /// ha pulsado el botón de llamada.
    fn dibujar_la_fila_de_la_planta(&self, ui: &mut egui::Ui, planta: Planta) -> bool {
        ui.horizontal(|ui| {
            ui.label(format!("Planta {:>2}", planta.numero()));
            let se_ha_pulsado_el_boton_de_llamada = ui.button("Llamar").clicked();
            for ascensor in self.panel.ascensores_en_la_planta(planta) {
                ui.label(format!("[Ascensor {}]", ascensor.numero()));
            }
            se_ha_pulsado_el_boton_de_llamada
        })
        .inner
    }

    fn dibujar_el_ultimo_aviso(&self, ui: &mut egui::Ui) {
        match self.panel.ultimo_aviso() {
            Some(aviso) => ui.label(aviso.como_texto()),
            None => ui.label("Sin avisos"),
        };
    }
}

impl eframe::App for VentanaDelSimulador {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.dibujar(ui);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adaptadores::historico_en_memoria::HistoricoEnMemoria;
    use crate::adaptadores::reloj_del_sistema::RelojDelSistema;
    use crate::dominio::control_de_trafico::ControlDeTrafico;
    use crate::dominio::edificio::Edificio;

    #[test]
    fn la_ventana_del_simulador_se_dibuja_con_egui_sin_pantalla_y_sin_fallar() {
        let control_de_trafico = ControlDeTrafico::new(
            Edificio::new(),
            Box::new(RelojDelSistema),
            Box::new(HistoricoEnMemoria::new()),
        );
        let mut ventana = VentanaDelSimulador::new(PanelDelSimulador::new(control_de_trafico));
        let contexto_de_egui = egui::Context::default();
        let mut salida_de_egui =
            contexto_de_egui.run_ui(egui::RawInput::default(), |ui| ventana.dibujar(ui));
        assert!(!salida_de_egui.shapes.is_empty());
        // Sin pantalla nadie aplica las texturas; egui exige descartarlas a mano antes de soltarlas.
        salida_de_egui.textures_delta.clear();
    }
}
