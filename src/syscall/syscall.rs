use super::{syscall0, syscall1, syscall2, syscall3, syscall4, syscall5, syscall6};

type Arg = Option<usize>;

pub unsafe fn syscall(n: usize, a1: Arg, a2: Arg, a3: Arg, a4: Arg, a5: Arg, a6: Arg) -> usize {
    match (a1, a2, a3, a4, a5, a6) {
        (Some(a1), Some(a2), Some(a3), Some(a4), Some(a5), Some(a6)) => {
            return syscall6(n, a1, a2, a3, a4, a5, a6);
        }
        (Some(a1), Some(a2), Some(a3), Some(a4), Some(a5), _) => {
            return syscall5(n, a1, a2, a3, a4, a5);
        }
        (Some(a1), Some(a2), Some(a3), Some(a4), _, _) => {
            return syscall4(n, a1, a2, a3, a4);
        }
        (Some(a1), Some(a2), Some(a3), _, _, _) => {
            return syscall3(n, a1, a2, a3);
        }
        (Some(a1), Some(a2), _, _, _, _) => {
            return syscall2(n, a1, a2);
        }
        (Some(a1), _, _, _, _, _) => {
            return syscall1(n, a1);
        }
        (_, _, _, _, _, _) => {
            return syscall0(n);
        }
    }
}
