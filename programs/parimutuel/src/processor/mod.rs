use borsh::BorshDeserialize;
use common::VariantName;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

mod claim_v1;
mod create_config_v1;
mod create_market_v1;
mod create_user_position_v1;
mod create_user_v1;
mod deposit_v1;
mod invalidate_inactive_v1;
mod resolve_v1;
mod withdraw_creator_fees_v1;
mod withdraw_platform_fees_v1;
mod withdraw_v1;

pub(crate) use self::claim_v1::*;
pub(crate) use self::create_config_v1::*;
pub(crate) use self::create_market_v1::*;
pub(crate) use self::create_user_position_v1::*;
pub(crate) use self::create_user_v1::*;
pub(crate) use self::deposit_v1::*;
pub(crate) use self::invalidate_inactive_v1::*;
pub(crate) use self::resolve_v1::*;
pub(crate) use self::withdraw_creator_fees_v1::*;
pub(crate) use self::withdraw_platform_fees_v1::*;
pub(crate) use self::withdraw_v1::*;

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    use crate::instruction::ParimutuelInstruction as I;

    let instruction = I::try_from_slice(instruction_data)?;

    log!("Instruction: {}", instruction.variant_name());

    match instruction {
        I::CreateConfigV1(args) => create_config_v1(program_id, accounts, args),
        I::CreateUserV1 => create_user_v1(program_id, accounts),
        I::CreateMarketV1(args) => create_market_v1(program_id, accounts, args),
        I::CreateUserPositionV1 => create_user_position_v1(program_id, accounts),
        I::DepositV1(args) => deposit_v1(program_id, accounts, args),
        I::ResolveV1(args) => resolve_v1(program_id, accounts, args),
        I::WithdrawV1 => withdraw_v1(program_id, accounts),
        I::ClaimV1 => claim_v1(program_id, accounts),
        I::WithdrawCreatorFeesV1 => withdraw_creator_fees_v1(program_id, accounts),
        I::WithdrawPlatformFeesV1 => withdraw_platform_fees_v1(program_id, accounts),
        I::InvalidateInactiveV1 => invalidate_inactive_v1(program_id, accounts),
    }
}
