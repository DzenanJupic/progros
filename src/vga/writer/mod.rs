use core::fmt::Arguments;

use super::Char;

pub use self::sync_raw_writer::SyncRawWriter;

mod raw_writer;
mod sync_raw_writer;

pub struct Writer {
    raw: SyncRawWriter
}

impl Writer {
    pub const fn new() -> Self {
        Self {
            raw: SyncRawWriter::new()
        }
    }

    pub fn write_str(&mut self, s: &str) {
        self.raw.lock().write_str(s)
    }

    pub fn write_bytes(&mut self, bytes: impl Iterator<Item=u8>) {
        self.raw.lock().write_bytes(bytes)
    }

    pub fn write_chars(&mut self, chars: impl Iterator<Item=Char>) {
        self.raw.lock().write_chars(chars)
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.raw.lock().write_byte(byte)
    }

    pub fn write_char(&mut self, char: Char) {
        self.raw.lock().write_char(char)
    }

    pub fn tabulator(&mut self) {
        self.raw.lock().tabulator();
    }

    pub fn skip(&mut self, n: usize) {
        self.raw.lock().skip(n);
    }

    pub fn inc_pos(&mut self) {
        self.raw.lock().inc_pos();
    }

    pub fn carriage_return(&mut self) {
        self.raw.lock().carriage_return();
    }

    pub fn new_line(&mut self) {
        self.raw.lock().new_line();
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_str(s);
        Ok(())
    }

    fn write_char(&mut self, c: char) -> core::fmt::Result {
        self.write_byte(c as u8);
        Ok(())
    }
}
