/*
Automatically generated file. Do not edit.
This file contains assembly from musl-1.2.5.tar.gz:
----------------------------------------------------------------------
Copyright © 2005-2020 Rich Felker, et al.

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
----------------------------------------------------------------------
*/

#![no_std]

pub use start_linux_attr::start_linux;

#[macro_export]
macro_rules! wrap_start {
    ($start:ident) => {

#[cfg(target_arch = "aarch64")]
core::arch::global_asm!(r#"
.text
.global _start
.type _start,%function
_start:
	mov x29, #0
	mov x30, #0
	mov x0, sp
.weak _DYNAMIC
.hidden _DYNAMIC
	adrp x1, _DYNAMIC
	add x1, x1, #:lo12:_DYNAMIC
	and sp, x0, #-16
	b {start}
"#, options(), start = sym $start);

#[cfg(target_arch = "arm")]
core::arch::global_asm!(r#"
.text
.global _start
.type _start,%function
_start:
	mov fp, #0
	mov lr, #0
	ldr a2, 1f
	add a2, pc, a2
	mov a1, sp
2:	and ip, a1, #-16
	mov sp, ip
	bl {start}
.weak _DYNAMIC
.hidden _DYNAMIC
.align 2
1:	.word _DYNAMIC-2b
"#, options(), start = sym $start);

#[cfg(target_arch = "x86")]
core::arch::global_asm!(r#"
.text
.weak _DYNAMIC
.hidden _DYNAMIC
.global _start
_start:
	xor %ebp,%ebp
	mov %esp,%eax
	and $-16,%esp
	push %eax
	push %eax
	call 1f
1:	addl $_DYNAMIC-1b,(%esp)
	push %eax
	call {start}
"#, options(att_syntax), start = sym $start);

#[cfg(target_arch = "loongarch64")]
core::arch::global_asm!(r#"
.text
.global _start
.type   _start, @function
_start:
	move $fp, $zero
	move $a0, $sp
.weak _DYNAMIC
.hidden _DYNAMIC
	la.local $a1, _DYNAMIC
	bstrins.d $sp, $zero, 3, 0
	b {start}
"#, options(), start = sym $start);

#[cfg(target_arch = "m68k")]
core::arch::global_asm!(r#"
.text
.weak _DYNAMIC
.hidden _DYNAMIC
.global _start
_start:
	suba.l %fp,%fp
	movea.l %sp,%a0
	lea _DYNAMIC-.-8,%a1
	pea (%pc,%a1)
	pea (%a0)
	lea {start}-.-8,%a1
	jsr (%pc,%a1)
"#, options(), start = sym $start);

#[cfg(target_arch = "mips64")]
core::arch::global_asm!(r#"
.set push
.set noreorder
.text
.global __start
.global _start
.global _start_data
.type   __start, @function
.type   _start, @function
.type   _start_data, @function
__start:
_start:
.align 8
	bal 1f
	 move $fp, $0
_start_data:
	.gpdword _start_data
	.gpdword {start}
.weak _DYNAMIC
.hidden _DYNAMIC
	.gpdword _DYNAMIC
1:	ld $gp, 0($ra)
	dsubu $gp, $ra, $gp
	move $4, $sp
	ld $5, 16($ra)
	daddu $5, $5, $gp
	ld $25, 8($ra)
	daddu $25, $25, $gp
	and $sp, $sp, -16
	jalr $25
	nop
.set pop
"#, options(), start = sym $start);

#[cfg(target_arch = "mips")]
core::arch::global_asm!(r#"
.set push
.set noreorder
.text
.global __start
.global _start
.type   __start, @function
.type   _start, @function
__start:
_start:
	bal 1f
	 move $fp, $0
	.gpword .
	.gpword {start}
.weak _DYNAMIC
.hidden _DYNAMIC
	.gpword _DYNAMIC
1:	lw $gp, 0($ra)
	subu $gp, $ra, $gp
	move $4, $sp
	lw $5, 8($ra)
	addu $5, $5, $gp
	lw $25, 4($ra)
	addu $25, $25, $gp
	and $sp, $sp, -8
	jalr $25
	 subu $sp, $sp, 16
.set pop
"#, options(), start = sym $start);

#[cfg(target_arch = "powerpc64")]
core::arch::global_asm!(r#"
.text
.global _start
.type   _start, %function
_start:
	addis  2, 12, .TOC.-_start@ha
	addi   2,  2, .TOC.-_start@l
	lwz    4, 1f-_start(12)
	add    4, 4, 12
	mr     3, 1
	clrrdi 1, 1, 4
	li     0, 0
	stdu   0, -32(1)
	mtlr   0
	bl {start}
.weak   _DYNAMIC
.hidden _DYNAMIC
1:	.long _DYNAMIC-_start
"#, options(), start = sym $start);

#[cfg(target_arch = "powerpc")]
core::arch::global_asm!(r#"
.text
.global _start
.type   _start, %function
_start:
	bl 1f
.weak _DYNAMIC
.hidden _DYNAMIC
	.long _DYNAMIC-.
1:	mflr 4
	lwz 3, 0(4)
	add 4, 3, 4
	mr 3, 1
	clrrwi 1, 1, 4
	li 0, 0
	stwu 1, -16(1)
	mtlr 0
	stw 0, 0(1)
	bl {start}
"#, options(), start = sym $start);

#[cfg(target_arch = "riscv32")]
core::arch::global_asm!(r#"
.section .sdata,"aw"
.text
.global _start
.type _start,%function
_start:
.weak __global_pointer$
.hidden __global_pointer$
.option push
.option norelax
	lla gp, __global_pointer$
.option pop
	mv a0, sp
.weak _DYNAMIC
.hidden _DYNAMIC
	lla a1, _DYNAMIC
	andi sp, sp, -16
	tail {start}
"#, options(), start = sym $start);

#[cfg(target_arch = "riscv64")]
core::arch::global_asm!(r#"
.section .sdata,"aw"
.text
.global _start
.type _start,%function
_start:
.weak __global_pointer$
.hidden __global_pointer$
.option push
.option norelax
	lla gp, __global_pointer$
.option pop
	mv a0, sp
.weak _DYNAMIC
.hidden _DYNAMIC
	lla a1, _DYNAMIC
	andi sp, sp, -16
	tail {start}
"#, options(), start = sym $start);

#[cfg(target_arch = "s390x")]
core::arch::global_asm!(r#"
.text
.global _start
.type   _start, %function
_start:
	lgr  %r2, %r15
	larl %r3, 1f
	agf  %r3, 0(%r3)
	aghi %r15, -160
	lghi %r0, 0
	stg  %r0, 0(%r15)
	jg {start}
	.align 8
.weak   _DYNAMIC
.hidden _DYNAMIC
1:	.long _DYNAMIC-.
"#, options(), start = sym $start);

#[cfg(target_arch = "x86_64")]
core::arch::global_asm!(r#"
.text
.global _start
_start:
	xor %rbp,%rbp
	mov %rsp,%rdi
.weak _DYNAMIC
.hidden _DYNAMIC
	lea _DYNAMIC(%rip),%rsi
	andq $-16,%rsp
	call {start}
"#, options(att_syntax), start = sym $start);
    }
}
