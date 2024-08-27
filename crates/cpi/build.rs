fn main() {
    if version_check::is_min_version("1.77.0").unwrap_or_default() {
        println!("cargo:rustc-check-cfg=cfg(target_os, values(\"solana\"))")
    }
}
