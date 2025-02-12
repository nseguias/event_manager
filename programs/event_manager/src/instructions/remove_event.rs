use crate::collections::*;
use crate::utility::*;
use anchor_lang::prelude::*;
use anchor_spl::token::*;

pub fn remove_event(ctx: Context<RemoveEvent>) -> Result<()> {
    let signer_seeds: &[&[&[u8]]] = &[&[
        ctx.accounts.event.metadata.id.as_ref(),
        Event::EVENT_SEED.as_bytes(),
        ctx.accounts.event.accounts.organizer.as_ref(),
        &[ctx.accounts.event.bumps.event],
    ]];

    let close_ticket_vault = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.ticket_vault.to_account_info(),
            destination: ctx.accounts.organizer.to_account_info(),
            authority: ctx.accounts.event.to_account_info(),
        },
    )
    .with_signer(signer_seeds);

    close_account(close_ticket_vault)?;

    let close_sponsorship_vault = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.sponsorship_vault.to_account_info(),
            destination: ctx.accounts.organizer.to_account_info(),
            authority: ctx.accounts.event.to_account_info(),
        },
    )
    .with_signer(signer_seeds);

    close_account(close_sponsorship_vault)?;

    let revoke_authorrity = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SetAuthority {
            current_authority: ctx.accounts.event.to_account_info(),
            account_or_mint: ctx.accounts.event_token.to_account_info(),
        },
    )
    .with_signer(signer_seeds);

    set_authority(
        revoke_authorrity,
        spl_token::instruction::AuthorityType::MintTokens,
        None,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct RemoveEvent<'info> {
    #[account(
        mut,
        seeds = [
            event.metadata.id.to_string().as_ref(),
            Event::EVENT_SEED.as_bytes(),
            organizer.key().as_ref(),
        ],
        bump = event.bumps.event,
        constraint = event.state.sponsorships_sold == 0 @ ContractError::EventHasSponsors,
        constraint = event.state.tickets_sold == 0 @ ContractError::EventHasParticipants,
        constraint = event.accounts.organizer == organizer.key() @ ContractError::Unauthorized,
        close = organizer,
    )]
    pub event: Account<'info, Event>,

    #[account(
        mut,
        seeds = [
            Event::SPONSORSHIP_VAULT_SEED.as_bytes(),
            event.key().as_ref(),
        ],
        bump = event.bumps.sponsorship_vault,
        constraint = sponsorship_vault.amount == 0 @ ContractError::VaultNotEmpty,
        // won't use close this time to practice CPIs
    )]
    pub sponsorship_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            Event::TICKET_VAULT_SEED.as_bytes(),
            event.key().as_ref(),
        ],
        bump = event.bumps.ticket_vault,
        constraint = ticket_vault.amount == 0 @ ContractError::VaultNotEmpty,

    )]
    pub ticket_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [
            Event::EVENT_TOKEN_SEED.as_bytes(),
            event.key().as_ref(),
        ],
        bump = event.bumps.event_token,
    )]
    pub event_token: Account<'info, Mint>,

    #[account(mut)]
    pub organizer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
