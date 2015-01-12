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
    pub x: int,
    pub y: int,
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

    pub fn get_fill(&self) -> int {
        return (self.y * 80 + self.x) * 2;
    }

    pub fn new_line(&mut self) {
        self.y += 1;
        self.x = -1;
    }
}

#[no_stack_check]
pub fn clear_screen(background: Colors) {
    let color = background as u16;
    for i in range!(80*25i) {
        unsafe{
            *((0xb8000 + i*2) as *mut u16) = color << 12;
        }
    }
}

#[no_stack_check]
pub fn put_char(info: &Cell, ch: char) {
    let chr = ch as u16;
    let fg = info.fg << 8;
    let bg = info.bg << 12;
    let fill: int = info.get_fill();
    unsafe{
        *((0xb8000 + fill) as *mut u16) = chr | fg | bg;
    }
}

#[no_stack_check]
pub fn puts(head: &mut Cell, s: &str) {
    let ss: &[u8] = unsafe {transmute(s)};
    let mut st = ss;
    loop {
        st = match st { 
            [x, xs..] => {
                if x == '\n' as u8 {
                    head.new_line();
                }
                else {
                    head.incr();
                    put_char(head, (x as char));
                }
                xs
            }
            [] => break,
        };
    }
}
