// TODO: 現状はVGA bufferを前提とした実装.
pub struct Writer {
    // VGA bufferのpointer.
    vga_buffer: *mut u8,
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
    pub fn write(&mut self, str: &'static str) {
        for (i, &byte) in str.as_bytes().iter().enumerate() {
            // MEMO: ここのcmp, もっと簡潔な書き方がありそうだが..
            if &byte.to_be_bytes() == b"\n" {
                self.new_line();
                continue;
            }
            unsafe {
                // offsetはptr型のmethodで、引数にisizeをとる.
                *self.vga_buffer.offset(self.cur_pos() as isize) = byte;
                *self.vga_buffer.offset(self.cur_pos() as isize + 1) =
                    if i % 2 == 0 { 0xc } else { 0x9 }
            }
            self.incre_offset(2);
        }
    }
}
impl Default for Writer {
    fn default() -> Self {
        return Self {
            vga_buffer: 0xb8000 as *mut u8,
            cur_x: 0,
            cur_y: 0,
        };
    }
}
