// 標準ライブラリをリンクしない.
#![no_std]
// 通常のエントリポイント(start)を使用しない.
// ref: https://os.phil-opp.com/ja/freestanding-rust-binary/#start-attribute
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// リンカに_startというシンボルでエントリを渡すために、コンパイラにmanglingを禁止させる
// ref: https://en.wikipedia.org/wiki/Name_mangling

// Cの呼び出し規約を使用するために extern キーワードを使用. (todo: 調査.)
// ref: https://doc.rust-lang.org/nomicon/ffi.html
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // offsetはptr型のmethodで、引数にisizeをとる.
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = if i % 2 == 0 { 0xc } else { 0x9 }
        }
    }

    loop {}
}

// panicハンドラの実装.
// no_std環境では、標準ライブラリに付属するpanicハンドラが使用できないので、
// 自作する. 戻り値はnever型 ref: https://doc.rust-lang.org/1.30.0/book/first-edition/functions.html#diverging-functions
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
