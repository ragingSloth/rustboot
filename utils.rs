#![macro_escape]
pub use self::Option::*;

pub fn outb(addr: u8, data: u8) {
    unsafe {
        asm!("outb $0, $1",
            :
            : "r"(addr), "r"(data)
            :
            : "volatile"
        );
    }
}
pub fn inb(addr: u8) -> u8 {
    unsafe {
        let mu
        asm!("outb $0, $1",
            :
            : "r"(addr)
            : "=r"(data)
            : "volatile"
        );
    }
}

//////////////////////////////////////////////////////
//lang_items
//////////////////////////////////////////////////////
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {}  }
#[lang="sized"]
pub trait Sized for Sized? {}
#[lang="copy"]
pub trait Copy for Sized? {}
#[lang="iterator"]
pub trait Iterator<T>{
    fn next(&mut self) -> Option<T>;
}

extern "rust-intrinsic" {
    pub fn transmute<T, U>(x: T) -> U;
    pub fn offset<T>(dst: *const T, offset: int) -> *const T;
    pub fn size_of<T>() -> uint;
}

//////////////////////////////////////////////////////
//structs, traits, enums, impls
//////////////////////////////////////////////////////
pub enum Option<T> {
    None,
    Some(T)
}

pub struct IntRange {
    pub cur: int,
    pub max: int,
    pub inc: int,
}

impl Iterator<int> for IntRange{
    #[no_stack_check]
    fn next(&mut self) -> Option<int> {
        if self.cur != self.max {
            if self.cur > self.max == self.inc >0 {
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
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8, n: uint) {
    for i in range!(n as int) {
        *(offset(dest as *const u8, i) as *mut u8) = *offset(src, i);
    }
}

#[no_mangle]
#[no_stack_check]
pub unsafe extern fn memmove(dest: *mut u8, src: *const u8, n: uint) {
    let mut block = match src < dest as *const u8 {
        true => range!((n as int) - 1, -1),
        false => range!(n as int),
    };
    for i in block {
        *(offset(dest as *const u8, i) as *mut u8) = *offset(src, i);
    }
}

#[no_mangle]
#[no_stack_check]
pub unsafe extern fn memset( s: *mut u8, c: i32, n: uint) -> *mut u8 {
    for i in range!(n as int) {
        *(offset(s as *const u8, i) as *mut u8) = c as u8;
    }
    return s;
}

#[no_mangle]
#[no_stack_check]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: uint) -> i32 {
    for i in range!(n as int) {
        let a = *offset(s1, i);
        let b = *offset(s2, i);
        if a != b {
            return  (a - b) as i32;
        }
    }
    return 0;
}
