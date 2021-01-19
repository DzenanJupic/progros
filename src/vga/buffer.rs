use super::Char;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Char; Buffer::WIDTH]; Buffer::HEIGHT]
}

impl Buffer {
    pub const HEIGHT: usize = 25;
    pub const WIDTH: usize = 80;
    pub const CHARS: usize = Self::WIDTH * Self::HEIGHT;
    pub const POINTER: *mut Char = 0xb8000 as *mut Char;

    /// Safety:
    /// This functions is only safe to call, when it's guaranteed that nobody else has a mutable 
    /// reference to the VGA buffer, or when all mutable references are synchronized. 
    pub const unsafe fn new() -> &'static mut Self {
        &mut *(Self::POINTER as *mut Buffer)
    }

    pub fn write_char(&mut self, char: Char, x: usize, y: usize) {
        // SAFETY:
        // The memory location is in the VGA buffer, and therefore valid.
        unsafe {
            core::ptr::write_volatile(
                &mut self.chars[y][x],
                char,
            )
        }
    }

    pub fn shift_rows(&mut self) {
        self.chars.rotate_left(1);
        self.chars[Self::HEIGHT - 1] = [Char::ZERO; Self::WIDTH];
    }
}
