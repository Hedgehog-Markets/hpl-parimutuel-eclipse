use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::CreateUserV1Accounts;
use crate::state::{InitAccount, InitContext, InitUser, UserV1};
use crate::{pda, utils};

pub fn create_user_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = CreateUserV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.wallet)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Step 1: Initialize user account.
    {
        let bump = pda::user::assert_pda(ctx.accounts.user.key, ctx.accounts.wallet.key)?;
        let signer_seeds = pda::user::seeds_with_bump(ctx.accounts.wallet.key, &bump);

        UserV1::init(InitUser { wallet: *ctx.accounts.wallet.key }).save(InitContext {
            account: ctx.accounts.user,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    Ok(())
}
