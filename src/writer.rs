// TODO: 現状はVGA bufferを前提とした実装.
pub struct Writer {
    // VGA bufferのpointer.
    vga_buffer: *mut u8,
    // vga_bufferからのoffset.
    offset: usize,
}

impl Writer {
    pub fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }
    pub fn write(&mut self, str: &'static str) {
        for (i, &byte) in str.as_bytes().iter().enumerate() {
            unsafe {
                // offsetはptr型のmethodで、引数にisizeをとる.
                *self.vga_buffer.offset(self.offset as isize) = byte;
                *self.vga_buffer.offset(self.offset as isize + 1) =
                    if i % 2 == 0 { 0xc } else { 0x9 }
            }
            self.offset += 2;
        }
    }
}
impl Default for Writer {
    fn default() -> Self {
        return Self {
            vga_buffer: 0xb8000 as *mut u8,
            offset: 0,
        };
    }
}
