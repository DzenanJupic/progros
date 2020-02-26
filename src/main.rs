#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;

mod vga_buffer;


#[no_mangle] // makes function name not change
extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}



#[panic_handler] // gets invoked if program panics (should never happen)
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
