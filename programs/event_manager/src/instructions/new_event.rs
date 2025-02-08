use crate::collections::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

// define the function
pub fn new_event(
    ctx: Context<NewEvent>,
    metadata: Metadata,
    // we assume prices are sent in lamports
    prices: Prices,
) -> Result<()> {
    // save metadata
    ctx.accounts.event.metadata = metadata;

    // save prices
    ctx.accounts.event.prices = prices;

    // save clean state
    ctx.accounts.event.state = State {
        is_active: true,
        ..Default::default()
    };

    // save accounts
    ctx.accounts.event.accounts.organizer = ctx.accounts.organizer.key();
    ctx.accounts.event.accounts.base_denom = ctx.accounts.base_denom.key();

    // save bumps
    ctx.accounts.event.bumps.event = ctx.bumps.event;
    ctx.accounts.event.bumps.event_token = ctx.bumps.event_token;
    ctx.accounts.event.bumps.ticket_vault = ctx.bumps.ticket_vault;
    ctx.accounts.event.bumps.sponsorship_vault = ctx.bumps.sponsorship_vault;

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
    pub event: Account<'info, Event>,

    pub base_denom: Account<'info, Mint>,

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
    pub event_token: Account<'info, Mint>,

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
