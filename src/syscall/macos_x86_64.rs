use core::arch::asm;

// Syscalls for macos x86_64 can be found here:
// https://opensource.apple.com/source/xnu/xnu-1504.3.12/bsd/kern/syscalls.master

// macos x86_64 syscalls have an offset of 0x2000000 (2 << 24)
// https://modexp.wordpress.com/2017/01/21/shellcode-osx/
// https://opensource.apple.com/source/xnu/xnu-792.13.8/osfmk/mach/i386/syscall_sw.h
const SYSCALL_SHIFT: usize = 0x2000000;

pub unsafe fn syscall0(n: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n + SYSCALL_SHIFT => ret,
        out("rcx") _, // rcx is used to store old rip
        out("r11") _, // r11 is used to store old rflags
        options(nostack, preserves_flags)
    );
    ret
}

pub unsafe fn syscall1(n: usize, arg1: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n + SYSCALL_SHIFT => ret,
        in("rdi") arg1,
        out("rcx") _, // rcx is used to store old rip
        out("r11") _, // r11 is used to store old rflags
        options(nostack, preserves_flags)
    );
    ret
}

pub unsafe fn syscall2(n: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n + SYSCALL_SHIFT => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        out("rcx") _, // rcx is used to store old rip
        out("r11") _, // r11 is used to store old rflags
        options(nostack, preserves_flags)
    );
    ret
}

pub unsafe fn syscall3(n: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n + SYSCALL_SHIFT => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        out("rcx") _, // rcx is used to store old rip
        out("r11") _, // r11 is used to store old rflags
        options(nostack, preserves_flags)
    );
    ret
}

pub unsafe fn syscall4(n: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let mut ret: usize;
    asm!(
        "syscall",
        inlateout("rax") n + SYSCALL_SHIFT => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        out("rcx") _, // rcx is used to store old rip
        out("r11") _, // r11 is used to store old rflags
        options(nostack, preserves_flags)
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
        "syscall",
        inlateout("rax") n + SYSCALL_SHIFT => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8")  arg5,
        out("rcx") _, // rcx is used to store old rip
        out("r11") _, // r11 is used to store old rflags
        options(nostack, preserves_flags)
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
        "syscall",
        inlateout("rax") n + SYSCALL_SHIFT => ret,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r10") arg4,
        in("r8")  arg5,
        in("r9")  arg6,
        out("rcx") _, // rcx is used to store old rip
        out("r11") _, // r11 is used to store old rflags
        options(nostack, preserves_flags)
    );
    ret
}
