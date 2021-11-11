// TODO: 現状はVGA bufferを前提とした実装.
use core::fmt;

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    /// A global `Writer` instance that can be used for printing to the VGA text buffer.
    ///
    /// Used by the `print!` and `println!` macros.
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
}

pub struct Writer {
    // VGA bufferのpointer.
    vga_buffer: usize,
    // vga_bufferからのoffset.
    cur_x: usize,
    cur_y: usize,
}
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 160;
impl Writer {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }
    // cur_xとcur_yから、vga_bufferからのoffsetを計算.
    pub fn cur_pos(&self) -> usize {
        return self.cur_y * BUFFER_WIDTH + self.cur_x;
    }
    // vga_bufferを*mut u8に変換.
    unsafe fn to_addr(&self) -> *mut u8 {
        self.vga_buffer as *mut u8
    }
    pub fn incre_offset(&mut self, offset: usize) {
        let x = self.cur_x + offset;
        if x > BUFFER_WIDTH {
            let overflow = x - BUFFER_WIDTH;
            self.cur_y += 1;
            self.cur_x = overflow;
            return;
        }
        self.cur_x = x;
        return;
    }
    pub fn new_line(&mut self) {
        self.cur_x = 0;
        self.cur_y += 1;
    }
    pub fn write(&mut self, str: &str) {
        for (i, &byte) in str.as_bytes().iter().enumerate() {
            // MEMO: ここのcmp, もっと簡潔な書き方がありそうだが..
            if &byte.to_be_bytes() == b"\n" {
                self.new_line();
                continue;
            }
            unsafe {
                // offsetはptr型のmethodで、引数にisizeをとる.
                *self.to_addr().offset(self.cur_pos() as isize) = byte;
                *self.to_addr().offset(self.cur_pos() as isize + 1) =
                    if i % 2 == 0 { 0xc } else { 0x9 }
            }
            self.incre_offset(2);
        }
    }
}
impl Default for Writer {
    fn default() -> Self {
        return Self {
            // vga_buffer: 0xb8000 as *mut u8,
            vga_buffer: 0xb8000,
            cur_x: 0,
            cur_y: 0,
        };
    }
}
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

/// Like the `print!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::writer::_print(format_args!($($arg)*)));
}

/// Like the `println!` macro in the standard library, but prints to the VGA text buffer.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// Prints the given formatted string to the VGA text buffer through the global `WRITER` instance.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args);
}
