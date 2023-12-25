section .data
    minus_symbol db "-"
    new_line db 0xA, 0xD

    x dd 5, 3, 2, 6, 1, 7, 4
    xlength equ ($-x) / 4

    y dd 0, 10, 1, 9, 2, 8, 5
    ylength equ ($-y) / 4

section .bss
    array_averange resd 1


section .text
    global _start


_start:
    mov rcx, xlength
    call sub_array

    mov rcx, xlength
    call calc_average

    jmp     exit


sub_array: 
    xor rbx, rbx

    .sub_loop:
        ; Вычисляем разность и записываем ее в 1 массив
        mov ah, [y+rbx*4]
        sub [x+rbx*4], ah

        inc rbx

        loop .sub_loop

    ret

calc_average:
    xor rbx, rbx

    .sum_loop:
        mov dx, [x+rbx*4]
        add [x+rbx*4+4], dx

        inc rbx

        loop .sum_loop

    ; Перемещаем итоговое значение
    xor rax, rax
    xor rdx, rdx
    movsx rax, byte [x+rbx*4]
    test rax, rax  ; Установим Signed Flag

    js .handle_negative

    .handle_positive:
        div rbx
        mov [array_averange], rax
        jmp .finish

    .handle_negative: 
        neg rax

        div rbx
        mov [array_averange], rax

        ; Выводим символ минуса
        mov rsi, minus_symbol
        call print_symbol

        jmp .finish
    
    .finish:
        ; Выводим модуль среднего значения 
        mov rsi, array_averange 
        add [rsi], byte '0'  ; Переводим число в ASCII символ
        call print_symbol

        ; Переходим на новую строку
        mov rsi, new_line
        call print_symbol
        
        ret


print_symbol: 
    ; input:    rsi - указатель на символ
    mov rax, 1  ; sys_write
    mov rdi, 1  ; stdout
    mov rdx, 1  ; bytes
    syscall
    ret

exit: 
    ; Завершение работы
    mov     rax,    60
    xor     rdi,    rdi
    syscall


; print_array:
;     ; input rsi - указатель на начало массива
;     ;       rcx - длина массива

;     ; Устанавливаем функцию вывода
;     mov     rax,    1   ; sys_write
;     mov     rdi,    1   ; stdout
;     mov     rdx,    1   ; Количество байт для вывода

;     .print_loop:
;         push rcx
;         call print_number
;         pop rcx

;         add rsi, 4         ; Переходим на следующий символ
        
;         loop .print_loop

;     ret

; print_number: 
;     add [rsi], byte '0'  ; Переводим число в ASCII символ
;     syscall

;     ret
