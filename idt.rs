use utils;

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

#[no_mangle]
static mut idt: [IDT, ..256] = [IDT {base1: 0, selector: 0, zero: 0, attrs: 0, base2: 0}, ..256];

#[no_stack_check]
#[no_mangle]
pub extern "C" fn load_idt() {

}

#[no_stack_check]
fn set_descriptor(desc: &mut IDT, offset: u32, ring: u8, gate_type: GATE_TYPE, selector: u16){
    desc.base1 = 0xFFFFu16 & offset;
    desc.base2 = 0xFFFFu16 & offset >> 16;
    desc.selector = selector;
    desc.zero = 0;
    attrs = 0x80 | ring << 5 | 0x10 | gate_type as u8;
}

enum GATE_TYPE {
    task   = 0x5,
    int16  = 0x6,
    trap16 = 0x7,
    int32  = 0xE,
    trap32 = 0xF,
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
}
