#[cfg(linux)]
mod linux;

#[cfg(linux)]
pub use linux::{
    list_toplevel_windows,
    make_window_borderless,
    make_window_fullscreen
};

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use windows::{
    list_toplevel_windows,
    make_window_borderless,
    make_window_fullscreen
};