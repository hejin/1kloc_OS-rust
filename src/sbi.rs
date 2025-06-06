#![allow(dead_code)]

#[repr(C)]
pub struct Sbiret {
    pub error: isize,
    pub value: isize,
}

#[inline(always)]
pub fn sbi_call(
    arg0: isize,
    arg1: isize,
    arg2: isize,
    arg3: isize,
    arg4: isize,
    arg5: isize,
    fid: isize,
    eid: isize,
) -> Sbiret {
    let mut a0 = arg0;
    let mut a1 = arg1;

    unsafe {
        core::arch::asm!(
            "ecall",
            inout("a0") a0,
            inout("a1") a1,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            options(nostack, preserves_flags)
        );
    }

    Sbiret { error: a0, value: a1 }
}

