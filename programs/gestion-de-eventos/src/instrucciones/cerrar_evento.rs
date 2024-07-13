use {
    crate::collections::Evento,
    crate::utils::errors::ErrorCode,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct CerrarEvento<'info> {

    #[account(
        mut,
        seeds = [
            Evento::SEED_EVENT.as_ref(),
            evento.authority.key().as_ref()
        ],
        bump = evento.event_bump,
        // constraint = evento.authority == authority.key() @ ErrorCode::Unauthorized  
    )]
    pub evento: Account<'info, Evento>,

    //Authoridad del evento
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>

}
pub fn handle(ctx: Context<CerrarEvento>) -> Result<()> {
    let evento = &mut ctx.accounts.evento;
    let fecha_de_cierra_alcanzada = Clock::get()?.unix_timestamp >= evento.timestamp_event_close;

    // El evento podrá ser cerrado si el firmante es la autoridad del Evento o si la fecha de cierre fue alcanzada
    if evento.authority != ctx.accounts.authority.key() || !fecha_de_cierra_alcanzada {
        return Err(ErrorCode::UstedNoPuedeCerrarElEventoAun.into())
    }
    evento.open_sales = false;
    msg!("El evento se encuentra {}", if evento.open_sales { "abierto" } else { "cerrado" });
    Ok(())    
}