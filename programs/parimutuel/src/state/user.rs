use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::ParimutuelError;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct UserV1 {
    account_type: AccountType,

    pub wallet: Pubkey,
    pub next_market: u32,
}

impl UserV1 {
    pub fn assert_wallet(&self, wallet: &Pubkey) -> Result<(), ParimutuelError> {
        if !solana_utils::pubkeys_eq(&self.wallet, wallet) {
            return Err(ParimutuelError::UserWalletMismatch);
        }
        Ok(())
    }
}

impl Account for UserV1 {
    const TYPE: AccountType = AccountType::UserV1;
}

impl From<InitUser> for (UserV1, usize) {
    fn from(params: InitUser) -> (UserV1, usize) {
        let InitUser { wallet } = params;

        (UserV1 { account_type: UserV1::TYPE, wallet, next_market: 0 }, UserV1::FIXED_SIZE)
    }
}

pub(crate) struct InitUser {
    pub wallet: Pubkey,
}
