
use {
    crate::collections::Evento, 
    anchor_lang::prelude::*, 
    anchor_spl::token::*
};

#[derive(Accounts)]
pub struct WithdrawEarnings<'info> {
    #[account(
        mut,
        seeds = [
            Evento::SEED_EVENT.as_ref(), // "event" seed
            evento.authority.key().as_ref() // authority public key
        ],
        bump = evento.event_bump,
    )]
    pub evento: Box<Account<'info, Evento>>, // event account

    #[account(
        mut,
        seeds = [
            Evento::SEED_EVENT_MINT.as_ref(),  // "event_mint" seed
            evento.key().as_ref() // event public key
        ],
        bump = evento.event_mint_bump,
      )]
    pub event_mint: Box<Account<'info, Mint>>, // event mint token
    
    #[account(mut)]
    // account receiveing funds -> example: USDC
    pub user_accepted_mint_ata: Box<Account<'info, TokenAccount>>, 

    #[account(mut)]
    // account burning tevent mint tokens
    pub user_event_mint_ata: Box<Account<'info, TokenAccount>>,

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
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<WithdrawEarnings>) -> Result<()> {
    // total event mint tokens sold (total in treasury vault)
    let total_tokens = ctx.accounts.evento.sponsor_amount; 
    // event mint tokens owned by the user
    let tokens_to_burn = ctx.accounts.user_event_mint_ata.amount;
    // total earnings from tickets (total in gain vault)
    let total_earnings = ctx.accounts.gain_vault.amount;

    // Porcentaje de participacion del usuario en las ganancias
    let share = (total_tokens * 100 ) as f64 / (tokens_to_burn as f64);
    // Monto total de ganancias del usuario en función del porcentaje de participación
    let total_to_deposit = ((total_earnings as f64 * share) /100.) as u64;

    burn(
        CpiContext::new(
          ctx.accounts.token_program.to_account_info(),
          Burn {
            mint: ctx.accounts.event_mint.to_account_info(), // event mint token
            from: ctx.accounts.user_event_mint_ata.to_account_info(), // user's event mint ATA
            authority: ctx.accounts.authority.to_account_info(), // burn authority (the user owner of the ATA)
          },
        ),
        ctx.accounts.user_event_mint_ata.amount, // burn all the event mint tokens owned by the user
      )?;
    
    // seed from event account PDA
    let seeds = [
        Evento::SEED_EVENT.as_bytes(), // "event" seed
        ctx.accounts.evento.authority.as_ref(), // event authority
        &[ctx.accounts.evento.event_bump], // event bump
    ]; 
    let signer = &[&seeds[..]]; // event PDA seeds
    
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.gain_vault.to_account_info(), // event gain vault 
                to: ctx.accounts.user_accepted_mint_ata.to_account_info(), // user's accepted mint ATA
                authority: ctx.accounts.evento.to_account_info(), // gain vault authority (the event)
            },
            signer, // event PDA seeds
        ),
        total_to_deposit, // total user earnings
    )?;

    Ok(())
}
