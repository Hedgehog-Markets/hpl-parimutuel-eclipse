use common::cpi;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::ParimutuelError;
use crate::instruction::accounts::ClaimV1Accounts;
use crate::state::{Account, AccountSized, MarketV1, State, UserPositionV1};
use crate::utils::TrySum;
use crate::{pda, utils};

pub fn claim_v1<'a>(_program_id: &'a Pubkey, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = ClaimV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.wallet)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;

    let market = MarketV1::from_account_info(ctx.accounts.market)?;

    // Guard market PDA.
    let market_bump = market.assert_pda(ctx.accounts.market.key)?;

    // Guard market mint.
    market.assert_mint(ctx.accounts.mint.key)?;

    // Guard PDAs.
    pda::user_position::assert_pda(
        ctx.accounts.user_position.key,
        ctx.accounts.market.key,
        ctx.accounts.wallet.key,
    )?;
    pda::deposit::assert_pda(ctx.accounts.deposit.key, ctx.accounts.market.key)?;
    pda::creator_fees::assert_pda(
        ctx.accounts.creator_fees.key,
        &market.creator,
        ctx.accounts.mint.key,
    )?;
    pda::platform_fees::assert_pda(ctx.accounts.platform_fees.key, ctx.accounts.mint.key)?;

    // Check market is resolved.
    match market.state {
        State::Open => return Err(ParimutuelError::MarketNotResolved.into()),
        State::Invalid => return Err(ParimutuelError::MarketInvalid.into()),
        State::Resolved => {}
    }

    let user_correct: u64;
    let pool_correct: u64;
    let pool_incorrect: u64;

    // Step 1: Get variables for claim calculation and mark the user as having claimed.
    {
        let mut user_position = UserPositionV1::from_account_info_mut(ctx.accounts.user_position)?;

        let outcome = usize::from(market.outcome);

        user_correct = user_position.amounts[outcome];
        pool_correct = market.amounts[outcome];

        let pool = market.amounts.iter().try_sum().ok_or(ProgramError::ArithmeticOverflow)?;

        pool_incorrect = checked_sub!(pool, pool_correct)?;

        // Step 1.2: Mark the user as having claimed.
        user_position.claim()?;

        user_position.save()?;
    }

    // Step 2: Compute winnings.
    let winnings = compute_winnings(user_correct, pool_correct, pool_incorrect);

    log!("Winnings: {winnings}");

    // Step 3: Compute fees.
    let creator_fee = market.creator_fee.calculate(winnings);
    let platform_fee = market.platform_fee.calculate(winnings);

    // Step 4: Compute winnings minus fees.
    let winnings = winnings.saturating_sub(creator_fee).saturating_sub(platform_fee);

    // Step 5: Compute tokens to send to user.
    let user_tokens = checked_add!(winnings, user_correct)?;

    log!("Creator fee: {creator_fee}");
    log!("Platform fee: {platform_fee}");

    let mint_decimals = cpi::spl::mint_decimals(ctx.accounts.mint)?;

    let signer_seeds = pda::market::seeds_with_bump(&market.creator, &market.index, &market_bump);

    // Step 6: Transfer creator fee.
    cpi::spl::transfer_checked(
        creator_fee,
        mint_decimals,
        cpi::spl::TransferChecked {
            source: ctx.accounts.deposit,
            destination: ctx.accounts.creator_fees,
            mint: ctx.accounts.mint,
            authority: ctx.accounts.market,
            token_program: ctx.accounts.token_program,
        },
        &[&signer_seeds],
    )?;

    // Step 7: Transfer platform fee.
    cpi::spl::transfer_checked(
        platform_fee,
        mint_decimals,
        cpi::spl::TransferChecked {
            source: ctx.accounts.deposit,
            destination: ctx.accounts.platform_fees,
            mint: ctx.accounts.mint,
            authority: ctx.accounts.market,
            token_program: ctx.accounts.token_program,
        },
        &[&signer_seeds],
    )?;

    // Step 8: Transfer user tokens.
    cpi::spl::transfer_checked(
        user_tokens,
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

    Ok(())
}

/// Computes the winnings from the incorrect pool, based on the user holdings as a percentage of the correct pool.
const fn compute_winnings(user_correct: u64, pool_correct: u64, pool_incorrect: u64) -> u64 {
    if user_correct == 0 || pool_correct == 0 {
        return 0;
    }
    ((user_correct as u128 * pool_incorrect as u128) / pool_correct as u128) as u64
}
