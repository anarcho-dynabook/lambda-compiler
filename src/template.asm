.text:
	align 16
	global _main

_main:
$main

	mov rbx, church_decode
	call rax
	mov rbx, 0
	call rax

	mov rdi, rax
	mov rax, 0x2000001
	syscall

$code

church_decode:
    mov rax, rbx
	add rax, 1
	ret
