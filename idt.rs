use core::intrinsics::size_of;
use core::slice::SliceExt;

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct IDT {
    pub base_lo: u16,
    pub sel: u16,
    pub always0: u8,
    pub flags: u8,
    pub base_hi: u16,
}

#[repr(C, packed)]
pub struct IDTR {
    pub limit: u16,
    pub base: usize,
}

static mut idt: [IDT; 256] = [IDT {base_lo: 0, sel: 0, always0: 0, flags: 0, base_hi: 0}; 256];

#[no_mangle]
pub static mut _idtr: IDTR = IDTR {limit: 0, base: 0};

extern "C" {
    fn idt_load();
}


pub fn set_gate(num: usize, base: usize, sel: u16, flags: u8) {
    unsafe {
        let bas_lo = ((base & 0xFFFF0000) >> 16) as u16;
        let bas_hi = (base & 0x0000FFFF) as u16;
        idt.get_unchecked_mut(num).base_lo = bas_lo;
        idt.get_unchecked_mut(num).base_hi = bas_hi;
        idt.get_unchecked_mut(num).sel = sel;
        idt.get_unchecked_mut(num).flags = flags;
    }
}

pub fn install_idt() {
    unsafe {
        _idtr.limit = (size_of::<IDT>() * 256 - 1) as u16;
        _idtr.base = &_idtr as *const IDTR as usize;
        idt_load()
    }
}
