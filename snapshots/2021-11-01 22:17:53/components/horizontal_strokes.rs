use crate::prelude::*;
use geo::coords_iter::CoordsIter;
use geo_booleanop::boolean::BooleanOp;
// use strokes::*;

pub struct HorizontalStrokes {
    // strokes: Strokes,
// boundary_polygon: Polygon<f32>,
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
    // let boundary_polygon = load_denormalized_boundary_polygon(app, container);

    // let grid = Grid::new(*container, 1000, 375);
    // let mut lines: Vec<Line> = vec![];
    // for row in grid.rows() {
    //     let mut current_line: Line = [pt2(0.0, 0.0), pt2(0.0, 0.0)];
    //     let mut previous_state = LineBuilding::OutOfBounds;

    //     for cell in row {
    //         let point = cell.rect.top_left();
    //         let [_boundary_x, boundary_y] = boundary_rtree
    //             .nearest_neighbor(&[point.x, point.y])
    //             .unwrap();

    //         let current_state = if boundary_y < &point.y {
    //             LineBuilding::InBounds
    //         } else {
    //             LineBuilding::OutOfBounds
    //         };

    //         match (&previous_state, &current_state) {
    //             (&LineBuilding::OutOfBounds, &LineBuilding::OutOfBounds) => {
    //                 // Do nothing.
    //             }
    //             (&LineBuilding::OutOfBounds, &LineBuilding::InBounds) => {
    //                 // Start a new line.
    //                 current_line = [point, point];
    //             }
    //             (&LineBuilding::InBounds, &LineBuilding::OutOfBounds) => {
    //                 // Mark the line as finished.
    //                 lines.push(current_line);
    //             }
    //             (&LineBuilding::InBounds, &LineBuilding::InBounds) => {
    //                 // Update line end.
    //                 current_line[1] = point;
    //             }
    //         }

    //         previous_state = current_state;
    //     }
    // }

    // let strokes = strokes::new(lines);

    HorizontalStrokes {
        // strokes,
        // boundary_polygon,
    }
}

fn load_denormalized_boundary_polygon(app: &App, container: &Rect) -> geo::MultiPolygon<f32> {
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
    let (center, radius) = circle::normalized_center_and_radius();

    let circle = Path2::regular_polygon(&center, radius, 100)
        .denormalize_xy(container)
        .into_geo_polygon();

    circle.intersection(&jagged_rectangle)
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
        let draw = params.draw;

        // self.strokes.render(params);
        // let stroke_weight = params.container.lerp_w(0.001);

        dbg!("Loading polygon.");
        let multi_polygon = load_denormalized_boundary_polygon(params.app, params.container);

        dbg!("Loaded. Converting coordinates.");
        for polygon in multi_polygon {
            let points: Vec<_> = polygon
                .exterior()
                .coords_iter()
                .map(|coordinate| pt2(coordinate.x, coordinate.y))
                .collect();

            dbg!("Done.");

            draw.polygon().color(soft_black()).points(points);
        }
    }
}
