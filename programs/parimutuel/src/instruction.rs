use borsh::BorshDeserialize;
use shank::{ShankContext, ShankInstruction};
use solana_utils::VariantName;

use crate::processor::*;

/// Instructions supported by the parimutuel program.
#[rustfmt::skip::attributes(account)]
#[derive(Clone, VariantName, ShankContext, ShankInstruction, BorshDeserialize)]
pub(crate) enum ParimutuelInstruction {
    /// Creates program config.
    #[account(0, writable, name = "config", desc = "Program config")]
    #[account(1, signer, writable, name = "payer", desc = "Payer")]
    #[account(2, name = "system_program", desc = "System program")]
    CreateConfigV1(CreateConfigV1Args),

    /// Creates a user.
    #[account(0, writable, name = "user", desc = "User")]
    #[account(1, signer, name = "wallet", desc = "User wallet")]
    #[account(2, signer, writable, name = "payer", desc = "Payer")]
    #[account(3, name = "system_program", desc = "System program")]
    CreateUserV1,

    /// Creates a market.
    #[account(0, name = "config", desc = "Program config")]
    #[account(1, writable, name = "market", desc = "Market")]
    #[account(2, writable, name = "user", desc = "User")]
    #[account(3, name = "mint", desc = "Deposit token mint")]
    #[account(4, writable, name = "deposit", desc = "Deposit token account")]
    #[account(5, writable, name = "creator_fees", desc = "Creator fees account")]
    #[account(6, writable, name = "platform_fees", desc = "Platform fees account")]
    #[account(7, signer, name = "wallet", desc = "User wallet")]
    #[account(8, signer, writable, name = "payer", desc = "Payer")]
    #[account(9, name = "token_program", desc = "SPL token program")]
    #[account(10, name = "system_program", desc = "System program")]
    CreateMarketV1(CreateMarketV1Args),

    /// Creates a user position.
    #[account(0, name = "market", desc = "Market")]
    #[account(1, writable, name = "user_position", desc = "User position")]
    #[account(2, name = "user", desc = "User")]
    #[account(3, signer, name = "wallet", desc = "User wallet")]
    #[account(4, signer, writable, name = "payer", desc = "Payer")]
    #[account(5, name = "system_program", desc = "System program")]
    CreateUserPositionV1,

    /// Deposits into a market.
    #[account(0, writable, name = "market", desc = "Market")]
    #[account(1, writable, name = "user_position", desc = "User position")]
    #[account(2, name = "mint", desc = "Deposit token mint")]
    #[account(3, writable, name = "deposit", desc = "Deposit token account")]
    #[account(4, writable, name = "token_account", desc = "User token account")]
    #[account(5, signer, name = "wallet", desc = "User wallet")]
    #[account(6, signer, writable, name = "payer", desc = "Payer")]
    #[account(7, name = "token_program", desc = "SPL token program")]
    #[account(8, name = "system_program", desc = "System program")]
    DepositV1(DepositV1Args),

    /// Resolves a market.
    #[account(0, writable, name = "market", desc = "Market")]
    #[account(1, signer, name = "resolver", desc = "Resolver")]
    #[account(2, name = "mint", desc = "Deposit token mint")]
    #[account(3, writable, name = "deposit", desc = "Deposit token account")]
    #[account(4, signer, writable, name = "payer", desc = "Payer")]
    #[account(5, name = "token_program", desc = "SPL token program")]
    #[account(6, name = "system_program", desc = "System program")]
    ResolveV1(ResolveV1Args),

    /// Withdraws a user's deposits from an invalid market.
    #[account(0, name = "market", desc = "Market")]
    #[account(1, writable, name = "user_position", desc = "User position")]
    #[account(2, name = "mint", desc = "Deposit token mint")]
    #[account(3, writable, name = "deposit", desc = "Deposit token account")]
    #[account(4, writable, name = "token_account", desc = "User token account")]
    #[account(5, signer, name = "wallet", desc = "User wallet")]
    #[account(6, name = "token_program", desc = "SPL token program")]
    WithdrawV1,

    /// Claims a user's winnings from a resolved market.
    #[account(0, name = "market", desc = "Market")]
    #[account(1, writable, name = "user_position", desc = "User position")]
    #[account(2, name = "mint", desc = "Deposit token mint")]
    #[account(3, writable, name = "deposit", desc = "Deposit token account")]
    #[account(4, writable, name = "token_account", desc = "User token account")]
    #[account(5, writable, name = "creator_fees", desc = "Creator fees account")]
    #[account(6, writable, name = "platform_fees", desc = "Platform fees account")]
    #[account(7, signer, name = "wallet", desc = "User wallet")]
    #[account(8, name = "token_program", desc = "SPL token program")]
    ClaimV1,

    /// Withdraws creator fees.
    #[account(0, name = "user", desc = "User")]
    #[account(1, name = "mint", desc = "Token mint")]
    #[account(2, writable, name = "creator_fees", desc = "Creator fees account")]
    #[account(3, writable, name = "token_account", desc = "User token account")]
    #[account(4, signer, name = "wallet", desc = "User wallet")]
    #[account(5, name = "token_program", desc = "SPL token program")]
    WithdrawCreatorFeesV1,

    /// Withdraws platform fees.
    #[account(0, name = "config", desc = "Config")]
    #[account(1, name = "mint", desc = "Token mint")]
    #[account(2, writable, name = "platform_fees", desc = "Platform fees account")]
    #[account(3, writable, name = "token_account", desc = "Withdrawal token account")]
    #[account(4, signer, name = "authority", desc = "Config authority")]
    #[account(5, name = "token_program", desc = "SPL token program")]
    WithdrawPlatformFeesV1,

    /// Invalidates a market that has not been resolved after a sufficient period of time.
    #[account(0, name = "config", desc = "Config")]
    #[account(1, writable, name = "market", desc = "Market")]
    #[account(2, name = "mint", desc = "Deposit token mint")]
    #[account(3, writable, name = "deposit", desc = "Deposit token account")]
    #[account(4, signer, writable, name = "payer", desc = "Payer")]
    #[account(5, name = "token_program", desc = "SPL token program")]
    #[account(6, name = "system_program", desc = "System program")]
    InvalidateInactiveV1,
}
