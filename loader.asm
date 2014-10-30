use16

org 0x7c00

loadSegments:
    xor ax, ax
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    lgdt gdtr

gdtr:
    dw (gdt_end - gdt) + 1
    db gdt
gdt:
    dq 0 ;now I can segfault
    ;code segment
    dw 0xFFFF;max addressable soace b0-16
    dw 0x0000;base unknown
    db 0x00;more base
    db 0b10011010;access byte 
        ;present
        ;2bit ring 0
        ;1
        ;executable = true
        ;grow up, eg a +1 = 0xe ->oxf
        ;readable
        ;cpu sets this
    db 0b01001111;limit/settings byte
        ;limit units 0 for bytes which matches 0xfffff
        ;size of protected, 1 for 32 bit
        ;2 0s
        ;0xf gets tacked onto end of limit
    dw 0x00 ; end of base
    ;data segment
    dw 0xFFFF
    dw 0x0000
    db 0x00
    db 0b10010010
    db 0b01001111
    dw 0x00 
