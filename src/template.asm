BITS 64
.text:
	align 16
	global _start

; Source
; $src

_start:
$main
	mov rbx, church_decode
	call stackframe
	mov rbx, 0
	call stackframe

	mov rdi, rax
	mov rax, 60
	syscall

$code

stackframe:
    push rbp
    mov rbp, rsp

    push rcx
    push rdx
    push rsi
    push rdi
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15

    call rax

    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rdi
    pop rsi
    pop rdx
    pop rcx

    mov rsp, rbp
    pop rbp
    ret

church_decode:
    mov rax, rbx
	add rax, 1
	ret
