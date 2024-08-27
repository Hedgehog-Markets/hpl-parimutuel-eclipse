use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::BorshSize;
use shank::ShankAccount;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

use crate::error::ParimutuelError;
use crate::utils::{SmallArray, SmallU64Array};

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
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
        let size = position.borsh_size();

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
        let actual = request.try_to_vec().unwrap().len();

        assert_eq!(expected, actual);
    }
}
