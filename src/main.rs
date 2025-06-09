#![no_std]
#![no_main]
#![warn(unsafe_op_in_unsafe_fn)]
//#![feature(naked_functions, asm_experimental_arch)]

mod sbi;
mod console;
mod common;

use core::arch::asm;
//use core::arch::naked_asm;
use core::ptr;
use core::panic::PanicInfo;

unsafe extern "C" {
    static mut __bss: u8;
    static mut __bss_end: u8;
    static __stack_top: u8;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(buf: *mut u8, value: u8, len: usize) -> *mut u8 {
    let mut p = buf;
    for _ in 0..len {
        unsafe { ptr::write(p, value); }
        unsafe { p = p.add(1); }
    }
    buf
}


#[repr(C, packed)]
pub struct TrapFrame {
    pub ra: u32,
    pub gp: u32,
    pub tp: u32,
    pub t0: u32,
    pub t1: u32,
    pub t2: u32,
    pub t3: u32,
    pub t4: u32,
    pub t5: u32,
    pub t6: u32,
    pub a0: u32,
    pub a1: u32,
    pub a2: u32,
    pub a3: u32,
    pub a4: u32,
    pub a5: u32,
    pub a6: u32,
    pub a7: u32,
    pub s0: u32,
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
    pub s5: u32,
    pub s6: u32,
    pub s7: u32,
    pub s8: u32,
    pub s9: u32,
    pub s10: u32,
    pub s11: u32,
    pub sp: u32,
}

//#[naked]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text")]
pub unsafe extern "C" fn kernel_entry() {
    asm!(
        "csrw sscratch, sp",
        "addi sp, sp, -4*31",
        // save registers
        "sw ra,  4*0(sp)",
        "sw gp,  4*1(sp)",
        "sw tp,  4*2(sp)",
        "sw t0,  4*3(sp)",
        "sw t1,  4*4(sp)",
        "sw t2,  4*5(sp)",
        "sw t3,  4*6(sp)",
        "sw t4,  4*7(sp)",
        "sw t5,  4*8(sp)",
        "sw t6,  4*9(sp)",
        "sw a0,  4*10(sp)",
        "sw a1,  4*11(sp)",
        "sw a2,  4*12(sp)",
        "sw a3,  4*13(sp)",
        "sw a4,  4*14(sp)",
        "sw a5,  4*15(sp)",
        "sw a6,  4*16(sp)",
        "sw a7,  4*17(sp)",
        "sw s0,  4*18(sp)",
        "sw s1,  4*19(sp)",
        "sw s2,  4*20(sp)",
        "sw s3,  4*21(sp)",
        "sw s4,  4*22(sp)",
        "sw s5,  4*23(sp)",
        "sw s6,  4*24(sp)",
        "sw s7,  4*25(sp)",
        "sw s8,  4*26(sp)",
        "sw s9,  4*27(sp)",
        "sw s10, 4*28(sp)",
        "sw s11, 4*29(sp)",

        "csrr a0, sscratch",
        "sw a0, 4*30(sp)",

        "mv a0, sp",
        "call handle_trap",

        // restore registers
        "lw ra,  4*0(sp)",
        "lw gp,  4*1(sp)",
        "lw tp,  4*2(sp)",
        "lw t0,  4*3(sp)",
        "lw t1,  4*4(sp)",
        "lw t2,  4*5(sp)",
        "lw t3,  4*6(sp)",
        "lw t4,  4*7(sp)",
        "lw t5,  4*8(sp)",
        "lw t6,  4*9(sp)",
        "lw a0,  4*10(sp)",
        "lw a1,  4*11(sp)",
        "lw a2,  4*12(sp)",
        "lw a3,  4*13(sp)",
        "lw a4,  4*14(sp)",
        "lw a5,  4*15(sp)",
        "lw a6,  4*16(sp)",
        "lw a7,  4*17(sp)",
        "lw s0,  4*18(sp)",
        "lw s1,  4*19(sp)",
        "lw s2,  4*20(sp)",
        "lw s3,  4*21(sp)",
        "lw s4,  4*22(sp)",
        "lw s5,  4*23(sp)",
        "lw s6,  4*24(sp)",
        "lw s7,  4*25(sp)",
        "lw s8,  4*26(sp)",
        "lw s9,  4*27(sp)",
        "lw s10, 4*28(sp)",
        "lw s11, 4*29(sp)",
        "lw sp,  4*30(sp)",

        "sret",
        options(noreturn)
    );
}


#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_main() -> ! { unsafe {
    let bss_start = &raw mut __bss as *mut u8;
    let bss_end = &raw mut __bss_end as *mut u8;
    let bss_size =  bss_end.offset_from(bss_start) as usize;

    memset(bss_start, 0, bss_size);

		let _use_putchar = false;

		if _use_putchar {
			let msg = b"\n\nHello World!\n";
			for &ch in msg {
				console::_putchar(ch);
			}
		}

		printf!("\nHello World from Rust!\n");
		printf!("String: {}, Decimal: {}, Hex: {:#x}\n", "test", -42, 3735928559u32);

    PANIC!("booted!");
    //printf!("unreachable here!");

		/*
    loop {
        asm!("wfi", options(nomem, nostack));
    }
		*/
}}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
pub unsafe extern "C" fn boot() -> ! {
    unsafe {asm!(
        "la sp, {stack_top}",
        "j {main}",
        stack_top = sym __stack_top,
        main = sym kernel_main,
        options(noreturn)
    ); }
}

