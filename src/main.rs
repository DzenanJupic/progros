#![feature(bool_to_option, const_mut_refs, const_raw_ptr_deref, asm, naked_functions)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::vga::buffer::Buffer;
use crate::vga::Char;

mod vga;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
extern "C" fn _start() -> ! {
    println!("Hello {}!", "World");

    loop { hlt() }
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop { hlt() }
}

#[naked]
fn hlt() {
    unsafe {
        asm!("HLT")
    }
}
