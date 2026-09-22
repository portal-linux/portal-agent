//! Bridges guest Wayland windows to native macOS windows via the host compositor.

pub fn connect() {
    unimplemented!("Wayland bridge requires a running compositor")
}

pub fn sync_window_geometry(_surface_id: u32, _x: i32, _y: i32, _width: u32, _height: u32) {
    unimplemented!("window geometry sync requires a running compositor")
}
