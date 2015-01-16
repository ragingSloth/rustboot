SHELL:=/usr/bin/env bash
LD=ld
RUSTC=rustc
NASM=nasm
QEMU=qemu-system-i386
RUSTCFLAGS= -L . -O --target i686-unknown-linux-gnu --crate-type lib -C relocation-model=static

.SUFFIXES: .o .rs .asm
.PHONY: clean run

all: kernel.bin

.asm.o:
	$(NASM) -f aout -o $@ $<

libcore.rlib: 
	cp libcore.rs libcore/lib.rs
	$(RUSTC) $(RUSTCFLAGS) libcore/lib.rs
	mv libcore/lib.rs ./

main.o: main.rs libcore.rlib
	$(RUSTC) $(RUSTCFLAGS) -o $@ --emit obj $<

kernel.bin: start.o main.o
	$(LD) -melf_i386 -Tlink.ld -o $@ $^


clean:
	rm -rf *.o *.iso kernel.bin *.rlib

run: kernel.bin
	$(QEMU) -kernel $<

cd: kernel.bin
	./build_image.sh
