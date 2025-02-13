use crate::collections::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

// define the function
pub fn new_event(
    ctx: Context<NewEvent>,
    id: String,
    name: String,
    description: String,
    // we assume prices are sent in lamports
    ticket_price: u64,
    sponsorship_price: u64,
) -> Result<()> {
    // save metadata
    ctx.accounts.event.id = id;
    ctx.accounts.event.name = name;
    ctx.accounts.event.description = description;

    // save prices
    ctx.accounts.event.ticket_price = ticket_price;
    ctx.accounts.event.sponsorship_price = sponsorship_price;

    // save clean state
    ctx.accounts.event.is_active = true;
    ctx.accounts.event.unique_sponsors = 0;
    ctx.accounts.event.active_sponsors = 0;
    ctx.accounts.event.tickets_sold = 0;
    ctx.accounts.event.sponsorships_sold = 0;

    // save accounts
    ctx.accounts.event.organizer = ctx.accounts.organizer.key();
    ctx.accounts.event.base_denom = ctx.accounts.base_denom.key();

    // save bumps
    ctx.accounts.event.event_bump = ctx.bumps.event;
    ctx.accounts.event.event_token_bump = ctx.bumps.event_token;
    ctx.accounts.event.ticket_vault_bump = ctx.bumps.ticket_vault;
    ctx.accounts.event.sponsorship_vault_bump = ctx.bumps.sponsorship_vault;

    Ok(())
}

// define the context
#[derive(Accounts)]
#[instruction(id: String)]
pub struct NewEvent<'info> {
    #[account(
        init,
        seeds = [
            id.to_string().as_ref(),
            Event::EVENT_SEED.as_bytes(),
            organizer.key().as_ref(),
        ],
        bump,
        payer = organizer,
        space = 8 + Event::INIT_SPACE,
    )]
    pub event: Box<Account<'info, Event>>,

    pub base_denom: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [
            Event::EVENT_TOKEN_SEED.as_bytes(),
            event.key().as_ref(),
        ],
        bump,
        payer = organizer,
        mint::decimals = 0,
        mint::authority = event,
    )]
    pub event_token: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [
            Event::TICKET_VAULT_SEED.as_bytes(),
            event.key().as_ref(),
        ],
        bump,
        payer = organizer,
        token::mint = base_denom,
        token::authority = event,
    )]
    pub ticket_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        seeds = [
            Event::SPONSORSHIP_VAULT_SEED.as_bytes(),
            event.key().as_ref(),
        ],
        bump,
        payer = organizer,
        token::mint = base_denom,
        token::authority = event,
    )]
    pub sponsorship_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub organizer: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}
