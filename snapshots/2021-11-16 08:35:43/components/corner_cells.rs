use crate::prelude::*;

pub fn get(grid: &Grid) -> Vec<&Cell> {
    let mut cells: Vec<&Cell> = vec![];
    for (y_index, row_width) in (1..=grid.size()).enumerate() {
        for x_index in 0..row_width {
            let cell = grid.get(x_index, y_index).unwrap();
            cells.push(cell);
        }
    }

    cells
}
