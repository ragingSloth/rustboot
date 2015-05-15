#![feature(asm, lang_items, no_std)]
#[no_std]
#[allow(missing_copy_implementations, unused_imports, unstable, improper_ctypes)]
extern crate core;

pub mod std {pub use core::*;}
pub mod utils;
pub mod io;
pub mod idt;


#[no_mangle]
#[no_stack_check]
pub extern "C" fn main() {
    io::clear_screen(io::Red);
    let mut x = io::Cell {
        x : 0,
        y : 0,
        bg : io::Red as u16,
        fg : io::Black as u16, 
    };
    x.puts("hello world!");
    loop {}
}
