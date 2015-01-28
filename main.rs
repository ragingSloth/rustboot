#![no_std]
#![allow(improper_ctypes)]
#![feature(lang_items)]
#![feature(intrinsics)]
#![feature(asm)]
#![allow(missing_copy_implementations, unused_imports, unstable)]
extern crate core;

pub mod std {pub use core::*;}
pub mod utils;
pub mod io;
pub mod idt;
//pub mod gdt;


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
