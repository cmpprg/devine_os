use super::buffer::{Buffer, BUFFER_HEIGHT, BUFFER_WIDTH};
use super::color_code::ColorCode;
use super::screen_character::ScreenChar;

const ASCII_PRINTABLE_START: u8 = 0x20;
const ASCII_PRINTABLE_END: u8 = 0x7e;
const NEWLINE: u8 = b'\n';
const REPLACEMENT_BYTE: u8 = 0xfe;

pub struct Writer {
    pub(crate) column_position: usize,
    pub(crate) color_code: ColorCode,
    pub(crate) buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, string: &str) {
        for byte in string.bytes() {
            match byte {
                // printable ASCII byte or newline
                ASCII_PRINTABLE_START..=ASCII_PRINTABLE_END | NEWLINE => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(REPLACEMENT_BYTE),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            NEWLINE => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        /* TODO */
    }
}
