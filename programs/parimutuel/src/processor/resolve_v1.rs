use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::ParimutuelError;
use crate::instruction::accounts::ResolveV1Accounts;
use crate::state::{AccountSized, MarketV1, State};
use crate::utils;

#[derive(Clone, BorshDeserialize)]
pub enum ResolveV1Args {
    Outcome { outcome: u8 },
    Invalid,
}

pub fn resolve_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: ResolveV1Args,
) -> ProgramResult {
    let ctx = ResolveV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.resolver)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Step 1: Update the market.
    {
        let mut market = MarketV1::from_account_info_mut(ctx.accounts.market)?;

        // Guard market.
        market.assert_pda(ctx.accounts.market.key)?;
        market.assert_resolver(ctx.accounts.resolver.key)?;

        // Check the market is not already resolved.
        if market.state != State::Open {
            return Err(ParimutuelError::AlreadyResolved.into());
        }

        let now = Clock::get()?;

        let (state, outcome) = match args {
            ResolveV1Args::Outcome { outcome } => {
                // Check outcome option is valid for the market options.
                if usize::from(outcome) >= usize::from(market.amounts.len()) {
                    return Err(ParimutuelError::InvalidOption.into());
                }

                // Check the market can be resolved.
                if now.unix_timestamp < market.resolve_timestamp {
                    return Err(ParimutuelError::ResolveTooEarly.into());
                }

                (State::Resolved, outcome)
            }
            ResolveV1Args::Invalid => (State::Invalid, 0),
        };

        // Update market outcome and outcome timestamp.
        market.state = state;
        market.outcome = outcome;
        market.outcome_timestamp = now.unix_timestamp;

        market.save()?;
    }

    Ok(())
}
