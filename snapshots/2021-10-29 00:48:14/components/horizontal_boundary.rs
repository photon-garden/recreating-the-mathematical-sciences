use crate::prelude::*;

pub struct HorizontalBoundary {
    points: Vec<Point2>,
}

pub fn new(app: &App) -> HorizontalBoundary {
    let points = crate::svg::parse_path(app, "vertical_boundary.svg");
    HorizontalBoundary { points }
}

impl HorizontalBoundary {
    pub fn render(&self, params: &mut RenderParams) {
        circle::render(params)
    }
}
