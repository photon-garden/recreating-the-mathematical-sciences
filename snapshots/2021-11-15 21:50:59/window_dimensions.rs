pub fn load() -> WindowDimensions {
    // let width_aspect = 2;
    // let height_aspect = 3;
    // let size = 350;
    // let width = size * width_aspect;
    // let height = size * height_aspect;

    let width = 1075;
    let height = 1075;

    let container_width = width as f32 * 0.06;
    let container_height = height as f32 * 0.06;

    WindowDimensions {
        width,
        height,
        container_width,
        container_height,
    }
}

pub struct WindowDimensions {
    pub width: u32,
    pub height: u32,
    pub container_width: f32,
    pub container_height: f32,
}
