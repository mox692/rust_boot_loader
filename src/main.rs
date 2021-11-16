// 標準ライブラリをリンクしない.
#![no_std]
// 通常のエントリポイント(start)を使用しない.
// ref: https://os.phil-opp.com/ja/freestanding-rust-binary/#start-attribute
#![no_main]
#![feature(panic_info_message)]
#![feature(custom_test_frameworks)]
// MEMO: ここのcrate keywordは、root crateからの相対pathを示す
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
mod serial;
mod writer;

// リンカに_startというシンボルでエントリを渡すために、コンパイラにmanglingを禁止させる
// ref: https://en.wikipedia.org/wiki/Name_mangling

// Cの呼び出し規約を使用するために extern キーワードを使用. (todo: 調査.)
// ref: https://doc.rust-lang.org/nomicon/ffi.html
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    // Test実行時(cargo test ...)のみ、test_main()の先の
    // instructionが出力される.
    // Ref: https://doc.rust-jp.rs/rust-by-example-ja/attribute/cfg.html
    #[cfg(test)]
    test_main();

    loop {}
}

// panicハンドラの実装.
// no_std環境では、標準ライブラリに付属するpanicハンドラが使用できないので、
// 自作する. 戻り値はnever型 ref: https://doc.rust-lang.org/1.30.0/book/first-edition/functions.html#diverging-functions
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// x86_64クレートのPort:writeのラップ.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

pub trait Testable {
    // TODO: 調べる.
    fn run(&self) -> ();
}
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
#[test_case]
fn trivial_assertion() {
    assert_eq!(0, 0);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// MEMO: アライメントの指定
// https://doc.rust-jp.rs/rust-nomicon-ja/repr-rust.html
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
