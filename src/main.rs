#![no_std]
#![no_main]
#![warn(unsafe_op_in_unsafe_fn)]

use core::arch::asm;
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kernel_main() -> ! {
    let bss_start = &raw mut __bss as *mut u8;
    let bss_end = &raw mut __bss_end as *mut u8;
    let bss_size = unsafe { bss_end.offset_from(bss_start) as usize} ;

    unsafe { memset(bss_start, 0, bss_size); }

    loop {}
}

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

