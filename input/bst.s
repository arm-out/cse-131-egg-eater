section .text
global our_code_starts_here
extern snek_error
extern snek_print

throw_error:
    mov rdi, rbx
    push rsp
    and rsp, -16
    call snek_error
    ret

bst:
    sub rsp, 32

    mov rax, [rsp + 40]
    mov [rsp + 0], rax
    mov rax, 1
    mov [rsp + 8], rax
    mov rax, [rsp + 8]
    mov [r15 + 8], rax
    mov rax, 2
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 16
    mov [rsp + 8], rax
    mov rax, 1
    mov [rsp + 16], rax
    mov rax, [rsp + 16]
    mov [r15 + 8], rax
    mov rax, 2
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 16
    mov [rsp + 16], rax
    mov rax, [rsp + 16]
    mov [r15 + 24], rax
    mov rax, [rsp + 8]
    mov [r15 + 16], rax
    mov rax, [rsp + 0]
    mov [r15 + 8], rax
    mov rax, 6
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 32

    add rsp, 32
    ret

bst_add:
    sub rsp, 48

    mov rax, [rsp + 56]
    mov [rsp + 0], rax
    mov rax, 0
    mov rbx, [rsp + 0]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 0]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 0]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 0], rax
    mov rax, 1
    cmp [rsp + 0], rax
    mov rbx, 7
    mov rax, 3
    cmove rax, rbx
    cmp rax, 3
    je ifelse2
    mov rax, [rsp + 64]
    mov [rsp + 0], rax
    sub rsp, 16
    mov rbx, [rsp + 16]
    mov [rsp + 0], rbx
    mov [rsp + 8], rdi
    call bst
    mov rdi, [rsp + 8]
    add rsp, 16
    jmp ifend1
    ifelse2:
    mov rax, [rsp + 64]
    mov [rsp + 0], rax
    mov rax, [rsp + 56]
    mov [rsp + 8], rax
    mov rax, 0
    mov rbx, [rsp + 8]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 8]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 8]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov rbx, rax
    or rbx, [rsp + 0]
    and rbx, 1
    cmp rbx, 1
    mov rbx, 7
    je throw_error
    cmp [rsp + 0], rax
    mov rbx, 7
    mov rax, 3
    cmovl rax, rbx
    cmp rax, 3
    je ifelse4
    mov rax, [rsp + 56]
    mov [rsp + 0], rax
    mov rax, 0
    mov rbx, [rsp + 0]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 0]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 0]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 0], rax
    mov rax, [rsp + 56]
    mov [rsp + 8], rax
    mov rax, 2
    mov rbx, [rsp + 8]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 8]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 8]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 8], rax
    mov rax, [rsp + 64]
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call bst_add
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 8], rax
    mov rax, [rsp + 56]
    mov [rsp + 16], rax
    mov rax, 4
    mov rbx, [rsp + 16]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 16]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 16]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 16], rax
    mov rax, [rsp + 16]
    mov [r15 + 24], rax
    mov rax, [rsp + 8]
    mov [r15 + 16], rax
    mov rax, [rsp + 0]
    mov [r15 + 8], rax
    mov rax, 6
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 32
    jmp ifend3
    ifelse4:
    mov rax, [rsp + 56]
    mov [rsp + 0], rax
    mov rax, 0
    mov rbx, [rsp + 0]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 0]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 0]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 0], rax
    mov rax, [rsp + 56]
    mov [rsp + 8], rax
    mov rax, 2
    mov rbx, [rsp + 8]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 8]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 8]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 8], rax
    mov rax, [rsp + 56]
    mov [rsp + 16], rax
    mov rax, 4
    mov rbx, [rsp + 16]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 16]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 16]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 16], rax
    mov rax, [rsp + 64]
    mov [rsp + 24], rax
    sub rsp, 24
    mov rbx, [rsp + 40]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 48]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call bst_add
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 16], rax
    mov rax, [rsp + 16]
    mov [r15 + 24], rax
    mov rax, [rsp + 8]
    mov [r15 + 16], rax
    mov rax, [rsp + 0]
    mov [r15 + 8], rax
    mov rax, 6
    mov [r15 + 0], rax
    mov rax, r15
    add rax, 1
    add r15, 32
    ifend3:
    ifend1:

    add rsp, 48
    ret

contains:
    sub rsp, 24

    mov rax, [rsp + 32]
    mov [rsp + 0], rax
    mov rax, 0
    mov rbx, [rsp + 0]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 0]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 0]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 0], rax
    mov rax, 1
    cmp [rsp + 0], rax
    mov rbx, 7
    mov rax, 3
    cmove rax, rbx
    cmp rax, 3
    je ifelse6
    mov rax, 3
    jmp ifend5
    ifelse6:
    mov rax, [rsp + 32]
    mov [rsp + 0], rax
    mov rax, 0
    mov rbx, [rsp + 0]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 0]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 0]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 0], rax
    mov rax, [rsp + 40]
    cmp [rsp + 0], rax
    mov rbx, 7
    mov rax, 3
    cmove rax, rbx
    cmp rax, 3
    je ifelse8
    mov rax, 7
    jmp ifend7
    ifelse8:
    mov rax, [rsp + 40]
    mov [rsp + 0], rax
    mov rax, [rsp + 32]
    mov [rsp + 8], rax
    mov rax, 0
    mov rbx, [rsp + 8]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 8]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 8]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov rbx, rax
    or rbx, [rsp + 0]
    and rbx, 1
    cmp rbx, 1
    mov rbx, 7
    je throw_error
    cmp [rsp + 0], rax
    mov rbx, 7
    mov rax, 3
    cmovl rax, rbx
    cmp rax, 3
    je ifelse10
    mov rax, [rsp + 32]
    mov [rsp + 0], rax
    mov rax, 2
    mov rbx, [rsp + 0]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 0]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 0]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 0], rax
    mov rax, [rsp + 40]
    mov [rsp + 8], rax
    sub rsp, 24
    mov rbx, [rsp + 24]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 32]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call contains
    mov rdi, [rsp + 16]
    add rsp, 24
    jmp ifend9
    ifelse10:
    mov rax, [rsp + 32]
    mov [rsp + 0], rax
    mov rax, 4
    mov rbx, [rsp + 0]
    and rbx, 3
    cmp rbx, 1
    mov rbx, 7
    jne throw_error
    mov rcx, [rsp + 0]
    cmp rax, [rcx + 0]
    mov rbx, 9
    jge throw_error
    mov rbx, 0
    cmp rax, rbx
    mov rbx, 9
    jl throw_error
    sar rax, 1
    add rax, 1
    imul rax, 8
    mov rbx, [rsp + 0]
    sub rbx, 1
    mov rax, [rbx + rax]
    mov [rsp + 0], rax
    mov rax, [rsp + 40]
    mov [rsp + 8], rax
    sub rsp, 24
    mov rbx, [rsp + 24]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 32]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call contains
    mov rdi, [rsp + 16]
    add rsp, 24
    ifend9:
    ifend7:
    ifend5:

    add rsp, 24
    ret



our_code_starts_here:
    mov r15, rsi
    sub rsp, 40

    mov rax, 20
    mov [rsp + 0], rax
    sub rsp, 16
    mov rbx, [rsp + 16]
    mov [rsp + 0], rbx
    mov [rsp + 8], rdi
    call bst
    mov rdi, [rsp + 8]
    add rsp, 16
    sub rsp, 8
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 8
    mov rax, 20
    mov [rsp + 0], rax
    sub rsp, 16
    mov rbx, [rsp + 16]
    mov [rsp + 0], rbx
    mov [rsp + 8], rdi
    call bst
    mov rdi, [rsp + 8]
    add rsp, 16
    mov [rsp + 0], rax
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 10
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call bst_add
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 0], rax
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 40
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call bst_add
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 0], rax
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 30
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call bst_add
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 0], rax
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 14
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call bst_add
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 0], rax
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 6
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call bst_add
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 0], rax
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 60
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call bst_add
    mov rdi, [rsp + 16]
    add rsp, 24
    mov [rsp + 0], rax
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 10
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call contains
    mov rdi, [rsp + 16]
    add rsp, 24
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 24
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call contains
    mov rdi, [rsp + 16]
    add rsp, 24
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 26
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call contains
    mov rdi, [rsp + 16]
    add rsp, 24
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 30
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call contains
    mov rdi, [rsp + 16]
    add rsp, 24
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 40
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call contains
    mov rdi, [rsp + 16]
    add rsp, 24
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 6
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call contains
    mov rdi, [rsp + 16]
    add rsp, 24
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24
    mov rax, [rsp + 0]
    mov [rsp + 8], rax
    mov rax, 0
    mov [rsp + 16], rax
    sub rsp, 24
    mov rbx, [rsp + 32]
    mov [rsp + 0], rbx
    mov rbx, [rsp + 40]
    mov [rsp + 8], rbx
    mov [rsp + 16], rdi
    call contains
    mov rdi, [rsp + 16]
    add rsp, 24
    sub rsp, 24
    mov [rsp + 0], rdi
    mov rdi, rax
    call snek_print
    mov rdi, [rsp + 0]
    add rsp, 24

    add rsp, 40
    ret
