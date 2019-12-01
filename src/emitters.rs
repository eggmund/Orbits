use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use ggez::timer;
use ggez::graphics::Color;
use rand::{rngs::ThreadRng, Rng};

use std::time::{Instant, Duration};
use std::collections::VecDeque;

use crate::tools;

pub trait Emitter {
    fn update(&mut self, dt: f32, dt_duration: &Duration, updated_position: Option<Point2<f32>>);
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn start_emitting(&mut self);
    fn stop_emitting(&mut self);
}

pub struct ParticleSystem {
    particles: VecDeque<Particle>,
    rand_thread: ThreadRng,
    spawn_timer: Duration,
    position: Point2<f32>,
    emitt: bool,
    params: ParticleSystemParam,
    stop_timer: Duration,
}

impl ParticleSystem {
    pub fn new(position: Point2<f32>, params: ParticleSystemParam) -> ParticleSystem {
        ParticleSystem {
            particles: VecDeque::with_capacity(0),
            rand_thread: rand::thread_rng(),
            spawn_timer: Duration::from_secs(0),
            position,
            emitt: true,
            params,
            stop_timer: Duration::new(0, 0),
        }
    }

    fn add_particle(&mut self, position: Point2<f32>) {
        let radius = self.params.particle_radius_minmax.0 + (self.rand_thread.gen::<f32>() * (self.params.particle_radius_minmax.1 - self.params.particle_radius_minmax.0));
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
            if Instant::now().duration_since(particle.time_created) >= self.params.particle_lifetime {
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
        let speed = self.params.particle_speed_minmax.0 + self.rand_thread.gen::<f32>() * (self.params.particle_speed_minmax.1 - self.params.particle_speed_minmax.0);

        tools::get_components(speed, angle)
    }
}

impl Emitter for ParticleSystem {
    fn update(&mut self, dt: f32, dt_duration: &Duration, updated_position: Option<Point2<f32>>) {
        if let Some(pos) = updated_position {
            self.position = pos;
        }

        self.kill_dead_particles();

        for particle in self.particles.iter_mut() {
            particle.update(dt);
        }

        self.stop_timer += *dt_duration;
        if let Some(stop_after) = self.params.stop_after {
            if self.stop_timer >= stop_after {
                self.stop_emitting();
            }
        }

        if self.emitt {
            self.spawn_timer += *dt_duration;
            if self.spawn_timer >= self.params.emission_period {
                let rounds_missed = (timer::duration_to_f64(self.spawn_timer)/timer::duration_to_f64(self.params.emission_period)).floor() as usize;     // Due to framerate
                //println!("Rounds missed: {}. Timer: {:?}, round time: {:?}", rounds_missed, self.spawn_timer, self.params.emission_period);
                for _ in 0..rounds_missed {
                    self.add_particle(self.position);
                }

                self.spawn_timer -= self.params.emission_period + (self.params.emission_period * (rounds_missed - 1) as u32);
            }
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        for p in self.particles.iter() {
            let mut col = self.params.base_color.clone();
            if self.params.fade {
                col.a *= 1.0 - timer::duration_to_f64(Instant::now().duration_since(p.time_created)) as f32/timer::duration_to_f64(self.params.particle_lifetime) as f32;
            }

            tools::draw_circle(
                ctx,
                p.position,
                p.radius,
                col
            )?;
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
    fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
    }
}

pub struct ParticleSystemParam {
    pub base_color: Color,
    pub fade: bool,
    pub emission_period: Duration,
    pub particle_lifetime: Duration,
    pub particle_speed_minmax: (f32, f32),
    pub particle_radius_minmax: (f32, f32),
    pub stop_after: Option<Duration>,       // Duration to stop after if any.
}

impl ParticleSystemParam {
    // A few presets
    pub fn planet_trail() -> ParticleSystemParam {
        ParticleSystemParam {
            base_color: [0.1, 0.4, 0.8, 1.0].into(),
            fade: true,
            emission_period: Duration::from_millis(50),
            particle_lifetime: Duration::from_millis(700),
            particle_speed_minmax: (1.0, 10.0),
            particle_radius_minmax: (0.5, 2.0),
            stop_after: None,
        }
    }

    // pub fn debris_emitter() -> ParticleSystemParam {
    //     ParticleSystemParam {
    //         base_color: [0.8, 0.8, 0.8, 1.0].into(),
    //         fade: true,
    //         emission_period: Duration::from_millis(1),
    //         particle_lifetime: Duration::from_millis(700),
    //         particle_speed_minmax: (30.0, 100.0),
    //         particle_radius_minmax: (0.5, 2.0),
    //         stop_after: Some(Duration::from_millis(100)),
    //     }
    // }
}