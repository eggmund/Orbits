use ggez::nalgebra::{Vector2, Point2};

use std::f32::consts::PI;
use crate::{G, planet::Planet};

#[inline]
pub fn volume_of_sphere(radius: f32) -> f32 {
    (4.0/3.0) * PI * radius.powi(3)
}

#[inline]
pub fn inverse_volume_of_sphere(volume: f32) -> f32 {
    ((3.0 * volume)/(4.0 * PI)).powf(1.0/3.0)
}

#[inline]
pub fn get_angle(vec: &Vector2<f32>) -> f32 {
    vec.y.atan2(vec.x)
}

#[inline]
pub fn get_components(magnitude: f32, angle: f32) -> Vector2<f32> {
    Vector2::new(magnitude * angle.cos(), magnitude * angle.sin())
}

#[inline]
pub fn newtonian_grav(pl1: &mut Planet, pl2: &mut Planet) {
    let dist_vec = pl2.position - pl1.position;
    let angle = get_angle(&dist_vec);

    let dist_squared = dist_vec.x.powi(2) + dist_vec.y.powi(2);

    //if dist_squared > pl1.radius.powi(2) + pl2.radius.powi(2) {    // if inside other body, no force. NOTE: Already checked.
    let force = (G * pl1.mass * pl2.mass)/dist_squared;
    let force_vec = get_components(force, angle);

    pl1.resultant_force += force_vec;
    pl2.resultant_force -= force_vec;
    //}
}

// Box collision for circles (AABB), and then circle collision
#[inline]
pub fn check_collision(pos1: &Point2<f32>, pos2: &Point2<f32>, r1: f32, r2: f32) -> bool {
    let min_dist = r1 + r2;
    let dist_vec = pos2 - pos1;

    if dist_vec.x.abs() <= min_dist && dist_vec.y.abs() <= min_dist { // If in box
        println!("In box {} {}, {}", dist_vec.x, dist_vec.y, min_dist);
        dist_vec.x.powi(2) + dist_vec.y.powi(2) <= min_dist.powi(2)
    } else {
        false
    }
}