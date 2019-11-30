use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use ggez::timer;
use rand::{rngs::ThreadRng, Rng};

use std::time::{Instant, Duration};
use std::collections::VecDeque;

use crate::tools;

const PARTICLE_LIFETIME: Duration = Duration::from_millis(700);
const PARTICLE_SPAWN_PERIOD: Duration = Duration::from_millis(50);

const PARTICLE_RADIUS_MINMAX: (f32, f32) = (0.5, 2.0);
const PARTICLE_RADIUS_RANGE: f32 = PARTICLE_RADIUS_MINMAX.1 - PARTICLE_RADIUS_MINMAX.0;
const PARTICLE_SPEED_MINMAX: (f32, f32) = (1.0, 10.0);
const PARTICLE_SPEED_RANGE: f32 = PARTICLE_SPEED_MINMAX.1 - PARTICLE_SPEED_MINMAX.0;

pub trait Emitter {
    fn update(&mut self, dt: f32, dt_duration: &Duration, current_position: Option<Point2<f32>>);
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn start_emitting(&mut self);
    fn stop_emitting(&mut self);
}

pub struct ParticleTrail {
    particles: VecDeque<Particle>,
    rand_thread: ThreadRng,
    spawn_timer: Duration,
    last_known_parent_position: Point2<f32>,
    emitt: bool,
}

impl ParticleTrail {
    pub fn new() -> ParticleTrail {
        ParticleTrail {
            particles: VecDeque::with_capacity(0),
            rand_thread: rand::thread_rng(),
            spawn_timer: Duration::from_secs(0),
            last_known_parent_position: Point2::new(-1000.0, -1000.0),
            emitt: true,
        }
    }

    fn add_particle(&mut self, position: Point2<f32>) {
        let radius = PARTICLE_RADIUS_MINMAX.0 + (self.rand_thread.gen::<f32>() * PARTICLE_RADIUS_RANGE);
        let velocity = self.random_velocity();
        
        self.particles.push_back(Particle {
            position,
            velocity,
            radius,
            time_created: Instant::now(),
        });
    }

    // NOTE: Could only check first element and pop until no longer dead
    fn kill_dead_particles(&mut self) {
        while let Some(particle) = self.particles.front() {
            if Instant::now().duration_since(particle.time_created) >= PARTICLE_LIFETIME {
                self.particles.pop_front();
            } else {
                break
            }
        }
    }

    #[inline]
    pub fn is_dead(&self) -> bool {
        !self.emitt && self.particles.is_empty()
    }

    #[inline]
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }

    fn random_velocity(&mut self) -> Vector2<f32> {
        use std::f32::consts::PI;
        const TWO_PI: f32 = PI * 2.0;

        let angle = self.rand_thread.gen::<f32>() * TWO_PI;
        let speed = PARTICLE_SPEED_MINMAX.0 + self.rand_thread.gen::<f32>() * PARTICLE_SPEED_RANGE;

        tools::get_components(speed, angle)
    }
}

impl Emitter for ParticleTrail {
    fn update(&mut self, dt: f32, dt_duration: &Duration, current_position: Option<Point2<f32>>) {
        if let Some(pos) = current_position {
            self.last_known_parent_position = pos;
        }

        self.kill_dead_particles();

        for particle in self.particles.iter_mut() {
            particle.update(dt);
        }

        if self.emitt {
            self.spawn_timer += *dt_duration;
            if self.spawn_timer >= PARTICLE_SPAWN_PERIOD {
                self.spawn_timer -= PARTICLE_SPAWN_PERIOD;
                self.add_particle(self.last_known_parent_position);
            }
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        for p in self.particles.iter() {
            p.draw(ctx)?;
        }
        Ok(())
    }

    fn start_emitting(&mut self) {
        self.emitt = true;
    }

    fn stop_emitting(&mut self) {
        self.emitt = false;
    }
}

struct Particle {
    position: Point2<f32>,
    velocity: Vector2<f32>,
    radius: f32,
    time_created: Instant,
}

impl Particle {
    fn draw(&self, ctx: &mut Context) -> GameResult {
        tools::draw_circle(
            ctx,
            self.position,
            self.radius,
            [
                0.1,
                0.4,
                0.8,
                1.0 - timer::duration_to_f64(Instant::now().duration_since(self.time_created)) as f32/timer::duration_to_f64(PARTICLE_LIFETIME) as f32
            ].into()
        )
    }

    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}