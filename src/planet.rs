use ggez::nalgebra::{Vector2, Point2};
use ggez::graphics::{MeshBuilder, DrawMode, Color};
use ggez::{Context, GameResult};
use ggez::timer;
use palette::{rgb::LinSrgb, Hsv};

use std::time::{Duration, Instant};
use std::collections::VecDeque;

use crate::tools;

pub const PLANET_DENSITY: f32 = 5000.0;
const PLANET_RADIUS_COLORING_LOOP: f32 = 5.0;  // Planets are rainbow and colour repeats every 10

pub struct Planet {
    pub id: usize,
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    pub mass: f32,
    pub radius: f32,
    pub resultant_force: Vector2<f32>,
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
            color: [rgb.red, rgb.blue, rgb.green, 1.0].into(),
            spawn_protection_timer,
        }
    }

    #[inline]
    pub fn update(&mut self, dt: f32, dt_duration: &Duration) {
        let acceleration = self.resultant_force/self.mass;  // F = ma, F/m = a
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
        
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

const PLANET_TRAIL_NODE_PLACEMENT_PERIOD: u64 = 32;
const PLANET_TRAIL_NODE_LIFETIME: u64 = 1000;

pub struct PlanetTrail {
    nodes: VecDeque<PlanetTrailNode>,
    node_placement_timer: Duration,
    has_parent: bool,
}

impl PlanetTrail {
    pub fn new(start_pos: Point2<f32>) -> Self {
        let mut nodes = VecDeque::with_capacity(32);
        nodes.push_front(PlanetTrailNode::from(start_pos));

        Self {
            nodes,
            node_placement_timer: Duration::new(0, 0),
            has_parent: true,
        }
    }

    pub fn update(&mut self, dt_duration: &Duration, parent_pos: Option<Point2<f32>>) {
        self.kill_dead_nodes();

        if let Some(parent_pos) = parent_pos {
            self.has_parent = true;
            self.node_placement_timer += *dt_duration;

            let period = Duration::from_millis(PLANET_TRAIL_NODE_PLACEMENT_PERIOD);
            if self.node_placement_timer > period {
                // Place new node
                self.add_node(parent_pos);
                self.node_placement_timer -= period;
            }
        } else {
            self.has_parent = false;
        }
    }

    pub fn draw(&self, mesh: &mut MeshBuilder) -> GameResult {
        let len = self.node_count();
        if len > 1 {
            for i in 0..len-1 {
                // Connect points
                let alpha = (1.0 - timer::duration_to_f64(Instant::now().duration_since(self.nodes[i].time_created)) as f32/timer::duration_to_f64(Duration::from_millis(PLANET_TRAIL_NODE_LIFETIME)) as f32).max(0.0).powi(2);
                //println!("Building line: {:?}, {:?}", self.nodes[i].pos, self.nodes[i + 1].pos);

                mesh.line(
                    &[self.nodes[i].pos, self.nodes[i + 1].pos],
                    1.0,
                    [0.1, 0.4, 0.8, alpha/4.0].into()
                )?;
            }
        }

        Ok(())
    }

    #[inline]
    fn kill_dead_nodes(&mut self) {
        while let Some(node) = self.nodes.front() {
            if Instant::now().duration_since(node.time_created) >= Duration::from_millis(PLANET_TRAIL_NODE_LIFETIME) {
                self.nodes.pop_front();
            } else {
                break
            }
        }
    }

    #[inline]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        self.nodes.is_empty() && !self.has_parent
    }

    #[inline]
    pub fn add_node(&mut self, pos: Point2<f32>) {
        // Make sure distance from last node is a sufficient distance so that line can be drawn without errors
        let can_place = {
            if let Some(last_node) = self.nodes.back() {
                self.nodes.is_empty() || ((pos.x - last_node.pos.x).powi(2) + (pos.y - last_node.pos.y).powi(2)) > 0.1
            } else {
                false
            }
        };

        if can_place {
            self.nodes.push_back(PlanetTrailNode::from(pos));
        }
    }
}

struct PlanetTrailNode {
    pos: Point2<f32>,
    time_created: Instant,
}

impl From<Point2<f32>> for PlanetTrailNode {
    fn from(pos: Point2<f32>) -> Self {
        Self {
            pos,
            time_created: Instant::now(),
        }
    }
}