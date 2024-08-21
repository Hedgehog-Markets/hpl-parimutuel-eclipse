use borsh::{BorshDeserialize, BorshSerialize};
use common::BorshSize;
use shank::ShankAccount;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::ParimutuelError;
use crate::utils::{SmallArray, SmallU64Array};

use super::{Account, AccountSized, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, ShankAccount)]
pub struct UserPositionV1 {
    account_type: AccountType,

    /// The address of the market.
    pub market: Pubkey,
    /// The address of the user's wallet.
    pub wallet: Pubkey,
    /// Whether the user has claimed for their position.
    pub claimed: bool,
    /// The amounts deposited for each position.
    pub amounts: SmallU64Array,
}

impl UserPositionV1 {
    const BASE_SIZE: usize =
        AccountType::SIZE // account_type
        + Pubkey::SIZE // market
        + Pubkey::SIZE // wallet
        + bool::SIZE // claimed
        + u8::SIZE // amounts.len()
        ;

    pub fn claim(&mut self) -> Result<(), ParimutuelError> {
        if self.claimed {
            return Err(ParimutuelError::AlreadyClaimed);
        }
        self.claimed = true;

        Ok(())
    }
}

impl Account for UserPositionV1 {
    const TYPE: AccountType = AccountType::UserPositionV1;
}

impl AccountSized for UserPositionV1 {
    const IS_FIXED_SIZE: bool = false;

    fn serialized_size(&self) -> Option<usize> {
        Self::BASE_SIZE.checked_add(usize::from(self.amounts.len()).checked_mul(u64::SIZE)?)
    }
}

impl TryFrom<InitUserPosition> for (UserPositionV1, usize) {
    type Error = ProgramError;

    fn try_from(params: InitUserPosition) -> Result<(UserPositionV1, usize), Self::Error> {
        let InitUserPosition { market, wallet, options } = params;

        let position = UserPositionV1 {
            account_type: UserPositionV1::TYPE,
            market,
            wallet,
            claimed: false,
            amounts: SmallArray::from_elem(0, options),
        };
        let size = position.serialized_size().ok_or(ProgramError::ArithmeticOverflow)?;

        Ok((position, size))
    }
}

pub(crate) struct InitUserPosition {
    pub market: Pubkey,
    pub wallet: Pubkey,
    pub options: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn market_size() {
        let init = InitUserPosition {
            market: Pubkey::new_unique(),
            wallet: Pubkey::new_unique(),
            options: 2,
        };

        let (request, expected) = <(UserPositionV1, usize)>::try_from(init).unwrap();
        let actual = common_test::serialized_len(&request).unwrap();

        assert_eq!(expected, actual);
    }
}
