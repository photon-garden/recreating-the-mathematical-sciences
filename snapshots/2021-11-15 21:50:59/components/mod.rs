use crate::prelude::*;

mod corner_grid;

pub struct Artwork {}

pub fn new(params: Params) -> Artwork {
    Artwork {}
}

// 487 x 487
// Each long side is 444. 444 / 487 = 0.91170431
// Push right by 37 pixels. Push down by 37 pixels. Normalized = 0.07597536
impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        let grid_container = params.container.scale_wh(0.9117043);
        let shift_amount = params.container.lerp_w(0.07597536);

        let right_grid_container = grid_container.shift_x(shift_amount);
        let down_grid_container = grid_container.shift_y(-shift_amount);

        params.draw.background().color(soft_white());

        corner_grid::render(params, right_grid_container);
        corner_grid::render(params, down_grid_container);
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}
