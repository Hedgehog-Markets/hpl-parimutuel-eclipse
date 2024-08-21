#[cfg(feature = "parse")]
pub mod parse;

/// Create a static string containing the security.txt file.
#[macro_export]
macro_rules! security_txt {
    ($($name:ident: $value:expr),* $(,)?) => {
        const _: () = {
            #[allow(dead_code)]
            #[no_mangle]
            #[cfg_attr(target_os = "solana", link_section = ".security.txt")]
            static security_txt: &str = concat! {
                "=======BEGIN SECURITY.TXT V1=======\0",
                $(stringify!($name), "\0", $value, "\0",)*
                "=======END SECURITY.TXT V1=======\0"
            };
        };
    };
}
