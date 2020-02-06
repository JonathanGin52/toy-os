#![no_std] // Don't link Rust stdlib
#![no_main] // Disable Rust-level entry point

use core::panic::PanicInfo;

// Create custom panic handler since normal panic is part of stdlib
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Create custom entry point - linker looks for a function named '_start' by default
#[no_mangle] // Don't mangle name of this function
pub extern "C" fn _start() -> ! {
    loop {}
}
