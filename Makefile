LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386

.SUFFIXES: .o .rs .asm
.PHONY: clean run

.asm.o:
	$(NASM) -f aout -o $@ $<

.rs.o:
	$(RUSTC) -O --target i686-unknown-linux-gnu --crate-type lib -o $@ --emit obj $<

kernel.bin: start.o main.o
	$(LD) -melf_i386 -Tlink.ld -o $@ $^

all: kernel.bin

clean:
	rm -rf *.o *.iso kernel.bin

run: kernel.bin
	$(QEMU) -kernel $<

cd: kernel.bin
	./build_image.sh

