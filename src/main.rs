//! `rust_os` is a kernel written in Rust based on Philipp Oppermann's blog series.

#![no_std]
#![no_main]

mod vga_buffer;

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");

    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unimplemented!("Rust OS is not yet implemented");

    #[allow(clippy::empty_loop)]
    loop {}
}
