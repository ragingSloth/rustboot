use io;
use idt::set_gate;
use core::slice::SliceExt;

//courtesy of mvdnes element76
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Regs {
        _ds: u32, _edi: u32, _esi: u32, _ebp: u32, _esp: u32, _ebx: u32, _edx: u32, _ecx: u32, _eax: u32,
        int_no : u32,
        error_code: u32,
        _eip: u32, _cs: u32, _eflags: u32, _useresp: u32, _ss: u32,
}


static messages: [&'static str; 32] = [
    "Divide By Zero",
    "Debug",
    "Non Maskable Interrupt",
    "Breakpoint ",
    "Into Detected Overflow",
    "Out of Bounds",
    "Double Fault",
    "Coprocessor Segment Overrun",
    "Bad TSS",
    "Segment Not Present",
    "Stack Fault",
    "General Protection Fault",
    "Page Fault Exception",
    "Unknown Interrupt",
    "Coprocessor Fault",
    "Alignment Check",
    "achine Check",
    "Reserved", 
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    ];

#[no_mangle]
#[no_stack_check]
pub unsafe extern "C" fn fault_handler(r: Regs) {
    if r.int_no < 32 {
        io::clear_screen(io::Green);
        let mut x = io::Cell {
            x : 0,
            y : 0,
            bg : io::Green as u16,
            fg : io::Black as u16, 
        };
        if r._edx == 0x808 {
            x.puts("well, could be worse\n")
        }
        x.puts(messages.get_unchecked(r.int_no as usize));
        x.puts(" Exception. System Halted!");
        loop {}
    }
}

pub fn install_isr() {
    set_gate(0, (_isr0 as usize), 0x08, 0x8E);
    set_gate(1, (_isr1 as usize), 0x08, 0x8E);
    set_gate(2, (_isr2 as usize), 0x08, 0x8E);
    set_gate(3, (_isr3 as usize), 0x08, 0x8E);
    set_gate(4, (_isr4 as usize), 0x08, 0x8E);
    set_gate(5, (_isr5 as usize), 0x08, 0x8E);
    set_gate(6, (_isr6 as usize), 0x08, 0x8E);
    set_gate(7, (_isr7 as usize), 0x08, 0x8E);
    set_gate(8, (_isr8 as usize), 0x08, 0x8E);
    set_gate(9, (_isr9 as usize), 0x08, 0x8E);
    set_gate(10, (_isr10 as usize), 0x08, 0x8E);
    set_gate(11, (_isr11 as usize), 0x08, 0x8E);
    set_gate(12, (_isr12 as usize), 0x08, 0x8E);
    set_gate(13, (_isr13 as usize), 0x08, 0x8E);
    set_gate(14, (_isr14 as usize), 0x08, 0x8E);
    set_gate(15, (_isr15 as usize), 0x08, 0x8E);
    set_gate(16, (_isr16 as usize), 0x08, 0x8E);
    set_gate(17, (_isr17 as usize), 0x08, 0x8E);
    set_gate(18, (_isr18 as usize), 0x08, 0x8E);
    set_gate(19, (_isr19 as usize), 0x08, 0x8E);
    set_gate(20, (_isr20 as usize), 0x08, 0x8E);
    set_gate(21, (_isr21 as usize), 0x08, 0x8E);
    set_gate(22, (_isr22 as usize), 0x08, 0x8E);
    set_gate(23, (_isr23 as usize), 0x08, 0x8E);
    set_gate(24, (_isr24 as usize), 0x08, 0x8E);
    set_gate(25, (_isr25 as usize), 0x08, 0x8E);
    set_gate(26, (_isr26 as usize), 0x08, 0x8E);
    set_gate(27, (_isr27 as usize), 0x08, 0x8E);
    set_gate(28, (_isr28 as usize), 0x08, 0x8E);
    set_gate(29, (_isr29 as usize), 0x08, 0x8E);
    set_gate(30, (_isr30 as usize), 0x08, 0x8E);
    set_gate(31, (_isr31 as usize), 0x08, 0x8E);
    set_gate(32, (_irq0 as usize), 0x08, 0x8E); 
    set_gate(33, (_irq1 as usize), 0x08, 0x8E);
    set_gate(34, (_irq2 as usize), 0x08, 0x8E);
    set_gate(35, (_irq3 as usize), 0x08, 0x8E);
    set_gate(36, (_irq4 as usize), 0x08, 0x8E);
    set_gate(37, (_irq5 as usize), 0x08, 0x8E);
    set_gate(38, (_irq6 as usize), 0x08, 0x8E);
    set_gate(39, (_irq7 as usize), 0x08, 0x8E);
    set_gate(40, (_irq8 as usize), 0x08, 0x8E);
    set_gate(41, (_irq9 as usize), 0x08, 0x8E);
    set_gate(42, (_irq10 as usize), 0x08, 0x8E); 
    set_gate(43, (_irq11 as usize), 0x08, 0x8E);
    set_gate(44, (_irq12 as usize), 0x08, 0x8E);
    set_gate(45, (_irq13 as usize), 0x08, 0x8E);
    set_gate(46, (_irq14 as usize), 0x08, 0x8E);
    set_gate(47, (_irq15 as usize), 0x08, 0x8E);
}

extern "C" {
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
