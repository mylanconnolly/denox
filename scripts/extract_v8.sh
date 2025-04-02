#!/bin/bash
set -e

# This script extracts V8 libraries from Deno for use in our Rust NIF

# Create directories
mkdir -p v8_extract/lib
mkdir -p v8_extract/include

# Find the Deno binary
DENO_PATH=$(which deno)
if [ -z "$DENO_PATH" ]; then
  echo "Deno not found. Installing..."
  curl -fsSL https://deno.land/install.sh | sh
  DENO_PATH="$HOME/.deno/bin/deno"
fi

echo "Using Deno at: $DENO_PATH"

# Create a temporary directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Extract the Deno binary (it's a self-contained executable)
echo "Extracting Deno binary..."
cp "$DENO_PATH" ./deno
chmod +x ./deno

# On Linux, we need to extract the V8 symbols
if [ "$(uname)" = "Linux" ]; then
  echo "Creating V8 symbol links for Linux..."
  
  # Create a simple wrapper library that links to Deno
  cat > v8_wrapper.c << EOF
#include <dlfcn.h>
#include <stdio.h>

// This is a simple wrapper to expose V8 symbols from Deno
void* get_v8_symbol(const char* name) {
    static void* deno_handle = NULL;
    if (!deno_handle) {
        deno_handle = dlopen("$DENO_PATH", RTLD_LAZY | RTLD_GLOBAL);
        if (!deno_handle) {
            fprintf(stderr, "Failed to load Deno: %s\n", dlerror());
            return NULL;
        }
    }
    return dlsym(deno_handle, name);
}
EOF

  # Compile the wrapper
  gcc -shared -fPIC -o libv8_wrapper.so v8_wrapper.c -ldl
  
  # Copy to the target directory
  cp libv8_wrapper.so "$OLDPWD/v8_extract/lib/"
  
  # Create a header file
  cat > "$OLDPWD/v8_extract/include/v8_wrapper.h" << EOF
#ifndef V8_WRAPPER_H
#define V8_WRAPPER_H

#ifdef __cplusplus
extern "C" {
#endif

void* get_v8_symbol(const char* name);

#ifdef __cplusplus
}
#endif

#endif /* V8_WRAPPER_H */
EOF
fi

# Clean up
cd "$OLDPWD"
rm -rf "$TMP_DIR"

echo "V8 extraction complete. Libraries in v8_extract/lib, headers in v8_extract/include"
