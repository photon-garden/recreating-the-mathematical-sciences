use crate::prelude::*;

pub fn render(params: &mut RenderParams, down: Grid, right: Grid) {
    let draw = params.draw;

    let down_cells = down.cells.iter();
    let right_cells = right.cells.iter();
    let zipped_cells = down_cells.zip(right_cells);

    for (down_cell, right_cell) in zipped_cells {
        let down_corners = down_cell.rect.corners_iter();
        let right_corners = right_cell.rect.corners_iter();
        let zipped_corners = down_corners.zip(right_corners);

        for (down_corner, right_corner) in zipped_corners {
            draw.line().start(down_corner).end(right_corner).color(RED);
        }
    }
}
