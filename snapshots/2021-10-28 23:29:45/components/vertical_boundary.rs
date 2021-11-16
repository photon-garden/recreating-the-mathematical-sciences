use crate::prelude::*;

pub struct VerticalBoundary {}

pub fn new() -> VerticalBoundary {
    VerticalBoundary {}
}

impl VerticalBoundary {
    pub fn render(&self, params: &mut RenderParams) {
        circle::render(params)
    }
}
