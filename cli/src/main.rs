use lib::{list_toplevel_windows, make_window_borderless};

fn main() {
    if let Some(hwnd) = should_make_window_borderless() {
        make_window_borderless(hwnd).expect("Failed to make window borderless");
        return;
    }

    if should_list_windows() {
        let windows = list_toplevel_windows().expect("Failed to list windows");

        for (hwnd, name) in windows {
            println!("{:?}\t\t{}", hwnd, name);
        }
        return;
    }

    show_help();
}

fn should_make_window_borderless() -> Option<usize> {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(value) => value,
        None => return None,
    };

    if command != "borderless" {
        return None;
    }

    let hwnd = match args.get(2) {
        Some(value) => value,
        None => return None,
    };

    match hwnd.parse::<usize>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn should_list_windows() -> bool {
    let args: Vec<String> = std::env::args().collect();

    let command = match args.get(1) {
        Some(value) => value,
        None => return false,
    };

    command == "list"
}

fn show_help() {
    println!("list \t\t\t Shows all toplevel windows");
    println!("borderless <hwnd> \t Makes the window borderless");
}