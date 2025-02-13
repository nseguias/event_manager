use crate::instructions::*;

use anchor_lang::prelude::*;

mod collections;
mod instructions;
mod utility;

use crate::collections::event::{Metadata, Prices};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("7NYupbzS2cL96Yyx6bAvxJeee75iNFuPaDtGuNw7jTRs");

#[program]
mod event_manager {
    use super::*;

    pub fn new_event(ctx: Context<NewEvent>, metadata: Metadata, prices: Prices) -> Result<()> {
        instructions::new_event(ctx, metadata, prices)
    }

    pub fn remove_event(ctx: Context<RemoveEvent>) -> Result<()> {
        instructions::remove_event(ctx)
    }

    pub fn end_event(ctx: Context<EndEvent>) -> Result<()> {
        instructions::end_event(ctx)
    }
}
