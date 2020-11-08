use std::error::Error;
// use x11::xlib::{};

pub fn list_toplevel_windows() -> Result<Vec<String>, Box<dyn Error>> {
    // xcb query tree?

    Err(Box::from("AAAAH"))
}

pub fn make_window_fullscreen(_name: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn make_window_borderless(_name: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}