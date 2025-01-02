#![cfg_attr(not(any(feature = "native-simulator", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "native-simulator", test))]
extern crate alloc;
mod entry;
mod error;

pub fn program_entry() -> i8 {
    match entry::main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}
