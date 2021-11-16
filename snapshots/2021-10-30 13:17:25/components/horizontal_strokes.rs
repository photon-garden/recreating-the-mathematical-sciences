use crate::prelude::*;
use ::rstar::RTree;
use strokes::*;

pub struct HorizontalStrokes {
    strokes: Strokes,
    boundary_points: Vec<Point2>,
}

enum LineBuilding {
    InBounds,
    OutOfBounds,
}

// previous: out of bounds, current: out of bounds
// do nothing
//
// previous: out of bounds, current: in bounds
// start the line
//
// previous: in bounds, current: out of bounds,
// finish the line
//
// previous: in bounds, current: in bounds,
// update the most recent in bounds point

pub fn new(app: &App, container: &Rect) -> HorizontalStrokes {
    let boundary_points = load_denormalized_boundary_points(app, container);
    let array_points: Vec<[f32; 2]> = boundary_points
        .iter()
        .map(|point| [point.x, point.y])
        .collect();
    let boundary_rtree = RTree::bulk_load(array_points);

    let grid = Grid::new(*container, 1000, 375);
    let mut lines: Vec<Line> = vec![];
    for row in grid.rows() {
        let mut current_line: Line = [pt2(0.0, 0.0), pt2(0.0, 0.0)];
        let mut previous_state = LineBuilding::OutOfBounds;

        for cell in row {
            let point = cell.rect.top_left();
            let [_boundary_x, boundary_y] = boundary_rtree
                .nearest_neighbor(&[point.x, point.y])
                .unwrap();

            let current_state = if boundary_y < &point.y {
                LineBuilding::InBounds
            } else {
                LineBuilding::OutOfBounds
            };

            match (&previous_state, &current_state) {
                (&LineBuilding::OutOfBounds, &LineBuilding::OutOfBounds) => {
                    // Do nothing.
                }
                (&LineBuilding::OutOfBounds, &LineBuilding::InBounds) => {
                    // Start a new line.
                    current_line = [point, point];
                }
                (&LineBuilding::InBounds, &LineBuilding::OutOfBounds) => {
                    // Mark the line as finished.
                    lines.push(current_line);
                }
                (&LineBuilding::InBounds, &LineBuilding::InBounds) => {
                    // Update line end.
                    current_line[1] = point;
                }
            }

            previous_state = current_state;
        }
    }

    let strokes = strokes::new(lines);

    HorizontalStrokes {
        strokes,
        boundary_points,
    }
}

fn load_denormalized_boundary_points(app: &App, container: &Rect) -> Vec<Point2> {
    let points = crate::svg::parse_path(app, "horizontal_boundary.svg");
    let width = container.lerp_w(0.889);
    let height = container.lerp_w(0.233);

    let top_left = pt2(0.0335, 0.4951111).denormalize(container);
    let bottom_right = top_left + vec2(width, -height);

    let bounds = Rect::from_corners(top_left, bottom_right);

    points.denormalize_xy(&bounds)
}

impl HorizontalStrokes {
    pub fn render(&self, params: &mut RenderParams) {
        let draw = params.draw;

        self.strokes.render(params);

        // let stroke_weight = params.container.lerp_w(0.001);

        draw.polyline()
            .points(self.boundary_points.clone())
            .color(soft_black());
    }
}
