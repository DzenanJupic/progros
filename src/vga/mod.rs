use core::fmt::Write;

pub use writer::Writer;

pub mod buffer;
pub mod writer;

#[macro_export]
macro_rules! println {
    () => ( $crate::print!("\n"); );
    ($($arg:tt)*) => ( $crate::print!("{}\n", format_args!($($arg)*)); );
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ( $crate::vga::_print(format_args!($($arg)*)); );
}

#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    // [`Writer::new`] is const, so there should be no performance cost here
    Writer::new().write_fmt(args).unwrap();
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Char {
    char: u8,
    color: ColorCode,
}

impl Char {
    const ZERO: Self = Self::new(0, ColorCode::ZERO);

    pub const fn new(char: u8, color: ColorCode) -> Self {
        Self {
            char,
            color,
        }
    }

    pub const fn new_bw(char: u8) -> Self {
        Self {
            char,
            color: ColorCode::BW,
        }
    }
}

impl Default for Char {
    fn default() -> Self {
        Self {
            char: 0,
            color: ColorCode::BW,
        }
    }
}

impl From<u8> for Char {
    fn from(c: u8) -> Self {
        Self::new_bw(c)
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorCode(u8);

impl ColorCode {
    const BW: Self = Self::new(Color::White, Color::Black);
    const ZERO: Self = Self(0);

    pub const fn new(foreground: Color, background: Color) -> Self {
        Self((background as u8) << 4 | foreground as u8)
    }
}

#[repr(u8)]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
