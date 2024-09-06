use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::BorshSize;
use shank::ShankAccount;
use solana_program::clock::Clock;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::sysvar::Sysvar;

use crate::error::ParimutuelError;
use crate::pda;
use crate::utils::{Bps, SmallArray, SmallU64Array};

use super::{Account, AccountType};

#[derive(Clone, Default, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSize)]
#[repr(u8)]
pub enum State {
    /// The market is open.
    #[default]
    Open,
    /// The market has a valid outcome.
    Resolved,
    /// The market is not valid (e.g. the event was canceled).
    Invalid,
}

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct MarketV1 {
    account_type: AccountType,

    /// The ddress of the market creator's wallet.
    pub creator: Pubkey,
    /// The index of the market in the creator's markets.
    pub index: u32,
    /// The address the can resolve the market.
    pub resolver: Pubkey,
    /// The mint of the token in which the market is denominated.
    pub mint: Pubkey,

    /// The Unix timestamp when the market closes.
    pub close_timestamp: i64,
    /// The Unix timestamp when the market can be resolved.
    pub resolve_timestamp: i64,
    /// The Unix timestamp when the market was resolved with an outcome.
    pub outcome_timestamp: i64,

    /// The market creator fee (in basis points).
    pub creator_fee: Bps,
    /// The platform fee (in basis points).
    pub platform_fee: Bps,

    /// The state of the market.
    pub state: State,
    /// The outcome of the market.
    ///
    /// This is only set if [`state`] is [`Resolved`].
    ///
    /// [`state`]: MarketV1::state
    /// [`Resolved`]: State::Resolved
    pub outcome: u8,
    /// The amounts of tokens on the different outcomes side.
    pub amounts: SmallU64Array,

    /// URI pointing to off-chain market info.
    pub uri: String,
}

impl MarketV1 {
    pub fn assert_pda(&self, market: &Pubkey) -> Result<u8, ProgramError> {
        pda::market::assert_pda(market, &self.creator, &self.index)
    }

    pub fn assert_mint(&self, mint: &Pubkey) -> Result<(), ProgramError> {
        if !solana_utils::pubkeys_eq(&self.mint, mint) {
            return Err(ParimutuelError::MarketMintMismatch.into());
        }
        Ok(())
    }

    pub fn assert_resolver(&self, resolver: &Pubkey) -> Result<(), ProgramError> {
        if !solana_utils::pubkeys_eq(&self.resolver, resolver) {
            return Err(ParimutuelError::MarketResolverMismatch.into());
        }
        Ok(())
    }

    pub fn assert_not_closed(&self) -> Result<(), ProgramError> {
        if self.state != State::Open || Clock::get()?.unix_timestamp >= self.close_timestamp {
            return Err(ParimutuelError::MarketClosed.into());
        }
        Ok(())
    }

    pub fn assert_invalid(&self) -> Result<(), ProgramError> {
        if self.state != State::Invalid {
            return Err(ParimutuelError::MarketNotInvalid.into());
        }
        Ok(())
    }
}

impl Account for MarketV1 {
    const TYPE: AccountType = AccountType::MarketV1;
}

impl TryFrom<InitMarket> for (MarketV1, usize) {
    type Error = ProgramError;

    fn try_from(params: InitMarket) -> Result<(MarketV1, usize), Self::Error> {
        let InitMarket {
            creator,
            index,
            resolver,
            mint,
            close_timestamp,
            resolve_timestamp,
            creator_fee,
            platform_fee,
            options,
            uri,
        } = params;

        let market = MarketV1 {
            account_type: MarketV1::TYPE,
            creator,
            index,
            resolver,
            mint,
            close_timestamp,
            resolve_timestamp,
            outcome_timestamp: 0,
            creator_fee,
            platform_fee,
            state: State::Open,
            outcome: 0,
            amounts: SmallArray::from_elem(0, options),
            uri,
        };
        let space = market.borsh_size();

        Ok((market, space))
    }
}

pub(crate) struct InitMarket {
    pub creator: Pubkey,
    pub index: u32,
    pub resolver: Pubkey,

    pub mint: Pubkey,

    pub close_timestamp: i64,
    pub resolve_timestamp: i64,

    pub creator_fee: Bps,
    pub platform_fee: Bps,

    pub options: u8,
    pub uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn market_size() {
        let init = InitMarket {
            creator: Pubkey::new_unique(),
            index: 0,
            resolver: Pubkey::new_unique(),
            mint: Pubkey::new_unique(),
            close_timestamp: 0,
            resolve_timestamp: 0,
            creator_fee: Bps::new(0).unwrap(),
            platform_fee: Bps::new(0).unwrap(),
            options: 2,
            uri: "https://example.com".to_owned(),
        };

        let (request, expected) = <(MarketV1, usize)>::try_from(init).unwrap();
        let actual = borsh::object_length(&request).unwrap();

        assert_eq!(expected, actual);
    }
}
