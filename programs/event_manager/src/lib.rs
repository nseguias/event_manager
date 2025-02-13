use crate::instructions::*;

use anchor_lang::prelude::*;

mod collections;
mod instructions;
mod utility;
#[cfg(not(feature = "no-entrypoint"))]

declare_id!("4nsfAkjy3u29a6zjpuaX6feWb8W3xDqwEx2Dz8srj7fb");

#[program]
mod event_manager {
    use super::*;

    pub fn new_event(
        ctx: Context<NewEvent>,
        id: String,
        name: String,
        description: String,
        ticket_price: u64,
        sponsorship_price: u64,
    ) -> Result<()> {
        instructions::new_event(ctx, id, name, description, ticket_price, sponsorship_price)
    }

    pub fn remove_event(ctx: Context<RemoveEvent>) -> Result<()> {
        instructions::remove_event(ctx)
    }

    pub fn end_event(ctx: Context<EndEvent>) -> Result<()> {
        instructions::end_event(ctx)
    }
}
