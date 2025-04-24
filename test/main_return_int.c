/*
	.section	__TEXT,__text,regular,pure_instructions
	; __TEXT段名, __text子段名, regular表明是常规段, pure_instructions表明该段只有指令不含数据
	.build_version macos, 15, 0	sdk_version 15, 2
	; 指定程序支持的操作系统版本和SDK版本
	.globl	_main                           ; -- Begin function main
	; 声明全局符号
	.p2align	2
	; 将下一条指令的地址按2的幂次方对齐（2表示按4字节对齐），下一条指令地址是4的倍数
_main:                                  ; @main
	.cfi_startproc
	; DWARF（Debugging With Attributed Record Fromats)调试信息相关指令，标记函数开始，此指令能让汇编器正确识别函数的起始点
; %bb.0:
	sub	sp, sp, #16
	; 为当前函数在栈上分配16字节空间
	.cfi_def_cfa_offset 16
	; DWRAF信息，定义当前帧指针（Canonical Frame Address,CFA）偏移量。这里表明当前函数栈帧大小是16字节
	str	wzr, [sp, #12]
	; wzr是零寄存器，其值始终为0。此指令将wzr的值（0）存储到sp+12的位置
	mov	w0, #42                         ; =0x2a
	; 将立即数42移动到w0寄存器，意味着返回值是42
	add	sp, sp, #16
	; 释放申请的16字节栈空间
	ret
	; 返回指令，程序会跳转到调用该函数的下一条指令继续执行
	.cfi_endproc
	; DWARF调试指令，标记函数结束
                                        ; -- End function
.subsections_via_symbols
; Macho-O中的一个特殊伪指令（assembler directive)，主要用于控制符号和段之间的关联方式
; 传统上，符号的地址直接对应某个字段的具体位置。但是某些情况下（如动态链接、符号重定位），需要灵活的管理符号与字段的映射关系。
; .subsections_via_symbols的作用:
; 告诉汇编器：符号的地址解析通过符号表(symbol table)中的信息来确定，而不是直接关联某个固定的字段.
*/
int main() {
    return 42;
}