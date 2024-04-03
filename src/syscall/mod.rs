// Expose the syscall function out of the module.
mod syscall;
pub use syscall::syscall;

// Support for macos aarch64
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod macos_aarch64;
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
use macos_aarch64::*;

// Support for macos x86_64
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
mod macos_x86_64;
#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
use macos_x86_64::*;
