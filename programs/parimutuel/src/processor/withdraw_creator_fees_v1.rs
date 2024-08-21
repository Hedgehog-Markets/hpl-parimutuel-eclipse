use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::WithdrawCreatorFeesV1Accounts;
use crate::state::{Account, UserV1};
use crate::{pda, utils};

pub fn withdraw_creator_fees_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = WithdrawCreatorFeesV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.wallet)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;

    // Guard PDAs.
    pda::creator_fees::assert_pda(
        ctx.accounts.creator_fees.key,
        ctx.accounts.wallet.key,
        ctx.accounts.mint.key,
    )?;

    // Step 1: Check user wallet matches.
    {
        let user = UserV1::from_account_info(ctx.accounts.user)?;

        user.assert_wallet(ctx.accounts.wallet.key)?;
    }

    // Step 2: Transfer tokens from fee account to user token account.
    {
        let bump = pda::user::assert_pda(ctx.accounts.user.key, ctx.accounts.wallet.key)?;
        let signer_seeds = pda::user::seeds_with_bump(ctx.accounts.wallet.key, &bump);

        let mint_decimals = cpi::spl::mint_decimals(ctx.accounts.mint)?;
        let withdraw_amount = cpi::spl::account_amount(ctx.accounts.creator_fees)?;

        cpi::spl::transfer_checked(
            withdraw_amount,
            mint_decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.creator_fees,
                destination: ctx.accounts.token_account,
                mint: ctx.accounts.mint,
                authority: ctx.accounts.user,
                token_program: ctx.accounts.token_program,
            },
            &[&signer_seeds],
        )?;
    }

    Ok(())
}
