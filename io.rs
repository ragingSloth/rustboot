pub use self::Colors::*;

use utils::IntRange;
//use core::prelude::{SliceExt, StrExt};

#[repr(u16)]
pub enum Colors {
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

pub struct Cell {
    pub x: isize,
    pub y: isize,
    pub bg: u16,
    pub fg: u16 
}

impl Cell {
    pub fn new() -> Self {
    Cell {
            x : 0,
            y : 0,
            bg : Red as u16,
            fg : Black as u16, 
        }
    }
    pub fn incr(&mut self) {
        if self.x == 80 {
            self.new_line();
        }
        self.x += 1;
    }

    pub fn get_fill(&self) -> isize {
        return (self.y * 80 + self.x) * 2;
    }

    pub fn new_line(&mut self) {
        self.y += 1;
        self.x = 0;
    }

    #[no_stack_check]
    #[no_mangle]
    pub fn put_char(&mut self, chr: char) {
        if chr == '\n' {
            self.new_line();
            return ();
        }
        let ch = chr as u16;
        let fg = self.fg << 8;
        let bg = self.bg << 12;
        let fill: isize = self.get_fill();
        unsafe {
            *((0xb8000 + fill) as *mut u16) = ch | fg | bg;
        }
        self.incr();
    }

    #[no_stack_check]
    #[no_mangle]
    pub fn puts(&mut self, s: &str) {
        for c in s.as_bytes().iter() {
            self.put_char(*c as char);
        }
    }

}

#[no_stack_check]
#[no_mangle]
pub fn clear_screen(background: Colors) {
    let color = background as u16;
    for i in range!(2000) {
        unsafe{
            *((0xb8000 + i*2) as *mut u16) = color << 12;
        }
    }
}

