use crate::instructions::*;

use anchor_lang::prelude::*;

mod collections;
mod instructions;

use crate::collections::event::{Metadata, Prices};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("EXj3NMkLMiUpErE4gmnfBiAU22oNh5Sr2QsBVzbYJPNX");

#[program]
mod event_manager {
    use super::*;

    pub fn new_event(ctx: Context<NewEvent>, metadata: Metadata, prices: Prices) -> Result<()> {
        instructions::new_event(ctx, metadata, prices)?;
        Ok(())
    }
}
