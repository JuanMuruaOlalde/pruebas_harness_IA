use crate::dominio::historico_de_movimientos::{ErrorDeHistorico, HistoricoDeMovimientos};
use crate::dominio::movimiento::Movimiento;

/// Histórico que solo vive mientras el programa está en ejecución.
#[derive(Default)]
pub struct HistoricoEnMemoria {
    movimientos: Vec<Movimiento>,
}

impl HistoricoEnMemoria {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HistoricoDeMovimientos for HistoricoEnMemoria {
    fn registrar(&mut self, movimiento: Movimiento) -> Result<(), ErrorDeHistorico> {
        self.movimientos.push(movimiento);
        Ok(())
    }

    fn movimientos(&self) -> Result<Vec<Movimiento>, ErrorDeHistorico> {
        Ok(self.movimientos.clone())
    }
}
