#![allow(dead_code)]

use core::ptr;

/// Basic aliases matching `common.h`
pub type BoolT = bool;
pub type Uint8T = u8;
pub type Uint16T = u16;
pub type Uint32T = u32;
pub type Uint64T = u64;
pub type SizeT = u32;
pub type PaddrT = u32;
pub type VaddrT = u32;

/// Constants
pub const TRUE: BoolT = true;
pub const FALSE: BoolT = false;
pub const NULL: *mut u8 = core::ptr::null_mut();

/// Align utilities
#[inline(always)]
pub const fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

#[inline(always)]
pub const fn is_aligned(value: usize, align: usize) -> bool {
    value & (align - 1) == 0
}

#[macro_export]
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe { &(*(core::ptr::null::<$ty>())).$field as *const _ as usize }
    };
}

/// Safe equivalents of libc-like routines

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: SizeT) -> *mut u8 { unsafe {
    for i in 0..n {
        ptr::write(dst.add(i as usize), ptr::read(src.add(i as usize)));
    }
    dst
}}

/*
#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(buf: *mut u8, c: i32, n: SizeT) -> *mut u8 {
    for i in 0..n {
        ptr::write(buf.add(i as usize), c as u8);
    }
    buf
}
*/

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcpy(dst: *mut u8, src: *const u8) -> *mut u8 { unsafe {
    let mut d = dst;
    let mut s = src;
    loop {
        let ch = ptr::read(s);
        ptr::write(d, ch);
        if ch == 0 {
            break;
        }
        s = s.add(1);
        d = d.add(1);
    }
    dst
}}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn strcmp(s1: *const u8, s2: *const u8) -> i32 { unsafe {
    let mut p1 = s1;
    let mut p2 = s2;

    loop {
        let c1 = ptr::read(p1);
        let c2 = ptr::read(p2);
        if c1 == 0 || c2 == 0 || c1 != c2 {
            return c1 as i32 - c2 as i32;
        }
        p1 = p1.add(1);
        p2 = p2.add(1);
    }
}}


