use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error, FromPrimitive)]
pub enum ParimutuelError {
    #[error("Invalid basis points value")]
    InvalidBps,

    #[error("Close timestamp is in the past")]
    CloseTimestampInPast,

    #[error("Resolve timestamp is before the close timestamp")]
    ResolveTimestampBeforeClose,

    #[error("Market is closed")]
    MarketClosed,

    #[error("Market is invalid")]
    MarketInvalid,

    #[error("Market is not resolved")]
    MarketNotResolved,

    #[error("Market is not invalid")]
    MarketNotInvalid,

    #[error("Market already resolved")]
    AlreadyResolved,

    #[error("Already claimed")]
    AlreadyClaimed,

    #[error("Market resolution timestamp has not been reached")]
    ResolveTooEarly,

    #[error("Invalid option for market")]
    InvalidOption,

    #[error("Mint address does not match market mint")]
    MarketMintMismatch,

    #[error("Resolver address does not match market resolver")]
    MarketResolverMismatch,

    #[error("Wallet address does not match user wallet")]
    UserWalletMismatch,

    #[error("Authority address does not match config authority")]
    ConfigAuthorityMismatch,

    #[error("Market inactive timestamp has not been reached")]
    InactiveTooEarly,

    #[error("Failed to deserialize account")]
    DeserializationError,

    #[error("Failed to serialize account")]
    SerializationError,

    #[error("Combined platform and creator fees exceed 100%")]
    FeeOverflow,
}

impl PrintProgramError for ParimutuelError {
    fn print<E>(&self) {
        log!("Error: {self}");
    }
}

impl From<ParimutuelError> for ProgramError {
    fn from(e: ParimutuelError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ParimutuelError {
    fn type_of() -> &'static str {
        "ParimutuelError"
    }
}
