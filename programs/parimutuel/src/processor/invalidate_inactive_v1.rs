use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::ParimutuelError;
use crate::instruction::accounts::InvalidateInactiveV1Accounts;
use crate::state::{Account, AccountSized, ConfigV1, MarketV1, State};
use crate::{pda, utils};

pub fn invalidate_inactive_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = InvalidateInactiveV1Accounts::context(accounts)?;

    // Guard PDAs.
    pda::config::assert_pda(ctx.accounts.config.key)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    let inactive_duration: u32;

    // Step 1: Get the inactive duration.
    {
        let config = ConfigV1::from_account_info(ctx.accounts.config)?;

        inactive_duration = config.inactive_duration;
    }

    // Step 2: Update the market.
    {
        let mut market = MarketV1::from_account_info_mut(ctx.accounts.market)?;

        // Guard market PDA.
        market.assert_pda(ctx.accounts.market.key)?;

        // Check the market is not already resolved.
        if market.state != State::Open {
            return Err(ParimutuelError::AlreadyResolved.into());
        }

        let now = Clock::get()?;

        let inactive_timestamp =
            checked_add!(market.resolve_timestamp, i64::from(inactive_duration))?;

        // Check the market can be resolved invalid for inactivity.
        if now.unix_timestamp < inactive_timestamp {
            return Err(ParimutuelError::InactiveTooEarly.into());
        }

        // Update market state and outcome timestamp.
        market.state = State::Invalid;
        market.outcome_timestamp = now.unix_timestamp;

        market.save()?;
    }

    Ok(())
}
