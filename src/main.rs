#![feature(start)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;

#[no_mangle]
extern "C" fn _start() -> ! {
    for (i, &byte) in b"Hello World!".iter().enumerate() {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = byte;
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0xb;
        }
    }


    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
