use {
    anchor_lang::prelude::*,
    crate::instrucciones::*,
};

mod collections;
mod instrucciones;


declare_id!("9hTWVDR6UAWR14cD8AWtfozaFH6Qc7bZZuWKvA2zHkZt");

#[program]
pub mod gestion_de_eventos {
    use super::*;
    //Crear evento
    pub fn crear_evento(
        ctx: Context<CrearEvento>,
        nombre: String,
        precio: u64,
        close_sales: i64
    ) -> Result<()> {
        instrucciones::crear_evento::handle(ctx, nombre, precio, close_sales)
    }
    // Sponsor event
    pub fn sponsor_event (
        ctx: Context<Sponsor>,
        quantity: u64,
    ) -> Result<()> {
        instrucciones::sponsor::handle(ctx, quantity)
    }
    
}
