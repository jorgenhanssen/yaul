use core::arch::asm;

pub unsafe fn syscall0(n: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x16") n,
        lateout("x0") ret,
        options(nostack)
    );
    ret
}

pub unsafe fn syscall1(n: usize, arg1: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x16") n,
        in("x0") arg1,
        lateout("x0") ret,
        options(nostack)
    );
    ret
}

pub unsafe fn syscall2(n: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x16") n,
        in("x0") arg1,
        in("x1") arg2,
        lateout("x0") ret,
        options(nostack)
    );
    ret
}

pub unsafe fn syscall3(n: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x16") n,
        in("x0") arg1,
        in("x1") arg2,
        in("x2") arg3,
        lateout("x0") ret,
        options(nostack)
    );
    ret
}

pub unsafe fn syscall4(n: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x16") n,
        in("x0") arg1,
        in("x1") arg2,
        in("x2") arg3,
        in("x3") arg4,
        lateout("x0") ret,
        options(nostack)
    );
    ret
}

pub unsafe fn syscall5(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x16") n,
        in("x0") arg1,
        in("x1") arg2,
        in("x2") arg3,
        in("x3") arg4,
        in("x4") arg5,
        lateout("x0") ret,
        options(nostack)
    );
    ret
}

pub unsafe fn syscall6(
    n: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
) -> usize {
    let mut ret: usize;
    asm!(
        "svc 0",
        in("x16") n,
        in("x0") arg1,
        in("x1") arg2,
        in("x2") arg3,
        in("x3") arg4,
        in("x4") arg5,
        in("x5") arg6,
        lateout("x0") ret,
        options(nostack)
    );
    ret
}
