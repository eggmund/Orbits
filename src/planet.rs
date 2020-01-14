use ggez::nalgebra::{Vector2, Point2};
use ggez::graphics::{self, MeshBuilder, DrawMode};
use ggez::{Context, GameResult};

use crate::tools;

pub const PLANET_DENSITY: f32 = 5000.0;

pub struct Planet {
    pub id: usize,
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub resultant_force: Vector2<f32>,
    pub last_resultant_force: Vector2<f32>,       // For debug
}

impl Planet {
    pub fn new(id: usize, position: Point2<f32>, velocity: Option<Vector2<f32>>, mass: Option<f32>, radius: f32) -> Planet {
        Planet {
            id,
            position,
            velocity: velocity.unwrap_or_else(|| Vector2::new(0.0, 0.0)),
            mass: mass.unwrap_or_else(|| Self::mass_from_radius(radius, PLANET_DENSITY)),
            radius,
            resultant_force: Vector2::new(0.0, 0.0),
            last_resultant_force: Vector2::new(0.0, 0.0),
        }
    }

    #[inline]
    pub fn update(&mut self, dt: f32) {
        let acceleration = self.resultant_force/self.mass;  // F = ma, F/m = a
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
        
        self.last_resultant_force = self.resultant_force;
        self.resultant_force = Vector2::new(0.0, 0.0);
    }

    pub fn draw(&self, mesh_builder: &mut MeshBuilder) {
        //tools::draw_circle(ctx, self.position, self.radius, graphics::WHITE)
        mesh_builder.circle(
            DrawMode::fill(),
            self.position,
            self.radius,
            0.1,
            graphics::WHITE
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
}