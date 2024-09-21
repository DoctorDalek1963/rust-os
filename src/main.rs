//! `rust_os` is a kernel written in Rust based on Philipp Oppermann's blog series.

#![no_std]
#![no_main]

mod vga_buffer;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use self::vga_buffer::{Color, ColorCode, Writer};
    use core::fmt::Write;

    let mut writer = Writer::new(ColorCode::new(Color::White, Color::Black));

    write!(writer, r"The numbers are {} and {}

This is some more text. Some placeholder text, if you will. The idea here is to have a really long line that forces some line wrapping.

And then
some
manual
newlines

And some non-ASCII: Ελλενικα
    ", 42, 1.0 / 3.0).unwrap();

    #[allow(clippy::empty_loop)]
    loop {}
}
