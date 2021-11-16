use crate::prelude::*;

pub fn render(params: &mut RenderParams, grid: Grid) {
    let draw = params.draw;
    let container = params.container;

    for (y_index, row_width) in (1..=grid.size()).enumerate() {
        for x_index in 0..row_width {
            let rect = grid.get(x_index, y_index).unwrap().rect;

            let center = rect.xy();
            let width = rect.w();
            let height = rect.h();
            let stroke_weight = container.lerp_w(0.005);

            draw.rect()
                .no_fill()
                .stroke_color(soft_black())
                .stroke_weight(stroke_weight)
                .w(width)
                .h(height)
                .xy(center);
        }
    }
}
