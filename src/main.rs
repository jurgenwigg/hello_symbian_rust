// Simple hello world app 
#![feature(restricted_std)]
#![feature(lang_items)]
fn main() {
    println!("Hello Rust on Symbian^3!");
}
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}