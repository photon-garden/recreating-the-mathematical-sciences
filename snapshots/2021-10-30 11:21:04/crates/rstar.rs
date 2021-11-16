use crate::prelude::*;
use rstar::{RTreeObject, AABB};

pub struct My<T>(pub T);

// https://docs.rs/rstar/0.9.1/rstar/trait.RTreeObject.html
impl RTreeObject for My<Point2> {
    type Envelope = AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let point = self.0;
        AABB::from_point([point.x, point.y])
    }
}
