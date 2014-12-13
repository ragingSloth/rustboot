#![no_std]
#![allow(ctypes)]

#![feature(lang_items)]
#![feature(globs)]

pub use Color::*;
pub use Option::*;

#[lang="sized"]
pub trait Sized for Sized? {}

#[lang="copy"]
trait Copy for Sized?{}

pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Pink       = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    LightPink  = 13,
    Yellow     = 14,
    White      = 15,
}

pub enum Option<T> {
    None,
    Some(T)
}

struct IntRange {
    cur: int,
    max: int
}

impl IntRange {
    fn next(&mut self) -> Option<int> {
        if self.cur < self.max {
            self.cur += 1;
            Some(self.cur - 1)
        } else {
            None
        }
    }
}

fn range(lo: int, hi: int) -> IntRange {
    IntRange { cur: lo, max: hi }
}

fn clear_screen(background: Color) {
    let mut r = range(0, 80 * 25);
    let color = background as u16;
    loop{
        match r.next() {
            Some(x) => {
                unsafe {
                    *((0xb8000 + x * 2) as *mut u16) = color << 12 | 7u16 << 8 | 70 ;
                }
            },
            None =>{break}
        }
    }
}

#[no_mangle]
#[no_split_stack]
pub fn main() {
    clear_screen(Cyan);
}
