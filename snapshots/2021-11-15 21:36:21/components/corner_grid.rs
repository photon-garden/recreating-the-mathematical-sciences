use crate::prelude::*;

pub fn render(params: &mut RenderParams) {
    let draw = params.draw;
    let container = params.container;

    let grid = Grid::new(*container, 3, 3);

    for (y_index, row_width) in (1..=4).rev().enumerate() {
        for x_index in 0..row_width {
            let rect = grid.get(x_index, y_index).unwrap().rect;

            let center = rect.xy();
            let width = rect.w();
            let height = rect.h();

            draw.rect()
                .color(soft_black())
                .w(width)
                .h(height)
                .xy(center);
        }
    }
}
