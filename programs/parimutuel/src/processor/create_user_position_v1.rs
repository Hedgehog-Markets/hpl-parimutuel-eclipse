use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::CreateUserPositionV1Accounts;
use crate::state::{Account, InitAccount, InitContext, InitUserPosition, MarketV1, UserPositionV1};
use crate::{pda, utils};

pub fn create_user_position_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = CreateUserPositionV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.wallet)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Guard PDAs.
    pda::user::assert_pda(ctx.accounts.user.key, ctx.accounts.wallet.key)?;

    let options: u8;

    // Step 1: Check market is not closed.
    {
        let market = MarketV1::from_account_info(ctx.accounts.market)?;

        // Guard market PDA.
        market.assert_pda(ctx.accounts.market.key)?;

        // Check market is not closed.
        market.assert_not_closed()?;

        options = market.amounts.len();
    }

    // Step 1: Initialize user position account.
    {
        let bump = pda::user_position::assert_pda(
            ctx.accounts.user_position.key,
            ctx.accounts.market.key,
            ctx.accounts.wallet.key,
        )?;
        let signer_seeds = pda::user_position::seeds_with_bump(
            ctx.accounts.market.key,
            ctx.accounts.wallet.key,
            &bump,
        );

        UserPositionV1::try_init(InitUserPosition {
            market: *ctx.accounts.market.key,
            wallet: *ctx.accounts.wallet.key,
            options,
        })?
        .save(InitContext {
            account: ctx.accounts.user_position,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    Ok(())
}
