pub use self::Colors::*;

use utils::{transmute, IntRange};

#[deriving(Copy)]
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

#[deriving(Copy)]
pub struct Cell {
    pub x: isize,
    pub y: isize,
    pub bg: u16,
    pub fg: u16 
}

impl Cell {
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
        self.x = -1;
    }

    #[no_stack_check]
    pub fn put_char(&self, ch: char) {
        let chr = ch as u16;
        let fg = self.fg << 8;
        let bg = self.bg << 12;
        let fill: isize = self.get_fill();
        unsafe{
            *((0xb8000 + fill) as *mut u16) = chr | fg | bg;
        }
    }

    #[no_stack_check]
    pub fn puts(&mut self, s: &str) {
        let ss: &[u8] = unsafe {transmute(s)};
        let mut st = ss;
        loop {
            st = match st { 
                [x, xs..] => {
                    if x == '\n' as u8 {
                        self.new_line();
                    }
                    else {
                        self.incr();
                        self.put_char((x as char));
                    }
                    xs
                }
                [] => break,
            };
        }
    }

}

#[no_stack_check]
pub fn clear_screen(background: Colors) {
    let color = background as u16;
    for i in range!(80*25is) {
        unsafe{
            *((0xb8000 + i*2) as *mut u16) = color << 12;
        }
    }
}

