#[cfg(feature = "parse")]
pub mod parse;

#[macro_export]
macro_rules! security_txt {

    (
        @required [$($required:ident)*]
        @stack [name $($stack:ident)*]
    ) => {
        $crate::security_txt! {
            @required [$($required)* name]
            @stack [$($stack)*]
        }
    };
    (
        @required [$($required:ident)*]
        @stack [project_url $($stack:ident)*]
    ) => {
        $crate::security_txt! {
            @required [$($required)* project_url]
            @stack [$($stack)*]
        }
    };
    (
        @required [$($required:ident)*]
        @stack [contacts $($stack:ident)*]
    ) => {
        $crate::security_txt! {
            @required [$($required)* contacts]
            @stack [$($stack)*]
        }
    };
    (
        @required [$($required:ident)*]
        @stack [policy $($stack:ident)*]
    ) => {
        $crate::security_txt! {
            @required [$($required)* policy]
            @stack [$($stack)*]
        }
    };
    (
        @required [$($required:ident)*]
        @stack [$head:ident $($stack:ident)*]
    ) => {
        $crate::security_txt! {
            @required [$($required)*]
            @stack [$($stack)*]
        }
    };

    (
        @required [$($required:ident)*]
        @stack []
    ) => {
        struct Required {
            name: &'static str,
            project_url: &'static str,
            contacts: &'static str,
            policy: &'static str,
        }

        let _ = Required {
            $($required: "",)*
        };
    };

    ($($field:ident: $value:expr),* $(,)?) => {
        const _: () = {
            let _ = || {
                #[derive(Default)]
                struct SecurityTxt {
                    name: &'static str,
                    project_url: &'static str,
                    contacts: &'static str,
                    policy: &'static str,
                    preferred_languages: &'static str,
                    source_code: &'static str,
                    source_release: &'static str,
                    source_revision: &'static str,
                    encryption: &'static str,
                    auditors: &'static str,
                    acknowledgements: &'static str,
                    expiry: &'static str,
                }

                // Check fields are valid and without duplicates.
                let _ = SecurityTxt {
                    $($field: $value,)*
                    ..Default::default()
                };

                // Check all required fields are present.
                $crate::security_txt! {
                    @required []
                    @stack [$($field)*]
                }
            };

            #[allow(dead_code)]
            #[no_mangle]
            #[cfg_attr(target_os = "solana", link_section = ".security.txt")]
            static security_txt: &str = concat! {
                "=======BEGIN SECURITY.TXT V1=======\0",
                $(stringify!($field), "\0", $value, "\0",)*
                "=======END SECURITY.TXT V1=======\0"
            };
        };
    };
}
