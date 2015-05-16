#![macro_use]
use core;
use core::prelude::{Option, None, Some};
use core::intrinsics::offset;
//use core::ptr::PtrExt;

pub fn outb(addr: u8, data: u8) {
    unsafe {
        asm!("outb %al, %dx"
            :
            : "{dx}"(addr), "{al}"(data)
            :
            : "volatile"
        );
    }
}
pub fn inb(addr: u8) -> u8 {
    let mut data: u8 = 0;
    unsafe {
        asm!("inb %dx, %al"
            : "={al}"(data)
            : "{dx}"(addr)
            :
            : "volatile"
        );
    }
    data } 

//////////////////////////////////////////////////////
//lang_items
//////////////////////////////////////////////////////
//#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
//#[lang = "eh_personality"] extern fn eh_personality() {}
//#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {}  }

//extern "rust-intrinsic" {
//    pub fn transmute<T, U>(x: T) -> U;
//    pub fn offset<T>(dst: *const T, offset: isize) -> *const T;
//    pub fn size_of<T>() -> usize;
//}

//////////////////////////////////////////////////////
//structs, traits, enums, impls
//////////////////////////////////////////////////////
pub struct IntRange {
    pub cur: isize,
    pub max: isize,
    pub inc: isize,
}

impl core::iter::Iterator for IntRange{
    type Item = isize;

    #[no_stack_check]
    fn next(&mut self) -> Option<isize> {
        if self.cur != self.max {
            if (self.cur > self.max) == (self.inc > 0) {
                self.cur = self.max;
                self.inc = 0;
            }
            self.cur += self.inc;
            Some(self.cur - self.inc)
        } else {
            None
        }
    }
}
//////////////////////////////////////////////////////
//macros
//////////////////////////////////////////////////////
#[macro_export]
macro_rules! range{
    ($e1: expr, $e2: expr) => ( 
                match $e2 > $e1 {
                    true => IntRange {cur: $e1, max: $e2, inc: 1},
                    false=> IntRange {cur: $e1, max: $e2, inc: -1},
                }
    );
    ($e1: expr) => (
            match $e1 >= 0 {
                true => IntRange {cur: 0, max: $e1, inc: 1},
                false => IntRange {cur: 0, max: $e1, inc: -1},
            }
    );
    ($e1: expr, $e2: expr, $e3: expr) => (
        IntRange {cur: $e1, max: $e2, inc: $e3}
    );
}
///////////////////////////////////////////////////////
//MEMORY
//////////////////////////////////////////////////////
#[no_mangle]
#[no_stack_check]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8{
    for i in 0..n {
        *dest.offset(i as isize) = *src.offset(i as isize);
    }
    return dest;
}

#[no_mangle]
#[no_stack_check]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8, n: usize)  -> *mut u8{
    let mut incr = match src < dest as *const u8 {
        true => {
            for i in 0..n {
                *dest.offset((n - i) as isize) = *src.offset((n - i) as isize);
            }
        },
        false => {
            for i in 0..n {
                *dest.offset(i as isize) = *src.offset(i as isize);
            }
        },
    };
    return dest;
}

#[no_mangle]
#[no_stack_check]
pub unsafe extern fn memset( s: *mut u8, c: i32, n: usize) -> *mut u8 {
    for i in 0..n {
        *s.offset(i as isize) = c as u8;
    }
    return s;
}

#[no_mangle]
#[no_stack_check]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return  (a - b) as i32;
        }
    }
    return 0;
}
