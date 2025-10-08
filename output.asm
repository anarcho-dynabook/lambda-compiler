.text:
	align 16
	global _main

_main:
	lea rax, [rel LA.0]


	mov rbx, church_decode
	mov r15, 0
	call rax

	mov rdi, r15
	mov rax, 0x2000001
	syscall

LA.1:
	mov rdx, rbx
	mov rax, rdx
	mov rbx, rax
	mov rax, rcx
	call rax
	mov rbx, rax
	mov rax, rcx
	call rax
	ret

LA.0:
	mov rcx, rbx
	lea rax, [rel LA.1]
	ret



church_decode:
	add r15, 1
	ret

