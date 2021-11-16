use crate::prelude::*;

pub struct VerticalBoundary {
    points: Vec<Point2>,
}

pub fn new(app: &App) -> VerticalBoundary {
    let points = crate::svg::parse_path(app, "vertical_boundary.svg");
    VerticalBoundary { points }
}

impl VerticalBoundary {
    pub fn render(&self, params: &mut RenderParams) {
        let draw = params.draw;
        let denormalized_points = self.points.denormalize_xy(&params.container);
        let stroke_weight = params.container.lerp_w(0.001);

        draw.polyline()
            .points(denormalized_points)
            .color(soft_black());
    }
}
