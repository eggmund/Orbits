use ggez::nalgebra::{Vector2, Point2};
use ggez::graphics::{self, MeshBuilder, DrawMode, Color};
use ggez::{Context, GameResult};
use palette::{rgb::LinSrgb, Hsv};

use std::time::Duration;

use crate::tools;

pub const PLANET_DENSITY: f32 = 5000.0;
const PLANET_RADIUS_COLORING_LOOP: f32 = 5.0;  // Planets are rainbow and colour repeats every 10
const PLANET_DEBRIS_SPAWN_PROTECTION: (u64, u64) = (0, 500);   // 0.5 Seconds of spawn protection for debris

pub struct Planet {
    pub id: usize,
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub resultant_force: Vector2<f32>,
    pub last_resultant_force: Vector2<f32>,       // For debug
    color: Color,
    spawn_protection_timer: Option<Duration>,
}

impl Planet {
    pub fn new(id: usize, position: Point2<f32>, velocity: Option<Vector2<f32>>, mass: Option<f32>, radius: f32, spawn_protection_timer: Option<Duration>) -> Planet {
        let hsv = Hsv::new((radius/PLANET_RADIUS_COLORING_LOOP * 360.0) % 360.0, 1.0, 1.0);
        let rgb = LinSrgb::from(hsv);

        Planet {
            id,
            position,
            velocity: velocity.unwrap_or_else(|| Vector2::new(0.0, 0.0)),
            mass: mass.unwrap_or_else(|| Self::mass_from_radius(radius, PLANET_DENSITY)),
            radius,
            resultant_force: Vector2::new(0.0, 0.0),
            last_resultant_force: Vector2::new(0.0, 0.0),
            color: [rgb.red, rgb.blue, rgb.green, 1.0].into(),
            spawn_protection_timer,
        }
    }

    #[inline]
    pub fn update(&mut self, dt: f32, dt_duration: &Duration) {
        let acceleration = self.resultant_force/self.mass;  // F = ma, F/m = a
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
        
        self.last_resultant_force = self.resultant_force;
        self.resultant_force = Vector2::new(0.0, 0.0);

        if let Some(spawn_timer) = self.spawn_protection_timer.as_mut() {
            if !(*spawn_timer < *dt_duration) {
                *spawn_timer -= *dt_duration;
            } else {        // Time is up
                self.spawn_protection_timer = None;
            }
        }
    }

    pub fn draw(&self, mesh_builder: &mut MeshBuilder) {
        //tools::draw_circle(ctx, self.position, self.radius, graphics::WHITE)
        mesh_builder.circle(
            DrawMode::fill(),
            self.position,
            self.radius,
            0.1,
            self.color,
        );
    }

    #[inline]
    fn mass_from_radius(radius: f32, density: f32) -> f32 {
        // m = vd
        tools::volume_of_sphere(radius) * density
    }

    #[inline]
    fn radius_from_mass(mass: f32, density: f32) -> f32 {
        // v = m/d, r = cube_root( 3v/4pi )
        tools::inverse_volume_of_sphere(mass/density)
    }

    #[inline]
    pub fn has_spawn_protection(&self) -> bool {
        self.spawn_protection_timer.is_some()
    }
}