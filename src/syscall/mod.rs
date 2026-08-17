// Support for aarch64
#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
use aarch64::*;

// Support for macos x86_64
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod x86_64_macos;
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use x86_64_macos::*;

type Arg = Option<usize>;

pub unsafe fn syscall(n: usize, a1: Arg, a2: Arg, a3: Arg, a4: Arg, a5: Arg, a6: Arg) -> usize {
    match (a1, a2, a3, a4, a5, a6) {
        (Some(a1), Some(a2), Some(a3), Some(a4), Some(a5), Some(a6)) => {
            syscall6(n, a1, a2, a3, a4, a5, a6)
        }
        (Some(a1), Some(a2), Some(a3), Some(a4), Some(a5), _) => syscall5(n, a1, a2, a3, a4, a5),
        (Some(a1), Some(a2), Some(a3), Some(a4), _, _) => syscall4(n, a1, a2, a3, a4),
        (Some(a1), Some(a2), Some(a3), _, _, _) => syscall3(n, a1, a2, a3),
        (Some(a1), Some(a2), _, _, _, _) => syscall2(n, a1, a2),
        (Some(a1), _, _, _, _, _) => syscall1(n, a1),
        (_, _, _, _, _, _) => syscall0(n),
    }
}
