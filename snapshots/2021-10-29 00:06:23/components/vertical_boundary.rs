use crate::prelude::*;

pub struct VerticalBoundary {}

pub fn new(app: &App) -> VerticalBoundary {
    let points = crate::svg::parse_path(app, "vertical_boundary.svg");
    VerticalBoundary {}
}

impl VerticalBoundary {
    pub fn render(&self, params: &mut RenderParams) {
        circle::render(params)
    }
}
