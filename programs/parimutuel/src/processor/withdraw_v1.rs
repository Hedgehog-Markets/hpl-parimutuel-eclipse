use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::WithdrawV1Accounts;
use crate::state::{Account, AccountSized, MarketV1, UserPositionV1};
use crate::utils::TrySum;
use crate::{pda, utils};

pub fn withdraw_v1<'a>(_program_id: &'a Pubkey, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = WithdrawV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.wallet)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;

    // Guard PDAs.
    pda::user_position::assert_pda(
        ctx.accounts.user_position.key,
        ctx.accounts.market.key,
        ctx.accounts.wallet.key,
    )?;
    pda::deposit::assert_pda(ctx.accounts.deposit.key, ctx.accounts.market.key)?;

    let market_creator: Pubkey;
    let market_index: u32;
    let market_bump: u8;

    // Step 1: Check market is invalid.
    {
        let market = MarketV1::from_account_info(ctx.accounts.market)?;

        market_creator = market.creator;
        market_index = market.index;

        // Guard market PDA.
        market_bump = market.assert_pda(ctx.accounts.market.key)?;

        market.assert_invalid()?;
    }

    let withdraw_amount: u64;

    // Step 2: Get withdraw amount and mark the user as having claimed.
    {
        let mut user_position = UserPositionV1::from_account_info_mut(ctx.accounts.user_position)?;

        // Get the amount the user can withdraw.
        withdraw_amount =
            user_position.amounts.iter().try_sum().ok_or(ProgramError::ArithmeticOverflow)?;

        // Mark the user as having claimed.
        user_position.claim()?;

        user_position.save()?;
    }

    // Step 3: Transfer tokens from deposit account to user token account.
    {
        let mint_decimals = cpi::spl::mint_decimals(ctx.accounts.mint)?;

        let signer_seeds =
            pda::market::seeds_with_bump(&market_creator, &market_index, &market_bump);

        cpi::spl::transfer_checked(
            withdraw_amount,
            mint_decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.deposit,
                destination: ctx.accounts.token_account,
                mint: ctx.accounts.mint,
                authority: ctx.accounts.market,
                token_program: ctx.accounts.token_program,
            },
            &[&signer_seeds],
        )?;
    }

    Ok(())
}
