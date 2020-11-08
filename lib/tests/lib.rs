use lib::{list_toplevel_windows, make_window_borderless};

#[test]
fn should_list_toplevel_windows() {
    let windows = list_toplevel_windows()
        .expect("Failed to list windows");

    assert!(windows.len() > 0);
}

#[test]
fn should_be_possible_to_make_window_borderless() {
    let windows = list_toplevel_windows()
        .expect("Failed to list windows");

    for (hwnd, name) in windows {
        if name == String::from("Code") {
            make_window_borderless(hwnd).expect("Failed to make window borderless");
        }
    }
}