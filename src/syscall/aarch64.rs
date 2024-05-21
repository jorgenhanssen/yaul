use core::arch::asm;

pub unsafe fn syscall0(n: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x8") n,
        lateout("x0") ret,
        options(nostack, preserve_flags)
    );
    ret
}

pub unsafe fn syscall1(n: usize, a1: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x8") n,
        in("x0") a1,
        lateout("x0") ret,
        options(nostack, preserve_flags)
    );
    ret
}

pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x8") n,
        in("x0") a1,
        in("x1") a2,
        lateout("x0") ret,
        options(nostack, preserve_flags)
    );
    ret
}

pub unsafe fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x8") n,
        in("x0") a1,
        in("x1") a2,
        in("x2") a3,
        lateout("x0") ret,
        options(nostack, preserve_flags)
    );
    ret
}

pub unsafe fn syscall4(n: usize, a1: usize, a2: usize, a3: usize, a4: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x8") n,
        in("x0") a1,
        in("x1") a2,
        in("x2") a3,
        in("x3") a4,
        lateout("x0") ret,
        options(nostack, preserve_flags)
    );
    ret
}

pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x8") n,
        in("x0") a1,
        in("x1") a2,
        in("x2") a3,
        in("x3") a4,
        in("x4") a5,
        lateout("x0") ret,
        options(nostack, preserve_flags)
    );
    ret
}

pub unsafe fn syscall6(
    n: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x8") n,
        in("x0") a1,
        in("x1") a2,
        in("x2") a3,
        in("x3") a4,
        in("x4") a5,
        in("x5") a6,
        lateout("x0") ret,
        options(nostack, preserve_flags)
    );
    ret
}
