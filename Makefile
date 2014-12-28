LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386

.SUFFIXES: .o .rs .asm
.PHONY: clean

.asm.o:
	$(NASM) -f aout -o $@ $<

.rs.o:
	$(RUSTC) -O --target i686-unknown-linux-gnu --crate-type lib -o $@ --emit obj $<

kernel.bin: start.o main.o
	$(LD) -m i386linux -T link.ld -o $@ $<

all: kernel.bin

clean:
	rm -rf *.o kernel.bin
