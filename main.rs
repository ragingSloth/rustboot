#![no_std]
#![allow(ctypes)]
#![feature(lang_items)]
#![feature(globs)]
#![feature(intrinsics)]


#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {}  }
#[lang="sized"]
pub trait Sized for Sized? {}
#[lang="copy"]
pub trait Copy for Sized? {}

pub mod utils;
pub mod io;

#[no_mangle]
#[no_stack_check]
pub extern "C" fn main() {
    io::clear_screen(io::Black);
    let ref mut x = io::Cell {
        x : 1,
        y : 0,
        bg : io::Black as u16,
        fg : io::White as u16, 
    };
    io::put_char(x, 'f');
    //io::put(&mut x, "hello world!");
}
