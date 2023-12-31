#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
use core::panic::PanicInfo;
mod vga_buffer;


static HELLO: &[u8] = b"epic gamer!";

#[no_mangle]
pub extern "C" fn _start() -> ! { // this function is the entry point, since the linker looks for a function
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
/// eee
/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
