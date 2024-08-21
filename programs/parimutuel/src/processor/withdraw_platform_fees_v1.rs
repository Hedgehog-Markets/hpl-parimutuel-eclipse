use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::WithdrawPlatformFeesV1Accounts;
use crate::state::{Account, ConfigV1};
use crate::{pda, utils};

pub fn withdraw_platform_fees_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = WithdrawPlatformFeesV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.authority)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;

    // Guard PDAs.
    pda::platform_fees::assert_pda(ctx.accounts.platform_fees.key, ctx.accounts.mint.key)?;

    let config_bump = pda::config::assert_pda(ctx.accounts.config.key)?;
    let signer_seeds = pda::config::seeds_with_bump(&config_bump);

    // Step 1: Check config authority matches.
    {
        let config = ConfigV1::from_account_info(ctx.accounts.config)?;

        config.assert_authority(ctx.accounts.authority.key)?;
    }

    // Step 2: Transfer tokens from fee account to withdrawal token account.
    {
        let mint_decimals = cpi::spl::mint_decimals(ctx.accounts.mint)?;
        let withdraw_amount = cpi::spl::account_amount(ctx.accounts.platform_fees)?;

        cpi::spl::transfer_checked(
            withdraw_amount,
            mint_decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.platform_fees,
                destination: ctx.accounts.token_account,
                mint: ctx.accounts.mint,
                authority: ctx.accounts.config,
                token_program: ctx.accounts.token_program,
            },
            &[&signer_seeds],
        )?;
    }

    Ok(())
}
