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
        constraint = evento.authority == authority.key() @ ErrorCode::Unauthorized  
    )]
    pub evento: Box<Account<'info, Evento>>,

    //Authoridad del evento
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>

}
pub fn handle(ctx: Context<CerrarEvento>) -> Result<()> {
    let  evento = &mut ctx.accounts.evento;
    evento.open_sales = false;
    msg!("El evento se encuentra {}", if evento.open_sales { "abierto" } else { "cerrado" });
    Ok(())    
}