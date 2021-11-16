use super::corner_cells;
use crate::prelude::*;

pub fn render(params: &mut RenderParams, grid: Grid) {
    let draw = params.draw;
    let container = params.container;

    for cell in corner_cells::get(&grid) {
        let rect = cell.rect;

        let center = rect.xy();
        let width = rect.w();
        let height = rect.h();
        let stroke_weight = container.lerp_w(0.005);

        draw.rect()
            .no_fill()
            .stroke_color(cooper_black())
            .stroke_weight(stroke_weight)
            .w(width)
            .h(height)
            .xy(center);
    }
}
