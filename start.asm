[BITS 32]
%include "gdt.asm"
%include "isr.asm"
global start
start:
    mov esp, _sys_stack
    jmp stub

ALIGN 4
mboot:
    MB_PAGE_ALIGN    equ 1<<0
    MB_MEMORY_INFO   equ 1<<1
    MB_AOUT_KLUDGE   equ 1<<16
    MB_HEADER_MAGIC  equ 0x1BADB002
    MB_HEADER_FLAGS  equ MB_PAGE_ALIGN | MB_MEMORY_INFO | MB_AOUT_KLUDGE
    MB_CHECKSUM  equ -(MB_HEADER_MAGIC + MB_HEADER_FLAGS)
    EXTERN code, bss, end

    dd MB_HEADER_MAGIC
    dd MB_HEADER_FLAGS
    dd MB_CHECKSUM

    dd mboot
    dd code
    dd bss
    dd end
    dd start
stub:
    extern main
    call load_gdt
    call main
    jmp $


SECTION .bss
    resb 8192
_sys_stack:
