use anchor_lang::prelude::*;

#[error_code]
pub enum ContractError {
    #[msg("Unauthorized")]
    Unauthorized,

    #[msg("At least one sponsorship has been sold")]
    EventHasSponsors,

    #[msg("Vault must be empty")]
    VaultNotEmpty,

    #[msg("At least one ticket has been sold")]
    EventHasParticipants,
}
