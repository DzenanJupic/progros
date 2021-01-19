use super::super::{buffer::Buffer, Char};

pub struct RawWriter {
    pos_x: usize,
    pos_y: usize,
    buffer: &'static mut Buffer,
}

impl RawWriter {
    /// Safety:
    /// This functions is only safe to call, when it's guaranteed that nobody else has a mutable 
    /// reference to the VGA buffer, or when all mutable references are synchronized.
    pub(super) const unsafe fn new() -> Self {
        Self {
            pos_x: 0,
            pos_y: 0,
            buffer: Buffer::new(),
        }
    }

    pub fn write_str(&mut self, s: &str) {
        self.write_bytes(s.bytes())
    }

    pub fn write_bytes(&mut self, bytes: impl Iterator<Item=u8>) {
        for byte in bytes {
            self.write_byte(byte);
        }
    }

    pub fn write_chars(&mut self, chars: impl Iterator<Item=Char>) {
        for char in chars {
            self.write_char(char);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.write_char(Char::new_bw(byte))
    }

    pub fn write_char(&mut self, mut char: Char) {
        match char.char {
            b'\n' => {
                self.new_line();
                self.carriage_return();
            },
            b'\r' => self.carriage_return(),
            b'\t' => self.tabulator(),
            _ => {
                // replace non printable chars
                if !(0x20..0x7e).contains(&char.char) {
                    char.char = 0xfe;
                }

                self.buffer.write_char(char, self.pos_x, self.pos_y);
                self.inc_pos();
            }
        }
    }

    pub fn tabulator(&mut self) {
        self.skip(4)
    }

    pub fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.inc_pos();
        }
    }

    pub fn inc_pos(&mut self) {
        if self.pos_x == Buffer::WIDTH - 1 {
            self.new_line();
            self.carriage_return();
        } else {
            self.pos_x += 1;
        }
    }

    pub fn carriage_return(&mut self) {
        self.pos_x = 0;
    }

    pub fn new_line(&mut self) {
        if self.pos_y == Buffer::HEIGHT - 1 {
            self.buffer.shift_rows();
        } else {
            self.pos_y += 1;
        }
    }
}

impl core::fmt::Write for RawWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_byte(c as u8);
        Ok(())
    }
}
