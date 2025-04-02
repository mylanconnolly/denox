use std::env;
use std::path::Path;

fn main() {
    // Detect the target OS
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    
    // Common settings for all platforms
    println!("cargo:rustc-env=V8_FROM_SOURCE=0");
    
    if target_os == "linux" {
        // On Linux, we need to ensure we're NOT using static linking for V8
        println!("cargo:rustc-env=V8_FORCE_STATIC=0");
        
        // These flags help with the TLS relocation issue on Linux
        println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
        println!("cargo:rustc-link-arg=-ldl");
        println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
        
        // Add -fPIC to ensure position-independent code
        println!("cargo:rustc-env=CFLAGS=-fPIC");
        println!("cargo:rustc-env=CXXFLAGS=-fPIC");
        
        // Check if our wrapper library exists in the project
        let project_root = env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| ".".to_string());
        let wrapper_path = Path::new(&project_root)
            .join("..").join("..").join("v8_extract").join("lib").join("libv8_wrapper.so");
        
        if wrapper_path.exists() {
            println!("cargo:warning=Using V8 wrapper library at {:?}", wrapper_path);
            println!("cargo:rustc-link-search=native={}", wrapper_path.parent().unwrap().display());
            println!("cargo:rustc-link-lib=dylib=v8_wrapper");
        } else {
            println!("cargo:warning=V8 wrapper library not found at {:?}, using default linking", wrapper_path);
        }
    } else if target_os == "macos" {
        // On macOS, we can use static linking
        println!("cargo:rustc-env=V8_FORCE_STATIC=1");
        
        // macOS specific linker flags
        println!("cargo:rustc-link-arg=-undefined");
        println!("cargo:rustc-link-arg=dynamic_lookup");
    }
    
    // Print a message to help with debugging
    println!("cargo:warning=Building for {}", target_os);
}
