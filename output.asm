.text:
	align 16
	global _main

_main:
	push rax
	lea rax, [rel LA.0]
	mov rbx, rax
	push rax
	lea rax, [rel LA.2]
	mov rbx, rax
	push rax
	lea rax, [rel LA.4]
	pop rax
	call rax
	pop rax
	call rax


	mov rbx, church_decode
	call rax
	mov rbx, 0
	call rax

	mov rdi, rax
	mov rax, 0x2000001
	syscall

LA.1:
	mov rdx, rbx	; Bind variable
	mov rax, rdx	; load variable `x`
	mov rbx, rax
	mov rax, rcx	; load variable `f`
	pop rax
	call rax
	mov rbx, rax
	mov rax, rcx	; load variable `f`
	pop rax
	call rax
	ret

LA.0:
	mov rcx, rbx	; Bind variable
	push rax
	lea rax, [rel LA.1]
	ret

LA.3:
	mov rsi, rbx	; Bind variable
	mov rax, rsi	; load variable `x`
	mov rbx, rax
	mov rax, rcx	; load variable `f`
	pop rax
	call rax
	mov rbx, rax
	mov rax, rcx	; load variable `f`
	pop rax
	call rax
	mov rbx, rax
	mov rax, rcx	; load variable `f`
	pop rax
	call rax
	ret

LA.2:
	mov rcx, rbx	; Bind variable
	push rax
	lea rax, [rel LA.3]
	ret

LA.6:
	mov rcx, rbx	; Bind variable
	mov rax, rcx	; load variable `f`
	mov rbx, rax
	mov rax, rdi	; load variable `n`
	pop rax
	call rax
	mov rbx, rax
	mov rax, rsi	; load variable `m`
	pop rax
	call rax
	ret

LA.5:
	mov rdi, rbx	; Bind variable
	push rax
	lea rax, [rel LA.6]
	ret

LA.4:
	mov rsi, rbx	; Bind variable
	push rax
	lea rax, [rel LA.5]
	ret



church_decode:
    mov rax, rbx
	add rax, 1
	ret

