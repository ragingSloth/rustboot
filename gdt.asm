global _gdt_flush
global _gdtr

_gdt_flush:
    lgdt [_gdtr]
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    jmp 0x08:flush2
flush2:
    ret

_gdtr:
    dw 0
    dd 0
