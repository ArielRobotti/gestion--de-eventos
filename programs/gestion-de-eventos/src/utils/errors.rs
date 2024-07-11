use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("La venta de tokens está cerrada.")]
    VentasCerradas,
    #[msg("El periodo de ventas acaba de finalizar y las ventas se encuentran cerradas")]
    SeCierranLasVentas,
    #[msg("La venta de tickets esta cerrada")]
    EventoCerrado,

    MintNoCoincide,
    FondosInsuficientes,
    TreasuryVaultError,
    Unauthorized
}