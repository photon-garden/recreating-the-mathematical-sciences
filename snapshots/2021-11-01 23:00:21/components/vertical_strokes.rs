use crate::prelude::*;
use geo::{coords_iter::CoordsIter, prelude::Contains, Polygon};
use geo_booleanop::boolean::BooleanOp;
use strokes::*;

pub struct VerticalStrokes {
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

pub fn new(app: &App, container: &Rect, num_strokes: usize) -> VerticalStrokes {
    let boundary_polygon = load_denormalized_boundary_polygon(app, container);

    let grid = Grid::new(*container, num_strokes, 1000);
    let mut lines: Vec<Line> = vec![];
    for column in grid.columns() {
        let mut current_line: Line = [pt2(0.0, 0.0), pt2(0.0, 0.0)];
        let mut previous_state = LineBuilding::OutOfBounds;

        for cell in column {
            let point = cell.rect.bottom_left();
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

    VerticalStrokes {
        lines,
        boundary_polygon,
    }
}

fn load_denormalized_boundary_polygon(app: &App, container: &Rect) -> geo::Polygon<f32> {
    let mut denormalized_points = load_denormalized_points(app, container);

    let top_left = pt2(container.left(), denormalized_points[0].y);
    let top_right = pt2(container.right(), denormalized_points.last().unwrap().y);
    let bottom_left = container.bottom_left();
    let bottom_right = container.bottom_right();

    denormalized_points.insert(0, top_left);
    denormalized_points.push(top_right);
    denormalized_points.push(bottom_right);
    denormalized_points.push(bottom_left);
    denormalized_points.push(top_left);

    let jagged_rectangle = denormalized_points.into_geo_polygon();
    let (center, radius) = circle::normalized_center_and_radius();

    dbg!(&container);
    let circle = Path2::regular_polygon(&center, radius, 100)
        .denormalize_xy(container)
        .into_geo_polygon();

    circle
        .intersection(&jagged_rectangle)
        .into_iter()
        .next()
        .unwrap()
}

fn load_denormalized_points(app: &App, container: &Rect) -> Vec<Point2> {
    let points = crate::svg::parse_path(app, "vertical_boundary.svg");

    let width = container.lerp_w(0.901);
    let height = container.lerp_w(0.197);

    let top_left = pt2(0.055, 0.49955556).denormalize(container);
    let bottom_right = top_left + vec2(width, -height);

    let point_bounds = Rect::from_corners(top_left, bottom_right);

    points.denormalize_xy(&point_bounds)
}

impl VerticalStrokes {
    pub fn render(&self, params: &mut RenderParams) {
        strokes::render(&self.lines, params);
        // let stroke_weight = params.container.lerp_w(0.001);

        // let points: Vec<_> = self
        //     .boundary_polygon
        //     .exterior()
        //     .coords_iter()
        //     .map(|coordinate| pt2(coordinate.x, coordinate.y))
        //     .collect();

        // params.draw.polygon().color(soft_black()).points(points);
    }
}
