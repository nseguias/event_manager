use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    pub metadata: Metadata,
    pub prices: Prices,
    pub state: State,
    pub accounts: Accounts,
    pub bumps: Bumps,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Metadata {
    // unique identifier of the event
    #[max_len(16)]
    pub id: String,
    // name of the event
    #[max_len(40)]
    pub name: String,
    // describes the event
    #[max_len(150)]
    pub description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Prices {
    // price per event ticket
    pub ticket: u64,
    // price for a sponsorship share
    pub sponsorship: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, InitSpace)]
pub struct State {
    // whether the event is active or not
    pub is_active: bool,
    // amount of unique sponsors
    pub unique_sponsors: u64,
    // amount of active sponsors
    pub active_sponsors: u64,
    // amount of tickets sold
    pub tickets_sold: u64,
    // amount of sponsorhip shares sold
    pub sponsorships_sold: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Accounts {
    // admin that creates the event
    pub organizer: Pubkey,
    // currency accepted to buy tickets and sponsorships
    pub base_denom: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Bumps {
    // event bump (event itself is a PDA)
    pub event: u8,
    // event token bump (only program can mint tokens)
    pub event_token: u8,
    // ticket vault bump (PDA that stores funds raised by selling tickets)
    pub ticket_vault: u8,
    // sponsorship vault bumop (PDA that stores funds raised by sponsors)
    pub sponsorship_vault: u8,
}

impl Event {
    pub const EVENT_SEED: &'static str = "event";
    pub const EVENT_TOKEN_SEED: &'static str = "event_token";
    pub const TICKET_VAULT_SEED: &'static str = "ticket_vault";
    pub const SPONSORSHIP_VAULT_SEED: &'static str = "sponsorship_vault";
}
