use utils::outb;
use isr::Regs;
use io::Cell;

static mut ticks: usize = 0;

fn timer(hz: u16) {
    let divisor = 1193180 / hz;
    outb(0x43, 0x36);
    outb(0x40, (divisor & (0x00FF as u16)) as u8);
    outb(0x40, (divisor >> 8) as u8);
}

pub fn timer_handler(regs: Regs) {
    unsafe {
        ticks += 1;
        if ticks % 18 == 0 {
            let mut screen = Cell::new();
            screen.puts("One second\n");
        }
    }
}
