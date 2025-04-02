fn main() {
    // Tell cargo to invalidate the built crate whenever the build script changes
    println!("cargo:rerun-if-changed=build.rs");
    
    // Only apply these settings on Linux
    #[cfg(target_os = "linux")]
    {
        // Link to system libraries
        println!("cargo:rustc-link-lib=dylib=dl");
        println!("cargo:rustc-link-lib=dylib=pthread");
        
        // Use static linking for C++ standard library
        println!("cargo:rustc-link-lib=static=stdc++");
        
        // Tell rustc to use the -rdynamic flag
        println!("cargo:rustc-link-arg=-rdynamic");
    }
}
