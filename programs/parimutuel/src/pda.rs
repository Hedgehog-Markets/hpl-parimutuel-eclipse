trait PdaSeed {
    fn pda_seed(&self) -> &[u8];
}

impl<T: bytemuck::NoUninit> PdaSeed for T {
    fn pda_seed(&self) -> &[u8] {
        bytemuck::bytes_of(self)
    }
}

#[cfg(target_endian = "big")]
compile_error!("only little endian targets are supported");

macro_rules! pdas {
    ($(
        $desc:literal: $name:ident($($seed:ident : $seed_ty:ty),* $(,)?);
    )*) => {
        $(
            pub mod $name {
                #![allow(unused_imports, clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]

                use super::*;

                use solana_program::program_error::ProgramError;
                use solana_program::pubkey::Pubkey;

                pub const PREFIX_SEED: &str = stringify!($name);

                const N_SEEDS: usize = 1 $(+ {
                    stringify!($seed);
                    1
                })*;

                pub fn seeds<'a>($($seed : &'a $seed_ty),*) -> [&'a [u8]; N_SEEDS] {
                    [PREFIX_SEED.as_bytes(), $(PdaSeed::pda_seed($seed)),*]
                }

                pub fn seeds_with_bump<'a>($($seed : &'a $seed_ty,)* bump: &'a u8) -> [&'a [u8]; N_SEEDS + 1] {
                    [PREFIX_SEED.as_bytes(), $(PdaSeed::pda_seed($seed),)* std::slice::from_ref(bump)]
                }

                pub fn pda<'a>($($seed : &'a $seed_ty),*) -> (Pubkey, u8) {
                    let seeds = seeds($($seed),*);
                    Pubkey::find_program_address(&seeds, &$crate::ID)
                }

                pub fn assert_pda<'a>($name: &'a Pubkey, $($seed : &'a $seed_ty),*) -> Result<u8, ProgramError> {
                    let (expected, bump) = pda($($seed),*);
                    if !solana_utils::pubkeys_eq($name, &expected) {
                        solana_utils::log!("Error: {} address does not match seed derivation", $desc);
                        return Err(ProgramError::InvalidSeeds);
                    }
                    Ok(bump)
                }
            }
        )*
    };
}

pdas! {
    "Config": config();

    "User": user(wallet: Pubkey);
    "Market": market(wallet: Pubkey, index: u32);
    "UserPosition": user_position(market: Pubkey, wallet: Pubkey);

    "Deposit account": deposit(market: Pubkey);
    "Creator fee account": creator_fees(wallet: Pubkey, mint: Pubkey);
    "Platform fee account": platform_fees(mint: Pubkey);
}
