#![no_std]
#![no_main]

mod vga_buffer;
use vga_buffer::{
    Writer,
    ColourCode,
    Colour,
    Buffer
};

use core::{panic::PanicInfo, fmt::Write};

#[panic_handler]
fn panic (info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("First line");
    println!("Second line");
    println!("Hello world from new println macro!!");

    panic!("we can now do panic messages, pretty nice!");

    loop {}
}
