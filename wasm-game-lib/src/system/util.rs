use web_sys::Window;

/// Return the window or panic
pub fn window() -> Window {
    web_sys::window().expect("should have a window")
}
