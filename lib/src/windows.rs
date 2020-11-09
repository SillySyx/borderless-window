use std::error::Error;

use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

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

unsafe extern "system" fn enumerate_callback(hwnd: HWND, userdata: LPARAM) -> BOOL {
    let windows: &mut Vec<(usize, String)> = std::mem::transmute(userdata);

    if let Some(text) = read_window_title(hwnd) {
        let handle = hwnd as usize;
        windows.push((handle, text));
    }

    TRUE
}

unsafe fn make_borderless(hwnd: HWND) -> Result<(), Box<dyn Error>> {
    SetWindowLongPtrW(hwnd, GWL_STYLE, WS_POPUP as isize);

    let x = 0;
    let y = 0;
    let width = GetSystemMetrics(SM_CXSCREEN);
    let height = GetSystemMetrics(SM_CYSCREEN);
    let flags = SWP_SHOWWINDOW | SWP_FRAMECHANGED;

    if SetWindowPos(hwnd, HWND_TOP, x, y, width, height, flags) == FALSE {
        return Err(Box::from("failed to make window borderless"));
    }

    Ok(())
}

pub fn list_toplevel_windows() -> Result<Vec<(usize, String)>, Box<dyn Error>> {
    let mut windows: Vec<(usize, String)> = vec![];

    unsafe {
        let userdata = &mut windows as *mut _;
        EnumWindows(Some(enumerate_callback), userdata as LPARAM);
    }

    Ok(windows)
}

pub fn make_window_borderless(hwnd: usize) -> Result<(), Box<dyn Error>> {
    unsafe { make_borderless(hwnd as HWND) }
}
