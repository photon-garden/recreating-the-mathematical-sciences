use crate::prelude::*;
use geo::{coords_iter::CoordsIter, prelude::Contains, Polygon};
use geo_booleanop::boolean::BooleanOp;
use strokes::*;

pub struct HorizontalStrokes {
    lines: Vec<Line>,
    boundary_polygon: Polygon<f32>,
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

pub fn new(app: &App, container: &Rect, num_strokes: usize) -> HorizontalStrokes {
    let boundary_polygon = load_denormalized_boundary_polygon(app, container);

    let grid = Grid::new(*container, 1000, num_strokes);
    let mut lines: Vec<Line> = vec![];
    for row in grid.rows() {
        let mut current_line: Line = [pt2(0.0, 0.0), pt2(0.0, 0.0)];
        let mut previous_state = LineBuilding::OutOfBounds;

        for cell in row {
            let point = cell.rect.top_left();
            let geo_coordinate = geo::point!(x: point.x, y: point.y);
            let in_bounds = boundary_polygon.contains(&geo_coordinate);

            let current_state = if in_bounds {
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

    HorizontalStrokes {
        lines,
        boundary_polygon,
    }
}

fn load_denormalized_boundary_polygon(app: &App, container: &Rect) -> geo::Polygon<f32> {
    let mut denormalized_points = load_denormalized_points(app, container);

    let bottom_left = pt2(container.left(), denormalized_points[0].y);
    let bottom_right = pt2(container.right(), denormalized_points.last().unwrap().y);
    let top_right = container.top_right();
    let top_left = container.top_left();

    denormalized_points.insert(0, bottom_left);
    denormalized_points.push(bottom_right);
    denormalized_points.push(top_right);
    denormalized_points.push(top_left);
    denormalized_points.push(bottom_left);

    let jagged_rectangle = denormalized_points.into_geo_polygon();
    let (normalized_center, normalized_radius) = circle::normalized_center_and_radius();
    let denormalized_center = container.denormalize_xy(&normalized_center);
    let denormalized_radius = container.lerp_w(normalized_radius + 0.001);

    let circle =
        Path2::regular_polygon(&denormalized_center, denormalized_radius, 100).into_geo_polygon();

    circle
        .intersection(&jagged_rectangle)
        .into_iter()
        .next()
        .unwrap()
}

fn load_denormalized_points(app: &App, container: &Rect) -> Vec<Point2> {
    let points = crate::svg::parse_path(app, "horizontal_boundary.svg");

    let width = container.lerp_w(0.889);
    let height = container.lerp_w(0.233);

    let top_left = pt2(0.0335, 0.4951111).denormalize(container);
    let bottom_right = top_left + vec2(width, -height);

    let point_bounds = Rect::from_corners(top_left, bottom_right);

    points.denormalize_xy(&point_bounds)
}

impl HorizontalStrokes {
    pub fn render(&self, params: &mut RenderParams) {
        strokes::render(&self.lines, params);

        // let stroke_weight = params.container.lerp_w(0.001);

        // let points: Vec<_> = self
        //     .boundary_polygon
        //     .exterior()
        //     .coords_iter()
        //     .map(|coordinate| pt2(coordinate.x, coordinate.y))
        //     .collect();

        // draw.polygon().color(soft_black()).points(points);
    }
}
