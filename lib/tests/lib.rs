use lib::list_toplevel_windows;

#[test]
fn should_list_toplevel_windows() {
    list_toplevel_windows().expect("Failed to list windows");
}