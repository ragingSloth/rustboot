global _isr0
_isr0:
	cli
	push byte 0
	push byte 0
	jmp isr_stub

global _isr1
_isr1:
	cli
	push byte 0
	push byte 1
	jmp isr_stub

global _isr2
_isr2:
	cli
	push byte 0
	push byte 2
	jmp isr_stub

global _isr3
_isr3:
	cli
	push byte 0
	push byte 3
	jmp isr_stub

global _isr4
_isr4:
	cli
	push byte 0
	push byte 4
	jmp isr_stub

global _isr5
_isr5:
	cli
	push byte 0
	push byte 5
	jmp isr_stub

global _isr6
_isr6:
	cli
	push byte 0
	push byte 6
	jmp isr_stub

global _isr7
_isr7:
	cli
	push byte 0
	push byte 7
	jmp isr_stub

global _isr8
_isr8:
	cli
	push byte 8
	jmp isr_stub

global _isr9
_isr9:
	cli
	push byte 0
	push byte 9
	jmp isr_stub

global _isr10
_isr10:
	cli
	push byte 10
	jmp isr_stub

global _isr11
_isr11:
	cli
	push byte 11
	jmp isr_stub

global _isr12
_isr12:
	cli
	push byte 12
	jmp isr_stub

global _isr13
_isr13:
	cli
	push byte 13
	jmp isr_stub

global _isr14
_isr14:
	cli
	push byte 14
	jmp isr_stub

global _isr15
_isr15:
	cli
	push byte 0
	push byte 15
	jmp isr_stub

global _isr16
_isr16:
	cli
	push byte 0
	push byte 16
	jmp isr_stub

global _isr17
_isr17:
	cli
	push byte 0
	push byte 17
	jmp isr_stub

global _isr18
_isr18:
	cli
	push byte 0
	push byte 18
	jmp isr_stub

global _isr19
_isr19:
	cli
	push byte 0
	push byte 19
	jmp isr_stub

global _isr20
_isr20:
	cli
	push byte 0
	push byte 20
	jmp isr_stub

global _isr21
_isr21:
	cli
	push byte 0
	push byte 21
	jmp isr_stub

global _isr22
_isr22:
	cli
	push byte 0
	push byte 22
	jmp isr_stub

global _isr23
_isr23:
	cli
	push byte 0
	push byte 23
	jmp isr_stub

global _isr24
_isr24:
	cli
	push byte 0
	push byte 24
	jmp isr_stub

global _isr25
_isr25:
	cli
	push byte 0
	push byte 25
	jmp isr_stub

global _isr26
_isr26:
	cli
	push byte 0
	push byte 26
	jmp isr_stub

global _isr27
_isr27:
	cli
	push byte 0
	push byte 27
	jmp isr_stub

global _isr28
_isr28:
	cli
	push byte 0
	push byte 28
	jmp isr_stub

global _isr29
_isr29:
	cli
	push byte 0
	push byte 29
	jmp isr_stub

global _isr30
_isr30:
	cli
	push byte 0
	push byte 30
	jmp isr_stub

global _isr31
_isr31:
	cli
	push byte 0
	push byte 31
	jmp isr_stub

isr_stub:
    pusha
    push ds
    push es
    push fs
    push gs

    mov ax 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mox eax, esp
    push eax
    
    extern _fault_handler
    mov eax, _fault_handler
    call eax

    pop eax
    pop gs
    pop fs
    pop es
    pop ds
    popa
    add esp, 8
    iret


;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
;;IRQs
;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;
global _irq0
_irq0:
	cli
	push byte 0
	push byte 0
	jmp irq_stub

global _irq1
_irq1:
	cli
	push byte 0
	push byte 1
	jmp irq_stub

global _irq2
_irq2:
	cli
	push byte 0
	push byte 2
	jmp irq_stub

global _irq3
_irq3:
	cli
	push byte 0
	push byte 3
	jmp irq_stub

global _irq4
_irq4:
	cli
	push byte 0
	push byte 4
	jmp irq_stub

global _irq5
_irq5:
	cli
	push byte 0
	push byte 5
	jmp irq_stub

global _irq6
_irq6:
	cli
	push byte 0
	push byte 6
	jmp irq_stub

global _irq7
_irq7:
	cli
	push byte 0
	push byte 7
	jmp irq_stub

global _irq8
_irq8:
	cli
	push byte 0
	push byte 8
	jmp irq_stub

global _irq9
_irq9:
	cli
	push byte 0
	push byte 9
	jmp irq_stub

global _irq10
_irq10:
	cli
	push byte 0
	push byte 10
	jmp irq_stub

global _irq11
_irq11:
	cli
	push byte 0
	push byte 11
	jmp irq_stub

global _irq12
_irq12:
	cli
	push byte 0
	push byte 12
	jmp irq_stub

global _irq13
_irq13:
	cli
	push byte 0
	push byte 13
	jmp irq_stub

global _irq14
_irq14:
	cli
	push byte 0
	push byte 14
	jmp irq_stub

irq_stub:
    pusha
    push ds
    push es
    push fs
    push gs
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov eax, esp
    push eax

    extern _irq_handler
    mov eax, _irq_handler
    call eax

    pop eax
    pop gs
    pop fs
    pop es
    pop ds
    popa
    add esp, 8
    iret
