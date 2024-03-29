#![no_std]
#![no_main]

use core::{ptr, arch::global_asm};
mod panic;

global_asm!(include_str!("start.S"));

#[no_mangle]
pub extern "C" fn not_main() {
    const UART0: *mut u8 = 0x0900_0000 as *mut u8;
    let out_str = b"AArch64 Bare Metal";
    for byte in out_str {
        unsafe {
            ptr::write_volatile(UART0, *byte);
        }
    }
}
