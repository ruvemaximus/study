section .data: 
    num db 169

section .bss
    x1 resb 1
    x2 resb 1

section .text
    global _start

_start:
    ; x1 = num / 2
    mov al, [num]
    mov bl, 2
    div bl  ; al - целая часть ah - остаток
    mov byte [x1], al

    call calc_x2

    .sqrt_loop:
        xor rax, rax
        xor rbx, rbx

        ; if x1 - x2 >= 1 then continue -> x1+1-x2 >= 0
        mov al, byte [x1]
        mov bl, byte [x2]
        sub al, bl
        cmp al, 1
        jl print_result

        ; x1 = x2
        mov bl, byte [x2]
        mov byte [x1], bl

        call calc_x2

        jmp .sqrt_loop

print_number: 
    mov rax, 1
    mov rdi, 1 
    mov rdx, 1
    add [rsi], byte '0'
    syscall
    ret

print_result: 
    .print_loop:
        mov al, byte [x2]
        mov bl, 10
        div bl
        mov byte [x2], al
        mov byte [x1], ah

        mov rsi, x1
        call print_number

        mov byte [x1], al
        cmp al, 1
        jnz .print_loop

    mov rsi, x2
    call print_number

    jmp exit


calc_x2: 
    ; x2 = (x1 + (num / x1)) // 2
    mov al, [num]
    mov bl, byte [x1]
    div bl 
    mov byte [x2], al
    add byte [x2], bl
    shr byte [x2], 1
    ret 

exit:
    ; Завершение работы 
    mov rax,    60
    xor rdi,    rdi
    syscall