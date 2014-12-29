LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386
RUSTCFLAGS= -O --target i686-unknown-linux-gnu --crate-type lib -o 

.SUFFIXES: .o .rs .asm
.PHONY: clean run

all: kernel.bin

.asm.o:
	$(NASM) -f aout -o $@ $<

main.o: main.rs utils.rs
	$(RUSTC) $(RUSTCFLAGS) $@ --emit obj $<

kernel.bin: start.o main.o
	$(LD) -melf_i386 -Tlink.ld -o $@ $^


clean:
	rm -rf *.o *.iso kernel.bin

run: kernel.bin
	$(QEMU) -kernel $<

cd: kernel.bin
	./build_image.sh
