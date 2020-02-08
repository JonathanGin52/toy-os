#![no_std] // Don't link Rust stdlib
#![no_main] // Disable Rust-level entry point

static HELLO: &[u8] = b"Hello World!";
use core::panic::PanicInfo;

// Create custom entry point - linker looks for a function named '_start' by default
#[no_mangle] // Don't mangle name of this function
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// Create custom panic handler since normal panic is part of stdlib
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
