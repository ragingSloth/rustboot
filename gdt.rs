use core::slice::SliceExt;
use core::intrinsics::size_of;

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct GDT {
    limit: u16,
    base_lo: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_hi: u8
}

#[repr(C, packed)]
struct GDTR {
    limit: u16,
    base: usize
}

static mut gdt: [GDT; 3] = [GDT {limit: 0, base_lo: 0, base_mid: 0, access: 0, granularity: 0, base_hi:0}; 3];
static mut gdtr: GDTR = GDTR {limit: 0, base: 0};

pub fn install_gdt() {
    unsafe {
        gdtr.limit = (size_of::<GDT>() * 5 - 1) as u16;
        gdtr.base  = &gdtr as *const _ as usize;
        set_gate(0, 0, 0, 0, 0);
        set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
        set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
        //set_gate(3, 0, 0xFFFFFFFF, 0xFA, 0xCF);
        //set_gate(4, 0, 0xFFFFFFFF, 0xF2, 0xCF);
        gdt_flush(&gdtr as *const _ as u32);
    }
}

unsafe fn set_gate(num: usize, base: usize, limit: usize, access: u8, granularity: u8) {
        gdt.get_unchecked_mut(num).base_lo = (base & 0xFFFF) as u16;
        gdt.get_unchecked_mut(num).base_mid = ((base >> 16) & 0xFF) as u8;
        gdt.get_unchecked_mut(num).base_hi = ((base >> 24) & 0xFF) as u8;
        gdt.get_unchecked_mut(num).limit = (limit & 0xFFFF) as u16;
        gdt.get_unchecked_mut(num).granularity = ((limit >> 16) & 0x0F) as u8;
        gdt.get_unchecked_mut(num).granularity |= granularity & 0xF0;
        gdt.get_unchecked_mut(num).access = access;
}

extern {fn gdt_flush(p: u32);}
