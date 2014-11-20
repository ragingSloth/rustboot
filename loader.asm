use16

org 0x7c00

clear_registers:
    xor ax, ax
    mov bx, ax
    mov ds, ax
    mov es, ax
    mov ss, ax
   
    mov ah, 2 ; read
    mov al, 24 ; 24 sectors (12 KiB)
    mov ch, 0 ; cylinder & 0xff
    mov cl, 2 ; sector | ((cylinder >> 2) & 0xc0)
    mov dh, 0 ; head
    mov bx, 0x7e00
    int 0x13
    xor ax, ax

    cli
    lgdt [gdtr]
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    jmp 0x08:protected

protected:
    use32
    mov esp, 0x7bfc
    call 0x7e00
    jmp $

gdtr:
    dw (gdt_end - gdt) + 1
    dd gdt
gdt:
    dq 0 ;now I can segfault
    ;code segment
    dw 0xffff;max addressable soace b0-16
    dw 0x0000;base unknown
    db 0x00;more base
    db 0b10011010
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
    db 0x00 
    ;data segment
    dw 0xffff
    dw 0x0000
    db 0x00
    db 0b10010010
    db 0b01001111
    db 0x00 
gdt_end:

times 510-($-$$) db 0
dw 0xaa55
