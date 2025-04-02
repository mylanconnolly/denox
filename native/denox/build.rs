fn main() {
    // Tell cargo to tell rustc to link the system's shared libraries
    println!("cargo:rustc-link-lib=dylib=dl");
    
    // Only apply these settings on Linux
    #[cfg(target_os = "linux")]
    {
        // Tell cargo to invalidate the built crate whenever the linker script changes
        println!("cargo:rerun-if-changed=build.rs");
        
        // Tell rustc to use the -rdynamic flag
        println!("cargo:rustc-link-arg=-rdynamic");
        
        // Add specific flags for V8
        println!("cargo:rustc-link-arg=-Wl,-z,muldefs");
        println!("cargo:rustc-link-arg=-Wl,--no-undefined");
        println!("cargo:rustc-link-arg=-Wl,--copy-dt-needed-entries");
    }
}
