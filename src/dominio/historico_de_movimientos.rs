use super::movimiento::Movimiento;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorDeHistorico {
    NoSePudoAccederAlAlmacenamiento,
    ContenidoNoValido,
}

/// Puerto del histórico permanente de movimientos. Se conservan en el orden en que se produjeron.
pub trait HistoricoDeMovimientos {
    fn registrar(&mut self, movimiento: Movimiento) -> Result<(), ErrorDeHistorico>;
    fn movimientos(&self) -> Result<Vec<Movimiento>, ErrorDeHistorico>;
}
