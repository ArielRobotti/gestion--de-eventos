use {
    anchor_lang::prelude::*,
    crate::instrucciones::*,
};

mod collections;
mod instrucciones;
mod utils;


declare_id!("9hTWVDR6UAWR14cD8AWtfozaFH6Qc7bZZuWKvA2zHkZt");

#[program]
pub mod gestion_de_eventos {
    use super::*;
    //Crear evento
    pub fn crear_evento(
        ctx: Context<CrearEvento>,
        nombre: String,
        precio: u64,
        open_sales: bool,
        close_presales_date: i64,
        close_sales_date: i64
    ) -> Result<()> {
        instrucciones::crear_evento::handle(ctx, nombre, precio, open_sales, close_presales_date, close_sales_date)
    }
    // Sponsor event
    pub fn sponsor_event ( ctx: Context<Sponsor>, quantity: u64 ) -> Result<()> {
        instrucciones::sponsor::handle(ctx, quantity)
    }
    // Verificar fecha de cierre y cerrar evento si es necesario
    // fn (timestamp: i64) {

    // }
    // comprar Tickets
    pub fn comprar_tickets ( ctx: Context<ComprarTickets>, quantity: u64 ) -> Result<()> {
        let fuera_de_fecha = ctx.accounts.evento.timestamp_event_close < Clock::get()?.unix_timestamp;
        if fuera_de_fecha {    
            let _ = cerrar_evento::handle();
        }
        instrucciones::comprar_tickets::handle(ctx, quantity)
    }
    // Withdraw
    pub fn withdraw ( ctx: Context<WithdrawFunds>, quantity: u64 ) -> Result<()> {
        instrucciones::withdraw_funds::handle(ctx, quantity)
    }

    // Cerrar evento
    pub fn cerrar_evento (ctx: Context<CerrarEvento>) -> Result<()> {
        instrucciones::cerrar_evento::handle(ctx)
    }

    // Withdraw earnings
    pub fn withdraw_earnings (ctx: Context<WithdrawEarnings>) -> Result<()> {
        instrucciones::withdraw_earnings::handle(ctx)
    }

    

}
