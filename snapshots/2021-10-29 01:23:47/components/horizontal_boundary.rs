use crate::prelude::*;

pub struct HorizontalBoundary {
    points: Vec<Point2>,
}

pub fn new(app: &App) -> HorizontalBoundary {
    let points = crate::svg::parse_path(app, "horizontal_boundary.svg");
    HorizontalBoundary { points }
}

impl HorizontalBoundary {
    pub fn render(&self, params: &mut RenderParams) {
        let draw = params.draw;

        let width = params.container.lerp_w(0.889);
        let height = params.container.lerp_w(0.233);

        let top_left = pt2(0.0295, 0.4951111).denormalize(params.container);
        let bottom_right = top_left + vec2(width, -height);

        let bounds = Rect::from_corners(top_left, bottom_right);

        let denormalized_points = self.points.denormalize_xy(&bounds);

        let stroke_weight = params.container.lerp_w(0.001);

        draw.polyline()
            .points(denormalized_points)
            .color(soft_black());
    }
}
