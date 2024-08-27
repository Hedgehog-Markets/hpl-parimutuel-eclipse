use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::error::ParimutuelError;
use crate::instruction::accounts::DepositV1Accounts;
use crate::state::{AccountSized, MarketV1, UserPositionV1};
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub struct DepositV1Args {
    pub option: u8,
    pub amount: u64,
}

pub fn deposit_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: DepositV1Args,
) -> ProgramResult {
    let ctx = DepositV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.wallet)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Guard PDAs.
    pda::user_position::assert_pda(
        ctx.accounts.user_position.key,
        ctx.accounts.market.key,
        ctx.accounts.wallet.key,
    )?;

    let option = usize::from(args.option);

    // Step 1: Update market.
    {
        let mut market = MarketV1::from_account_info_mut(ctx.accounts.market)?;

        // Guard market PDA.
        market.assert_pda(ctx.accounts.market.key)?;

        // Check market is not closed.
        market.assert_not_closed()?;
        // Check market mint.
        market.assert_mint(ctx.accounts.mint.key)?;

        // Check the option is valid.
        if option >= usize::from(market.amounts.len()) {
            return Err(ParimutuelError::InvalidOption.into());
        }

        // Update option amount.
        market.amounts[option] = checked_add!(market.amounts[option], args.amount)?;

        market.save()?;
    }

    // Step 2: Update user position.
    {
        let mut user_position = UserPositionV1::from_account_info_mut(ctx.accounts.user_position)?;

        // Track the total amount of deposits into the market for the given option.
        user_position.amounts[option] = checked_add!(user_position.amounts[option], args.amount)?;

        user_position.save()?;
    }

    // Step 3: Deposit amount into market deposit account.
    {
        let mint_decimals = cpi::spl::mint_decimals(ctx.accounts.mint)?;

        cpi::spl::transfer_checked(
            args.amount,
            mint_decimals,
            cpi::spl::TransferChecked {
                source: ctx.accounts.token_account,
                destination: ctx.accounts.deposit,
                mint: ctx.accounts.mint,
                authority: ctx.accounts.wallet,
                token_program: ctx.accounts.token_program,
            },
            &[],
        )?;
    }

    Ok(())
}
