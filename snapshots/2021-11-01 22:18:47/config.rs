pub fn load() -> ConfigParams {
    // let width_aspect = 2;
    // let height_aspect = 3;
    // let size = 350;
    // let width = size * width_aspect;
    // let height = size * height_aspect;

    ConfigParams {
        width: 1000,
        height: 1125,
        scale: 0.8,
    }
}

pub struct ConfigParams {
    pub width: u32,
    pub height: u32,
    pub scale: f32,
}
