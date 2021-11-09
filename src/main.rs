// 標準ライブラリをリンクしない.
#![no_std]
// 通常のエントリポイント(start)を使用しない.
// ref: https://os.phil-opp.com/ja/freestanding-rust-binary/#start-attribute
#![no_main]

use core::panic::PanicInfo;

mod writer;

// リンカに_startというシンボルでエントリを渡すために、コンパイラにmanglingを禁止させる
// ref: https://en.wikipedia.org/wiki/Name_mangling

// Cの呼び出し規約を使用するために extern キーワードを使用. (todo: 調査.)
// ref: https://doc.rust-lang.org/nomicon/ffi.html
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut w = writer::Writer::new();
    w.write("Motoyuki Kimura");
    loop {}
}

// panicハンドラの実装.
// no_std環境では、標準ライブラリに付属するpanicハンドラが使用できないので、
// 自作する. 戻り値はnever型 ref: https://doc.rust-lang.org/1.30.0/book/first-edition/functions.html#diverging-functions
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
