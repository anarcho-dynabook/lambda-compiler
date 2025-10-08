.text:
	align 16
	global _main

_main:
$main

	mov rbx, church_decode
	mov r15, 0
	call rax

	mov rdi, r15
	mov rax, 0x2000001
	syscall

$code

church_decode:
	add r15, 1
	ret
