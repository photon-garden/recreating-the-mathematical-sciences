pub fn load() -> ConfigParams {
    // let width_aspect = 2;
    // let height_aspect = 3;
    // let size = 350;
    // let width = size * width_aspect;
    // let height = size * height_aspect;

    let width = 1075;
    let height = 805;

    let container_width = width as f32 * 0.687;
    let container_height = height as f32;

    ConfigParams {
        width,
        height,
        container_width,
        container_height,
    }
}

pub struct ConfigParams {
    pub width: u32,
    pub height: u32,
    pub container_width: f32,
    pub container_height: f32,
}
