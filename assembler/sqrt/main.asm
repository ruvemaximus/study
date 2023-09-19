section .data: 
    num db 9

section .bss
    x1 resb 1
    x2 resb 1

section .text
    global _start

_start:
    ; x1 = number / 2
    mov ax, [ num ]
    mov bx, 2
    div bx

    ; mov [x1], byte al

    ; Завершение работы 
    mov rax,    60
    xor rdi,    rdi
    syscall

div_without_remainder: 
    ; (ax: делимое) -> (ax: целая часть)
    shr ax, 1
    ret