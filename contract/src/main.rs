#![no_std]
#![no_main]
#![feature(start)]
#![feature(lang_items)]

#[no_mangle]
#[start]
pub fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
