//! `rust_os` is a kernel written in Rust based on Philipp Oppermann's blog series.

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_infra::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![cfg_attr(test, allow(clippy::eq_op))]

#[cfg(test)]
pub mod test_infra;

mod serial;
mod vga_buffer;

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");

    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Rust OS!");

    #[cfg(test)]
    test_main();

    #[allow(clippy::empty_loop)]
    loop {}
}
