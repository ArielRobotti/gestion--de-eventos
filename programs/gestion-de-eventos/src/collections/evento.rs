use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Evento {
    #[max_len(20)]
    pub name: String,
    pub ticket_price: u64,
    pub open_sales: bool,
    //cerrar las ventas (open_sales = false) cuando timestamp_event sea alcanzado por el Clock
    pub timestamp_event_close: i64,
    pub sponsor_ammount: u64,

    pub authority: Pubkey,
    pub accepted_mint: Pubkey,

    pub event_bump: u8,
    pub event_mint_bump: u8,
    pub treasury_vault_bump: u8,
    pub gain_vault_bump: u8,

}

impl Evento {
    pub const SEED_EVENT: &'static str = "event";
    pub const SEED_EVENT_MINT: &'static str = "event-mint";
    pub const SEED_TREASURY_VAULT: &'static str = "treasury_vault";
    pub const SEED_GAIN_VAULT: &'static str = "gain_vault";

}

