// https://doc.rust-lang.org/1.7.0/book/no-stdlib.html
#![feature(lang_items)]
#![feature(start)]
#![no_std]
#![no_main]
use core::panic::PanicInfo;

extern crate libc;

// ensure that this symbol is called `main` in the output
#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    // Since we are passing a C string the final null character is mandatory.
    const HELLO: &'static str = "Hello, world!\n\0";
    unsafe {
        libc::printf(HELLO.as_ptr() as *const _);
    }

    0
}

// #[lang = "eh_personality"]
// extern "C" fn eh_personality() {}

#[panic_handler]
fn _handle_panic(_info: &PanicInfo) -> ! {
    loop {}
}
