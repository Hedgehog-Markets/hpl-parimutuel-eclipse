#![deny(clippy::disallowed_macros, clippy::disallowed_methods, clippy::disallowed_types)]

#[macro_use]
mod macros;
mod utils;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod pda;
pub mod processor;
pub mod state;

// Export sdk types for downstream users with a different sdk version.
pub use solana_program;

#[cfg(not(feature = "no-entrypoint"))]
security_txt::security_txt! {
    name: "Hedgehog Parimutuel",
    project_url: "https://hedgehog.markets",
    contacts: "email:james@hedgehog.markets",
    policy: "https://github.com/Hedgehog-Markets/security/security",
}

#[cfg(not(feature = "no-entrypoint"))]
include_idl::include_idl!(concat!(env!("OUT_DIR"), "/solana.idl.zip"));

solana_program::declare_id!("PARrVs6F5egaNuz8g6pKJyU4ze3eX5xGZCFb3GLiVvu");
