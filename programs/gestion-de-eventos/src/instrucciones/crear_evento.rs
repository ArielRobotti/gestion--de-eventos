use {
    crate::collections::Evento,
    anchor_lang::prelude::*,
    anchor_spl::token::*,
};

#[derive(Accounts)]
pub struct CrearEvento<'info> {
    #[account(
        init,
        seeds = [
            Evento::SEED_EVENT.as_ref(),
            authority.key().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + Evento::INIT_SPACE,

    )]
    pub evento: Account<'info, Evento>,

    pub accepted_mint: Account<'info,Mint>,

    #[account(
        init,
        seeds = [
            Evento::SEED_EVENT_MINT.as_ref(),
            evento.key().as_ref()
        ],
        bump,
        payer = authority,
        mint::decimals = 0,
        mint::authority = evento,
    )]
    pub event_mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [
            Evento::SEED_TREASURY_VAULT.as_ref(),
            evento.key().as_ref()
        ],
        bump,
        payer = authority,
        token::mint = accepted_mint,
        token::authority = evento,
    )]
    pub treasury_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [
            Evento::SEED_GAIN_VAULT.as_ref(),
            evento.key().as_ref()
        ],
        bump,
        payer = authority,
        token::mint = accepted_mint,
        token::authority = evento,
    )]
    pub gain_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

}

pub fn handle(ctx: Context<CrearEvento>, nombre: String, precio: u64, close_sales: i64) -> Result<()> {
    ctx.accounts.evento.name = nombre;
    ctx.accounts.evento.ticket_price = precio;
    ctx.accounts.evento.open_sales = true;
    ctx.accounts.evento.timestamp_event_close = close_sales;

    ctx.accounts.evento.authority = ctx.accounts.authority.key();
    ctx.accounts.evento.accepted_mint = ctx.accounts.accepted_mint.key();

    ctx.accounts.evento.event_bump = ctx.bumps.evento;
    ctx.accounts.evento.event_mint_bump = ctx.bumps.event_mint;
    ctx.accounts.evento.treasury_vault_bump = ctx.bumps.treasury_vault;
    ctx.accounts.evento.gain_vault_bump = ctx.bumps.gain_vault;

    Ok(())
}


