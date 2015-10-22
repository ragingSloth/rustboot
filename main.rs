#![no_std]
#![feature(asm, lang_items, no_std, core, core_intrinsics, core_slice_ext, core_str_ext)]
#[allow(missing_copy_implementations, unused_imports, improper_ctypes)]

pub mod utils;
pub mod io;
pub mod idt;
pub mod isr;
pub mod timer;
//pub mod gdt;


#[no_mangle]
#[no_stack_check]
pub extern "C" fn setup() {
    //gdt::install_gdt();
    unsafe {
        idt::irq_handlers[0] = timer::timer_handler;
    }
    idt::remap_irq();
    idt::install_idt();
    isr::install_isr();
}

#[no_mangle]
#[no_stack_check]
pub extern "C" fn main() {
    io::clear_screen(io::Red);
    let mut x = io::Cell::new();
    x.puts("hello world!");
    loop {}
}


//////////////////////////////////////////////////////
//lang_items
//////////////////////////////////////////////////////
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {}  }
