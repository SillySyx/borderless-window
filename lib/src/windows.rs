use std::error::Error;

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::{EnumWindows, GetWindowTextLengthW, GetWindowTextW};

unsafe fn read_window_title(hwnd: HWND) -> Option<String> {
    let window_text_length = GetWindowTextLengthW(hwnd);
    if window_text_length == 0 {
        return None;
    }

    let window_text_length = window_text_length + 1;

    let mut buffer: Vec<u16> = Vec::with_capacity(window_text_length as usize);
    let read_length = GetWindowTextW(hwnd, buffer.as_mut_ptr(), window_text_length);

    if read_length == 0 {
        return None;
    }

    buffer.set_len(read_length as usize);
    let string = String::from_utf16_lossy(&buffer);

    Some(string)
}

unsafe extern "system" fn enumerate_callback(hwnd: HWND, _: LPARAM) -> BOOL {
    if let Some(text) = read_window_title(hwnd) {
        println!("{}", text);
    }

    1
}

#[cfg(windows)]
pub fn list_toplevel_windows() -> Result<Vec<String>, Box<dyn Error>> {
    unsafe {
        EnumWindows(Some(enumerate_callback), 0);
    }
    Err(Box::from("Oppai panic"))
}

pub fn make_window_fullscreen(_name: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn make_window_borderless(_name: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}