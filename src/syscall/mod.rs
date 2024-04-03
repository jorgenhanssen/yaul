// Expose the syscall function.
mod syscall;
pub use syscall::syscall;

// Support for macos aarch64
#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
use aarch64::*;

// Support for macos x86_64
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod x86_64_macos;
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use x86_64_macos::*;
