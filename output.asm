.text:
	align 16
	global _main

; Source
; (\m.\n.\f.m (n f)) (\f.\x.f (f (f x))) (\f.\x.f (f x))

_main:
	lea rax, [rel LA.0]
	push rax
	lea rax, [rel LA.3]
	mov rbx, rax
	pop rax
	call stackframe
	push rax
	lea rax, [rel LA.5]
	mov rbx, rax
	pop rax
	call stackframe


	mov rbx, church_decode
	call rax
	mov rbx, 0
	call rax

	mov rdi, rax
	mov rax, 0x2000001
	syscall

LA.2:
	; Environment: "m: rcx, n: rdx, f: rsi"
	mov rsi, rbx	; Bind variable `f`
	mov rax, rcx	; load variable `m`
	push rax
	mov rax, rdx	; load variable `n`
	push rax
	mov rax, rsi	; load variable `f`
	mov rbx, rax
	pop rax
	call stackframe
	mov rbx, rax
	pop rax
	call stackframe
	ret

LA.1:
	; Environment: "m: rcx, n: rdx"
	mov rdx, rbx	; Bind variable `n`
	lea rax, [rel LA.2]
	ret

LA.0:
	; Environment: "m: rcx"
	mov rcx, rbx	; Bind variable `m`
	lea rax, [rel LA.1]
	ret

LA.4:
	; Environment: "m: rcx, f: rdx, x: rsi"
	mov rsi, rbx	; Bind variable `x`
	mov rax, rdx	; load variable `f`
	push rax
	mov rax, rdx	; load variable `f`
	push rax
	mov rax, rdx	; load variable `f`
	push rax
	mov rax, rsi	; load variable `x`
	mov rbx, rax
	pop rax
	call stackframe
	mov rbx, rax
	pop rax
	call stackframe
	mov rbx, rax
	pop rax
	call stackframe
	ret

LA.3:
	; Environment: "m: rcx, f: rdx"
	mov rdx, rbx	; Bind variable `f`
	lea rax, [rel LA.4]
	ret

LA.6:
	; Environment: "m: rcx, f: rdx, x: rsi"
	mov rsi, rbx	; Bind variable `x`
	mov rax, rdx	; load variable `f`
	push rax
	mov rax, rdx	; load variable `f`
	push rax
	mov rax, rsi	; load variable `x`
	mov rbx, rax
	pop rax
	call stackframe
	mov rbx, rax
	pop rax
	call stackframe
	ret

LA.5:
	; Environment: "m: rcx, f: rdx"
	mov rdx, rbx	; Bind variable `f`
	lea rax, [rel LA.6]
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

    leave
    ret

church_decode:
    mov rax, rbx
	add rax, 1
	ret

