
use nalgebra::{Point2, Vector2};
use crate::{Float, body::Body};

#[derive(new)]
pub struct Star {
    p: Point2<Float>,
    v: Vector2<Float>,
    m: Float,
    pub radius: f32,    // More for drawing than anything really
}

impl Star {
}

impl Body for Star {
    default_body_gets!(p, v, m);
}

