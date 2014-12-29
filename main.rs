#![no_std]
#![allow(ctypes)]
#![feature(lang_items)]
#![feature(globs)]

#[lang="sized"]
pub trait Sized for Sized? {}
#[lang="copy"]
pub trait Copy for Sized?{}


pub mod utils;

#[no_mangle]
#[no_split_stack]
pub extern "C" fn main() {
    utils::clear_screen(utils::Cyan);
}
