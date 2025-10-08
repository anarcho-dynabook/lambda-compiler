.text:
	align 16
	global _main

; Source
; (\f.\x.f(f(fx)))


_main:
	mov rax, LA.0

	lea rbx, [rel church_decode]
	call stackframe
	mov rbx, 0
	call stackframe

	mov rdi, rax
	mov rax, 0x2000001
	syscall

LA.1:
	; Environment: "f: rcx, x: rdx"
	mov rdx, rbx	; Bind variable `x`
	mov rax, rdx	; load variable `x`
	mov rbx, rax
	mov rax, rcx	; load variable `f`
	call stackframe
	mov rbx, rax
	mov rax, rcx	; load variable `f`
	call stackframe
	mov rbx, rax
	mov rax, rcx	; load variable `f`
	call stackframe
	ret

LA.0:
	; Environment: "f: rcx"
	mov rcx, rbx	; Bind variable `f`
	mov rax, LA.1
	ret



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

