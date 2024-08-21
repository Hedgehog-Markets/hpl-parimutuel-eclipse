use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::instruction::accounts::CreateConfigV1Accounts;
use crate::state::{ConfigV1, InitAccount, InitConfig, InitContext};
use crate::utils::Bps;
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub struct CreateConfigV1Args {
    pub authority: Pubkey,
    pub platform_fee: u16,
    pub inactive_duration: u32,
}

pub fn create_config_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateConfigV1Args,
) -> ProgramResult {
    let ctx = CreateConfigV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Step 1: Initialize config account.
    {
        let bump = pda::config::assert_pda(ctx.accounts.config.key)?;
        let signer_seeds = pda::config::seeds_with_bump(&bump);

        let platform_fees = Bps::try_from(args.platform_fee)?;

        ConfigV1::init(InitConfig {
            authority: args.authority,
            platform_fee: platform_fees,
            inactive_duration: args.inactive_duration,
        })
        .save(InitContext {
            account: ctx.accounts.config,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    Ok(())
}
