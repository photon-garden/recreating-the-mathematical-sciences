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
        let bounds = Rect::from_w_h(width, height);

        let denormalized_points = self.points.denormalize_xy(&bounds);

        let stroke_weight = params.container.lerp_w(0.001);

        draw.polyline()
            .points(denormalized_points)
            .color(soft_black());
    }
}
