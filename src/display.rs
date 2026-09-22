//! Handles guest display resolution and DPI changes requested by the macOS host.

pub fn set_resolution(_width: u32, _height: u32) {
    unimplemented!("resolution changes require a running guest session")
}

pub fn set_scale_factor(_scale: f32) {
    unimplemented!("DPI changes require a running guest session")
}
