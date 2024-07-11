use {
    crate::collections::Evento,
    crate::utils::errors::ErrorCode,
    anchor_lang::prelude::*,
    anchor_spl::{
        token::*,
        associated_token::*,
    },
};

#[derive(Accounts)]
pub struct Sponsor<'info> {
    #[account(
        mut,
        seeds = [
            Evento::SEED_EVENT.as_ref(),
            evento.authority.key().as_ref()
        ],
        bump = evento.event_bump,
        constraint = evento.open_sales == true @ ErrorCode::VentasCerradas,
    )]
    pub evento: Box<Account<'info, Evento>>,

    #[account(
        mut,
        seeds = [
            Evento::SEED_EVENT_MINT.as_ref(),
            evento.key().as_ref(),
        ],
        bump = evento.event_mint_bump,
    )]
    pub event_mint: Box<Account<'info, Mint>>,

    //payer accepted mint
    #[account(
        mut,
        constraint = payer_accepted_mint_ata.mint == evento.accepted_mint,
        constraint = payer_accepted_mint_ata.amount > 0
    )]
    pub payer_accepted_mint_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed, // create account if doesn't exist
        payer = authority, 
        associated_token::mint = event_mint, // event mint
        associated_token::authority = authority, // token account authority
    )]
    pub payer_event_mint_ata: Box<Account<'info, TokenAccount>>, // payer event mint ATA

    #[account(
        mut,
        seeds = [
            Evento::SEED_TREASURY_VAULT.as_ref(), // "treasury_event" seed
            evento.key().as_ref() // event public key
        ],
        bump = evento.treasury_vault_bump,
      )]
    pub treasury_vault: Box<Account<'info, TokenAccount>>, // event treasury token account

    #[account(mut)]
    pub authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    // Reloj para monitorear la marca de tiempo de cierre de ventas
    pub clock: Sysvar<'info, Clock>,
    // programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    
}
#[event]
pub struct EventSalesClosed {
    pub event: Pubkey,
    pub timestamp: i64,
}

pub fn handle( ctx: Context<Sponsor>, quantity: u64) -> Result<()> {

    let evento = &mut ctx.accounts.evento;
    if ctx.accounts.clock.unix_timestamp > evento.timestamp_event_close {   
        evento.open_sales = false; 
        return Err(ErrorCode::SeCierranLasVentas.into())
    }
    
    let seeds = [
        Evento::SEED_EVENT.as_bytes(),
        ctx.accounts.evento.authority.as_ref(),
        &[ctx.accounts.evento.event_bump],
    ];
    let signer = &[&seeds[..]];
    // Charge the accepted_token amount from payer
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.payer_accepted_mint_ata.to_account_info(),
                to: ctx.accounts.treasury_vault.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        quantity,
    )?;
    // Transfer the token
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.event_mint.to_account_info(),
                to: ctx.accounts.payer_event_mint_ata.to_account_info(),
                authority: ctx.accounts.evento.to_account_info(),
            },
            signer,
        ),
        quantity,
    )?;

    ctx.accounts.evento.sponsor_ammount += quantity;
    Ok(())
  }