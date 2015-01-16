#![no_std]
#![allow(ctypes)]
#![feature(lang_items)]
#![feature(intrinsics)]
#![feature(asm)]
pub extern crate core;

pub mod std {pub use core::*;}
pub mod utils;
pub mod io;
pub mod idt;


#[no_mangle]
#[no_stack_check]
pub extern "C" fn main() {
    io::clear_screen(io::Black);
    let mut x = io::Cell {
        x : -1,
        y : 0,
        bg : io::Black as u16,
        fg : io::White as u16, 
    };
    //io::put_char(&x, 'f');
    x.puts("hello world!\nwow");
    //io::r_write((x as *mut io::Cell), unsafe{utils::transmute("hello world!")});
}
