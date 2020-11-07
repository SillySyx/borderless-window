use std::error::Error;

pub fn list_toplevel_windows() -> Result<Vec<String>, Box<dyn Error>> {
    Err(Box::from("Oppai panic"))
}

pub fn make_window_fullscreen(_name: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn make_window_borderless(_name: String) -> Result<(), Box<dyn Error>> {
    Ok(())
}