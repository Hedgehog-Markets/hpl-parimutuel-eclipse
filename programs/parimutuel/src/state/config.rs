use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use crate::error::ParimutuelError;
use crate::utils::Bps;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct ConfigV1 {
    account_type: AccountType,

    /// The address of the authority that can update the config.
    pub authority: Pubkey,

    /// The fee taken by the platform in basis points.
    pub platform_fee: Bps,
    /// The duration in seconds, after the resolve timestamp of a market,
    /// beyond which a market can be resolved as invalid for inactivity.
    pub inactive_duration: u32,
}

impl ConfigV1 {
    pub fn assert_authority(&self, authority: &Pubkey) -> Result<(), ParimutuelError> {
        if !solana_utils::pubkeys_eq(&self.authority, authority) {
            return Err(ParimutuelError::ConfigAuthorityMismatch);
        }
        Ok(())
    }
}

impl Account for ConfigV1 {
    const TYPE: AccountType = AccountType::ConfigV1;
}

impl From<InitConfig> for (ConfigV1, usize) {
    fn from(params: InitConfig) -> (ConfigV1, usize) {
        let InitConfig { authority, platform_fee, inactive_duration } = params;

        (
            ConfigV1 { account_type: ConfigV1::TYPE, authority, platform_fee, inactive_duration },
            ConfigV1::FIXED_SIZE,
        )
    }
}

pub(crate) struct InitConfig {
    pub authority: Pubkey,

    pub platform_fee: Bps,
    pub inactive_duration: u32,
}
