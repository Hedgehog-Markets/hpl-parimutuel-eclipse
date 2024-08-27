use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::ParimutuelError;
use crate::instruction::accounts::CreateMarketV1Accounts;
use crate::state::{
    Account, AccountSized, ConfigV1, InitAccount, InitContext, InitMarket, MarketV1, UserV1,
};
use crate::utils::Bps;
use crate::{pda, utils};

#[derive(Clone, BorshDeserialize)]
pub struct CreateMarketV1Args {
    pub resolver: Pubkey,
    pub close_timestamp: i64,
    pub resolve_timestamp: i64,
    pub creator_fee: u16,
    pub options: u8,
    pub uri: String,
}

pub fn create_market_v1<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    args: CreateMarketV1Args,
) -> ProgramResult {
    let ctx = CreateMarketV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.wallet)?;
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_token_program(ctx.accounts.token_program.key)?;
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Guard PDAs.
    pda::config::assert_pda(ctx.accounts.config.key)?;
    pda::user::assert_pda(ctx.accounts.user.key, ctx.accounts.wallet.key)?;

    let platform_fees: Bps;
    let market_index: u32;

    let creator_fee = Bps::try_from(args.creator_fee)?;

    // Step 1: Get platform fees.
    {
        let config = ConfigV1::from_account_info(ctx.accounts.config)?;

        platform_fees = config.platform_fee;
    }

    // Step 2: Check combined fees don't exceed 100%.
    if creator_fee.get().wrapping_add(platform_fees.get()) > Bps::MAX {
        return Err(ParimutuelError::FeeOverflow.into());
    }

    // Step 3: Get and increment market index.
    {
        let mut user = UserV1::from_account_info_mut(ctx.accounts.user)?;

        market_index = user.next_market;

        user.next_market = checked_add!(user.next_market, 1)?;
        user.save()?;
    }

    // Step 4: Initialize market account.
    {
        let bump = pda::market::assert_pda(
            ctx.accounts.market.key,
            ctx.accounts.wallet.key,
            &market_index,
        )?;
        let signer_seeds =
            pda::market::seeds_with_bump(ctx.accounts.wallet.key, &market_index, &bump);

        let now = Clock::get()?;

        if args.close_timestamp <= now.unix_timestamp {
            return Err(ParimutuelError::CloseTimestampInPast.into());
        }
        if args.resolve_timestamp < args.close_timestamp {
            return Err(ParimutuelError::ResolveTimestampBeforeClose.into());
        }

        MarketV1::try_init(InitMarket {
            creator: *ctx.accounts.wallet.key,
            index: market_index,
            resolver: args.resolver,
            mint: *ctx.accounts.mint.key,
            close_timestamp: args.close_timestamp,
            resolve_timestamp: args.resolve_timestamp,
            creator_fee,
            platform_fee: platform_fees,
            options: args.options,
            uri: args.uri,
        })?
        .save(InitContext {
            account: ctx.accounts.market,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    // Step 5: Initialize deposit token account.
    {
        let bump = pda::deposit::assert_pda(ctx.accounts.deposit.key, ctx.accounts.market.key)?;
        let signer_seeds = pda::deposit::seeds_with_bump(ctx.accounts.market.key, &bump);

        cpi::spl::create_token_account(
            ctx.accounts.market.key,
            cpi::spl::CreateTokenAccount {
                account: ctx.accounts.deposit,
                mint: ctx.accounts.mint,
                payer: ctx.accounts.payer,
                token_program: ctx.accounts.token_program,
                system_program: ctx.accounts.system_program,
            },
            &[&signer_seeds],
        )?;
    }

    // Step 6: Initialize creator fees account if necessary.
    {
        let bump = pda::creator_fees::assert_pda(
            ctx.accounts.creator_fees.key,
            ctx.accounts.wallet.key,
            ctx.accounts.mint.key,
        )?;

        if ctx.accounts.creator_fees.data_is_empty() {
            let signer_seeds = pda::creator_fees::seeds_with_bump(
                ctx.accounts.wallet.key,
                ctx.accounts.mint.key,
                &bump,
            );

            cpi::spl::create_token_account(
                ctx.accounts.user.key,
                cpi::spl::CreateTokenAccount {
                    account: ctx.accounts.creator_fees,
                    mint: ctx.accounts.mint,
                    payer: ctx.accounts.payer,
                    token_program: ctx.accounts.token_program,
                    system_program: ctx.accounts.system_program,
                },
                &[&signer_seeds],
            )?;
        }
    }

    // Step 7: Initialize platform fees account if necessary.
    {
        let bump =
            pda::platform_fees::assert_pda(ctx.accounts.platform_fees.key, ctx.accounts.mint.key)?;

        if ctx.accounts.platform_fees.data_is_empty() {
            let signer_seeds = pda::platform_fees::seeds_with_bump(ctx.accounts.mint.key, &bump);

            cpi::spl::create_token_account(
                ctx.accounts.config.key,
                cpi::spl::CreateTokenAccount {
                    account: ctx.accounts.platform_fees,
                    mint: ctx.accounts.mint,
                    payer: ctx.accounts.payer,
                    token_program: ctx.accounts.token_program,
                    system_program: ctx.accounts.system_program,
                },
                &[&signer_seeds],
            )?;
        }
    }

    Ok(())
}
