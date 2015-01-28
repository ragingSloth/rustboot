use core::prelude::{Copy};
use core::mem::{size_of};

#[derive(Copy)]
#[repr(C, packed)]
struct GDT {
    limit1: u16,
    base1: u16,
    base2: u8,
    access: u8,
    granularity: u8,
    base3: u8,
}

#[repr(C, packed)]
struct GDTR {
    limit: u16,
    base: u32,
}

#[no_mangle]
static mut gdt: [GDT; 3] = [GDT {limit1: 0, base1: 0, base2: 0, access: 0, granularity: 0, base3: 0}; 3];

extern "C" {
    fn _gdt_flush();
    static mut _gdtr: GDTR;
}

#[no_stack_check]
#[no_mangle]
unsafe fn gdt_set_gate(entry: usize, base: u32, limit: u32, access: u8, granularity: u8) {
    gdt[entry].base1 = (base & 0xFFFF) as u16;
    gdt[entry].base2 = ((base >> 16) & 0xFF) as u8;
    gdt[entry].base3 = ((base >> 24) & 0xFF) as u8;

    gdt[entry].limit1 = (limit & 0xFFFF) as u16;
    gdt[entry].granularity = ((limit >> 16) & 0x0F) as u8;

    gdt[entry].granularity |= (granularity & 0xF0) as u8;
    gdt[entry].access = access as u8;
}

#[no_stack_check]
#[no_mangle]
pub unsafe extern "C" fn _load_gdt() {
    _gdtr.limit = (size_of::<GDT>() * 3 - 1) as u16;
    _gdtr.base = &mut gdt as *mut [GDT; 3] as u32;
    gdt_set_gate(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
    gdt_set_gate(2, 0, 0xFFFFFFFF, 0x92, 0xCF);
    _gdt_flush();
}
