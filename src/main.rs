#![no_std] // don't link the Rust standard Library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Ohishi Izumi Suki\n Sushi\n")
        .unwrap();
    write!(vga_buffer::WRITER.lock(), "Ohishi Izumi Suko").unwrap();
    loop {}
}

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
