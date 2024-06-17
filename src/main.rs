// src/main.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod modules;
use modules::vga_buffer::buffer::Buffer;
use modules::vga_buffer::color::Color;
use modules::vga_buffer::color_code::ColorCode;
use modules::vga_buffer::writer::Writer;

// static HELLO: &[u8] =
//     b"Hello Ryan Something is bound to go wrong here I wonder what it will be huh!!!!! There could be over the line buffer somet thing else";

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    // let vga_buffer = 0xb8000 as *mut u8;

    // let foreground = Color::Blue;
    // let background = Color::White;
    // let color_code = ColorCode::new(foreground, background);

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = color_code.value();
    //     }
    // }
    print_something();
    loop {}
}

pub(self) fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Red, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string("Hello, World! Here is some text that is probably way too long to fit on one line. Let's see what happens!dsafasdfnasdmfnbm,asdnf,mnasdmf,na,smdfnmasdndmfnmasdfnasd,fna,msdnfm,asdnf,mnasdm,fn,masdfnmdasf");
}
