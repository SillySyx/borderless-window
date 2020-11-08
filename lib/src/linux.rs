use std::error::Error;
// use x11::xlib::{};

pub fn list_toplevel_windows() -> Result<Vec<String>, Box<dyn Error>> {
    // xcb query tree?

    Err(Box::from("AAAAH"))
}

pub fn make_window_borderless() -> Result<(), Box<dyn Error>> {
    Ok(())
}