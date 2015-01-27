use core::prelude::{Option, None, Some, SliceExt};
use core::panicking::*;
use core::mem::{size_of};
use utils;
use utils::{IntRange};
use io;

#[derive(Copy)]
#[repr(C, packed)]
struct IDT {
    base1: u16,
    selector: u16,
    zero: u8,
    attrs: u8,
    base2: u16,
}

#[repr(C, packed)]
struct IDTR {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
pub struct Regs {
    gs: u32,
    fs: u32,
    es: u32,
    ds: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    int_no: u32,
    err_no: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
    useresp: u32,
    ss: u32,
}

#[no_mangle]
extern {static mut idtr: IDTR;}
#[no_mangle]
static mut idt: [IDT; 256] = [IDT {base1: 0, selector: 0, zero: 0, attrs: 0, base2: 0} ;256];

static mut isrs: [unsafe extern "C" fn(); 32] = [_isr0, _isr1, _isr2, _isr3, _isr4, _isr5, _isr6, _isr7, _isr8, _isr9, _isr10, _isr11, _isr12, _isr13, _isr14, _isr15, _isr16, _isr17, _isr18, _isr19, _isr20, _isr21, _isr22, _isr23, _isr24, _isr25, _isr26, _isr27, _isr28, _isr29, _isr30, _isr31];
static mut irqs: [unsafe extern "C" fn(); 16] = [_irq0, _irq1, _irq2, _irq3, _irq4, _irq5, _irq6, _irq7, _irq8, _irq9, _irq10, _irq11, _irq12, _irq13, _irq14, _irq15];
static mut _irq_routines: [Option<unsafe extern "C" fn(Regs)>; 16] = [None; 16];
static MESSAGES: [&'static str; 21] = [
    "divide by zero\n",
    "debug\n",
    "non maskable isizeerrupt\n",
    "breakpoisize exception\n",
    "isizeo detected overflow\n",
    "out of bounds\n",
    "invalid opcode\n",
    "no coprocessor\n",
    "double fault\n",
    "coprocessor segment overrun\n",
    "exception\n",
    "bad tss\n",
    "segment not present\n",
    "stack fault\n",
    "general protection fault\n",
    "page fault\n",
    "unknown isizeerrupt\n",
    "coprocessor fault\n",
    "alignment check (i486+)\n",
    "machine check(pentium/i586+)\n",
    "reserved\n",
];

#[no_stack_check]
#[no_mangle]
pub unsafe extern "C" fn _load_idt() {
    for i in range!(32) {
        set_descriptor(&mut idt[i as usize], isrs[i as usize] as u32, 0x08, 0x8E);
    }
    for i in range!(32, 48) {
        set_descriptor(&mut idt[i as usize], irqs[i as usize - 32] as u32, 0x08, 0x8E);
    }
    utils::outb(0x20, 0x11);
    utils::outb(0xA0, 0x11);
    utils::outb(0x21, 0x20);
    utils::outb(0xA1, 0x28);
    utils::outb(0x21, 0x04);
    utils::outb(0xA1, 0x02);
    utils::outb(0x21, 0x01);
    utils::outb(0xA1, 0x01);
    utils::outb(0x21, 0x0);
    utils::outb(0xA1, 0x0);
    utils::outb(0x21,0xfd);
    utils::outb(0xa1,0xff);

    utils::outb(0x21,0xFD);
    utils::outb(0xA1,0xFF);    

    idtr.limit = (size_of::<IDT>() * 256 - 1) as u16;
    idtr.base = &mut idt as *mut[IDT; 256] as u32;
}

#[no_stack_check]
#[no_mangle]
pub extern "C" fn _fault_handler(stack: Regs){
    let idx = match stack.int_no >= 19 {
        false  => stack.int_no,
        true => 19,
    };
    if stack.int_no < 32 {
        io::clear_screen(io::Black);
        let mut x = io::Cell {
            x : 0,
            y : 0,
            bg : io::Black as u16,
            fg : io::White as u16, 
        };
        unsafe{x.puts(*messages.get_unchecked(idx as usize));}
        x.puts("Exception, halting...");
        loop {}
    }
}

#[no_stack_check]
#[no_mangle]
pub extern "C" fn _irq_handler(stack: Regs) {
    if stack.int_no > 7 {
        utils::outb(0xA0, 0x20);
    }
    utils::outb(0x20, 0x20);
    let callback = unsafe {_irq_routines.get_unchecked(stack.int_no as usize)};
    match *callback {
        None => (),
        Some(funk) => unsafe {funk(stack)},
    };
}

#[no_stack_check]
fn set_descriptor(desc: &mut IDT, offset: u32, selector: u16, config: u8){
    desc.base1 = 0xFFFFu16 & offset as u16;
    desc.base2 = 0xFFFFu16 & (offset >> 16) as u16;
    desc.selector = selector;
    desc.zero = 0;
    desc.attrs = config;
}
/////////////////
//ISRs
/////////////////
extern {
    fn _isr0();
    fn _isr1();
    fn _isr2();
    fn _isr3();
    fn _isr4();
    fn _isr5();
    fn _isr6();
    fn _isr7();
    fn _isr8();
    fn _isr9();
    fn _isr10();
    fn _isr11();
    fn _isr12();
    fn _isr13();
    fn _isr14();
    fn _isr15();
    fn _isr16();
    fn _isr17();
    fn _isr18();
    fn _isr19();
    fn _isr20();
    fn _isr21();
    fn _isr22();
    fn _isr23();
    fn _isr24();
    fn _isr25();
    fn _isr26();
    fn _isr27();
    fn _isr28();
    fn _isr29();
    fn _isr30();
    fn _isr31(); 

    fn _irq0();
    fn _irq1();
    fn _irq2();
    fn _irq3();
    fn _irq4();
    fn _irq5();
    fn _irq6();
    fn _irq7();
    fn _irq8();
    fn _irq9();
    fn _irq10();
    fn _irq11();
    fn _irq12();
    fn _irq13();
    fn _irq14();
    fn _irq15();
}

