#[cfg(feature = "shrink")]
mod shrink;

#[cfg(feature = "parse")]
pub mod parse;

#[cfg(feature = "shrink")]
pub use crate::shrink::compress_idl;

#[macro_export]
macro_rules! include_idl {
    ($file:expr $(,)?) => {
        const _: () = {
            #[allow(dead_code)]
            #[no_mangle]
            #[cfg_attr(target_os = "solana", link_section = ".solana.idl")]
            static solana_idl: &[u8] = include_bytes!($file);
        };
    };
}

#[macro_export]
macro_rules! include_kinobi_idl {
    ($file:expr $(,)?) => {
        const _: () = {
            #[allow(dead_code)]
            #[no_mangle]
            #[cfg_attr(target_os = "solana", link_section = ".kinobi.idl")]
            static kinobi_idl: &[u8] = include_bytes!($file);
        };
    };
}
