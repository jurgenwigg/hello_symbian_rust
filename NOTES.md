# Cargo Build command
`cargo +nightly build -Z unstable-options -Zbuild-std --target arm-nokia-none-gnueabihf.json`

# rustc build command
`rustc --target arm-nokia-none-gnueabihf.json -C linker=arm-none-symbianelf-gcc src/main.rs`

# Additional notes
- maybe building symbian compiler of rust would be necesary