use utils;
use io;

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
struct Regs {
    gs: uint,
    fs: uint,
    es: uint,
    ds: uint,
    edi: uint,
    esi: uint,
    ebp: uint,
    esp: uint,
    ebx: uint,
    edx: uint,
    ecx: uint,
    eax: uint,
    int_no: uint,
    err_no: uint,
    eip: uint,
    cs: uint,
    eflags: uint,
    useresp: uint,
    ss: uint,
}

#[no_mangle]
static mut idt: [IDT, ..256] = [IDT {base1: 0, selector: 0, zero: 0, attrs: 0, base2: 0}, ..256];
static mut isrs: [extern "C" unsafe fn, ..32] = [isr0, isr1, isr2, isr3, isr4, isr5, isr6, isr7, isr8, isr9, isr10, isr11, isr12, isr13, isr14, isr15, isr16, isr17, isr18, isr19, isr20, isr21, isr22, isr23, isr24, isr25, isr26, isr27, isr28, isr29, isr30, isr31] 
static mut irqs: [extern "C" unsafe fn, ..32] = [irq0, irq1, irq2, irq3, irq4, irq5, irq6, irq7, irq8, irq9, irq10, irq11, irq12, irq13, irq14, irq15]
static messages = [
    "divide by zero\n",
    "debug\n",
    "non maskable interrupt\n",
    "breakpoint exception\n",
    "into detected overflow\n",
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
    "unknown interrupt\n",
    "coprocessor fault\n",
    "alignment check (i486+)\n",
    "machine check(pentium/i586+)\n",
    "reserved\n",
];

#[no_stack_check]
#[no_mangle]
pub extern "C" fn load_idt() {
    for i in range!(32) {
        set_descriptor(idt[i], isrs[i] as u32, 0x08, 0x8E);
    }
    for i in range!(32, 48) {
        set_descriptor(idt[i], irqs[i-32] as u32, 0x08, 0x8E);
    }
    outb(0x20, 0x11);
    outb(0xA0, 0x11);
    outb(0x21, 0x20);
    outb(0xA1, 0x28);
    outb(0x21, 0x04);
    outb(0xA1, 0x02);
    outb(0x21, 0x01);
    outb(0xA1, 0x01);
    outb(0x21, 0x0);
    outb(0xA1, 0x0);
}

#[no_stack_check]
#[no_mangle]
pub extern "C" fn _fault_handler(stack: Regs) {
    let idx = match r.int_no {
        x<19 => x,
        x>19 => 19,
    }
    if r.int_no < 32 {
    io::clear_screen(io::Black);
    let mut x = io::Cell {
        x : 0,
        y : 0,
        bg : io::Black as u16,
        fg : io::White as u16, 
    };
    io::puts(&mut x, messages[idx]);
    io::puts(&mut x, "Exception, halting...");
    loop {}
    }
}

#[no_stack_check]
fn set_descriptor(desc: &mut IDT, offset: u32, selector: u16, config: u8){
    desc.base1 = 0xFFFFu16 & offset;
    desc.base2 = 0xFFFFu16 & offset >> 16;
    desc.selector = selector;
    desc.zero = 0;
    desc.attrs = config;
}
/////////////////
//ISRs
/////////////////
extern {
    fn isr0();
    fn isr1();
    fn isr2();
    fn isr3();
    fn isr4();
    fn isr5();
    fn isr6();
    fn isr7();
    fn isr8();
    fn isr9();
    fn isr10();
    fn isr11();
    fn isr12();
    fn isr13();
    fn isr14();
    fn isr15();
    fn isr16();
    fn isr17();
    fn isr18();
    fn isr19();
    fn isr20();
    fn isr21();
    fn isr22();
    fn isr23();
    fn isr24();
    fn isr25();
    fn isr26();
    fn isr27();
    fn isr28();
    fn isr29();
    fn isr30();
    fn isr31(); 

    fn irq0();
    fn irq1();
    fn irq2();
    fn irq3();
    fn irq4();
    fn irq5();
    fn irq6();
    fn irq7();
    fn irq8();
    fn irq9();
    fn irq10();
    fn irq11();
    fn irq12();
    fn irq13();
    fn irq14();
    fn irq15();
}

