use crate::prelude::*;
use rstar::*;

pub struct HorizontalStrokes {
    points: Vec<Point2>,
    boundary_rtree: RTree<My<Point2>>,
}

pub fn new(app: &App, container: &Rect) -> HorizontalStrokes {
    let points = load_denormalized_points(app, container);
    let my_points: Vec<My<Point2>> = points.iter().map(|point| My(*point)).collect();

    HorizontalStrokes {
        points,
        boundary_rtree: RTree::bulk_load(my_points),
    }
}

fn load_denormalized_points(app: &App, container: &Rect) -> Vec<Point2> {
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

        // let stroke_weight = params.container.lerp_w(0.001);

        draw.polyline()
            .points(self.points.clone())
            .color(soft_black());
    }
}
