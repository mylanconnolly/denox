fn main() {
    // Detect the target OS
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    if target_os == "linux" {
        // On Linux, we need to ensure we're using shared libraries for V8
        println!("cargo:rustc-env=V8_FROM_SOURCE=0");
        println!("cargo:rustc-env=V8_FORCE_STATIC=0");
    } else if target_os == "macos" {
        // On macOS, we can use static linking
        println!("cargo:rustc-env=V8_FROM_SOURCE=0");
        println!("cargo:rustc-env=V8_FORCE_STATIC=1");
    }
}
