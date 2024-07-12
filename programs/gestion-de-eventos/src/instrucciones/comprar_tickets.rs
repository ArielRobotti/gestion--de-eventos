use {
    crate::collections::Evento,
    crate::utils::errors::ErrorCode,
    anchor_lang::prelude::*, 
    anchor_spl::token::*
};

#[derive(Accounts)]
pub struct ComprarTickets<'info> {
    #[account(
        mut,
        // check if the event is still active
        seeds = [
            Evento::SEED_EVENT.as_ref(), // "event" seed
            // checks only authority provided can withdraw
            evento.authority.key().as_ref() // authority public key
        ],
        bump = evento.event_bump,
        constraint = evento.open_sales == true @ ErrorCode::EventoCerrado, 
    )] 
    pub evento: Box<Account<'info, Evento>>, // event account

    // payer accepted mint ATA
    #[account(
        mut,
        // checks the ATA holds the accepted mint
        constraint = payer_accepted_mint_ata.mint == evento.accepted_mint @ ErrorCode::MintNoCoincide,
        constraint = payer_accepted_mint_ata.amount > 0 @ ErrorCode::FondosInsuficientes
    )]
    pub payer_accepted_mint_ata: Box<Account<'info, TokenAccount>>, 

    #[account(
        mut,
        seeds = [
            Evento::SEED_GAIN_VAULT.as_ref(), // "gain_vault" seed
            evento.key().as_ref() // event public key
        ],
        bump = evento.gain_vault_bump,
      )]
    pub gain_vault: Box<Account<'info, TokenAccount>>, // event gain vault account

    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    // Reloj para monitorear la marca de tiempo de cierre de ventas
    pub clock: Sysvar<'info, Clock>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle( ctx: Context<ComprarTickets>, quantity: u64 ) -> Result<()> {

    ////////// Verificacion de fecha de cierre del evento  //////////
    let evento = &mut ctx.accounts.evento;
    
    // if ctx.accounts.clock.unix_timestamp > evento.timestamp_event_close {

    //     // cerrar_evento::handle(ctx_cerrar_evento);
    //     return Err(ErrorCode::SeCierranLasVentas.into()) // deolver un Err con mensaje de ventas cerradas
    // };

    // Cálculo del monto a debitar de la cuenta de tokens de evento
    let mut amount = evento.ticket_price.checked_mul(quantity).unwrap();

    // Verificación de preventa activa. Caso afirmativo reducir a la mitad el amount a debitar
    if ctx.accounts.clock.unix_timestamp < evento.timestamp_presales_close {
        let temp_amount = amount / 2;
        if temp_amount == 0 {
            amount = 1
        } else {
            amount = temp_amount
        } //
    };
    ///////////////////////////////////////////////////////////////////////////////////////////

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer_accepted_mint_ata.to_account_info(), // payer accepted mint ata
                to: ctx.accounts.gain_vault.to_account_info(), // event gain vault
                authority: ctx.accounts.authority.to_account_info(), // payer (authority of the from account)
            },
        ),
        amount, // amount to charge
    )?;
    Ok(())
  }
