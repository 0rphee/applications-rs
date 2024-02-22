#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "macos")]
pub use mac::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;
