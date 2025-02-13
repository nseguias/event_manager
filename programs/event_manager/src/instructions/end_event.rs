use crate::collections::*;
use crate::utility::*;
use anchor_lang::prelude::*;

pub fn end_event(ctx: Context<EndEvent>) -> Result<()> {
    ctx.accounts.event.is_active = false;
    Ok(())
}

#[derive(Accounts)]
pub struct EndEvent<'info> {
    #[account(
        mut,
        seeds = [
            event.id.to_string().as_ref(),
            Event::EVENT_SEED.as_bytes(),
            event.organizer.as_ref(),
        ],
        bump = event.event_bump,
        constraint = event.organizer == organizer.key() @ ContractError::Unauthorized,
    )]
    pub event: Account<'info, Event>,

    #[account(mut)]
    pub organizer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
