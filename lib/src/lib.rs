#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::{
    list_toplevel_windows,
    make_window_borderless,
    make_window_fullscreen
};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::{
    list_toplevel_windows,
    make_window_borderless,
    make_window_fullscreen
};