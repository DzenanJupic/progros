use core::fmt;
use spin::Mutex;
use volatile::Volatile;
use lazy_static::lazy_static;

const BUFFER: i32 = 0xb8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(ColorCode::new(
        Color::Green, Color::Black
    )));
}

pub fn print_something() {
    use fmt::Write;
    WRITER.lock().prints("Hallo Dzenan!\n");
    write!(WRITER.lock(), "Die Antowort is wieder {}\n", 42).unwrap();
    WRITER.lock().prints("\n\nBis morgen!");
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(color_code: ColorCode) -> Self {
        Self {
            column_position: 0,
            color_code,
            buffer: unsafe { &mut *(BUFFER as *mut Buffer) }
        }
    }

    pub fn prints(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20 ..= 0x7e | b'\n' => self.printb(byte),
                _ => self.printb(0xfe)
            }
        }
    }

    pub fn printb(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),

            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT-1;
                let col = self.column_position;

                self.write_byte(byte, row, col);

                self.column_position += 1;
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8, row: usize, col: usize) {
        let character = ScreenChar::new(
            byte,
            self.color_code
        );
        self.buffer.chars[row][col].write(character);
    }

    pub fn read_byte(&self, row: usize, col: usize) -> u8 {
        let character = self.buffer.chars[row][col].read();
        character.ascii_character
    }

    fn write_char(&mut self, chararcter: ScreenChar, row: usize, col: usize) {
        self.buffer.chars[row][col].write(chararcter);
    }

    fn read_char(&self, row: usize, col: usize) -> ScreenChar {
        self.buffer.chars[row][col].read()
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.read_char(row, col);
                self.write_char(character, row-1, col);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::blank();
        for col in 0..BUFFER_WIDTH {
            self.write_char(blank, row, col);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.prints(s);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

impl ScreenChar {
    pub fn new(ascii_character: u8, color_code: ColorCode) -> Self {
        Self {
            ascii_character,
            color_code
        }
    }

    pub fn blank() -> Self {
        Self {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Black, Color::Black)
        }
    }
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}