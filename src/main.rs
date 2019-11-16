mod tools;
mod planet;

use ggez::event;
use ggez::graphics::{self, spritebatch::SpriteBatch, DrawParam};
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use ggez::timer;

use std::collections::HashMap;
use std::cell::RefCell;

use planet::Planet;

pub const G: f32 = 0.0001;    // Gravitational constant
pub const PLANET_IMAGE_DIMS: (u32, u32) = (256, 256);
const PLANET_DRAW_RATIO: (f32, f32) = (PLANET_IMAGE_DIMS.0 as f32/2.0, PLANET_IMAGE_DIMS.1 as f32/2.0);

struct MainState {
    planet_id_count: usize,
    planets: HashMap<usize, RefCell<Planet>>,
    planet_spritebatch: SpriteBatch,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let planet_sprite = graphics::Image::new(ctx, "/circle.png").unwrap();
        let planet_spritebatch = SpriteBatch::new(planet_sprite);

        let mut s = MainState {
            planet_id_count: 0,
            planets: HashMap::new(),
            planet_spritebatch,
        };

        s.add_planet(
            Point2::new(300.0, 400.0),
            None,
            None,
            50.0
        );

        s.add_planet(
            Point2::new(600.0, 400.0),
            None,
            None,
            50.0
        );

        Ok(s)
    }

    #[inline]
    fn add_planet(&mut self, position: Point2<f32>, velocity: Option<Vector2<f32>>, mass: Option<f32>, radius: f32) {
        self.planets.insert(
            self.planet_id_count,
            RefCell::new(Planet::new(
                self.planet_id_count,
                position,
                velocity,
                mass,
                radius
            ))
        );
        self.planet_id_count += 1;
    }

    #[inline]
    fn draw_planet(planet: &Planet, spritebatch: &mut SpriteBatch) {
        spritebatch.add(DrawParam::new()
            .dest(planet.position)
            .offset(Point2::new(0.5, 0.5))
            .scale(Vector2::new(planet.radius/PLANET_DRAW_RATIO.0 as f32, planet.radius/PLANET_DRAW_RATIO.1 as f32))
        );
    }

    #[inline]
    fn draw_fps(ctx: &mut Context) -> GameResult {
        let text = graphics::Text::new(
            format!("{:.3}", timer::fps(ctx))
        );
        graphics::draw(
            ctx,
            &text,
            DrawParam::new().dest([10.0, 10.0])
        )
    }

    fn collide_planets(new_id: usize, pl1: &Planet, pl2: &Planet) -> Planet {  // Returns new planet that is sum of other two.
        // Conservation of momentum
        let total_mass = pl1.mass + pl2.mass;
        let m_i = pl1.mass * pl1.velocity + pl2.mass * pl2.velocity;
        let total_volume = tools::volume_of_sphere(pl1.radius) + tools::volume_of_sphere(pl2.radius);
        let new_radius = tools::inverse_volume_of_sphere(total_volume);
        // Use centre of mass as new position
        let new_position = Point2::new(
            (pl1.position.x * pl1.mass + pl2.position.x * pl2.mass)/total_mass,
            (pl1.position.y * pl1.mass + pl2.position.y * pl2.mass)/total_mass
        );

        Planet::new(new_id, new_position, Some(m_i/total_mass), Some(total_mass), new_radius)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = timer::duration_to_f64(timer::average_delta(ctx)) as f32;

        let mut colliding_planets = Vec::with_capacity(self.planets.len());
        let mut new_planets = Vec::with_capacity(self.planets.len()/2);

        let keys: Vec<&usize> = self.planets.keys().collect();
        let len = self.planets.len();
        for i in 0..len-1 {
            if !colliding_planets.contains(keys[i]) {
                let pl1 = self.planets.get(keys[i]).expect("Couldn't get planet 1");
                for j in i+1..len {
                    if !colliding_planets.contains(keys[j]) {
                        let pl2 = self.planets.get(keys[j]).expect("Couldn't get planet 2");
                        let colliding = {
                            let bpl1 = pl1.borrow();
                            let bpl2 = pl2.borrow();
                            tools::check_collision(&bpl1.position, &bpl2.position, bpl1.radius, bpl2.radius)
                        };

                        if colliding {
                            println!("Is colliding: {}", colliding);
                            colliding_planets.push(*keys[i]);
                            colliding_planets.push(*keys[j]);

                            new_planets.push(Self::collide_planets(self.planet_id_count, &pl1.borrow(), &pl2.borrow()));
                            self.planet_id_count += 1;
                        } else {
                            tools::newtonian_grav(&mut pl1.borrow_mut(), &mut pl2.borrow_mut());
                        }
                    }
                }
            }
        }

        for (_, pl) in self.planets.iter() {
            pl.borrow_mut().update(dt);
        }

        // Remove collided planets
        self.planets.retain(|_, p| !colliding_planets.contains(&p.borrow().id));

        for new_planet in new_planets {
            self.planets.insert(new_planet.id, RefCell::new(new_planet));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.planet_spritebatch.clear();
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for (_, planet) in self.planets.iter() {
            Self::draw_planet(&planet.borrow(), &mut self.planet_spritebatch);
        }

        graphics::draw(ctx, &self.planet_spritebatch, DrawParam::default())?;

        Self::draw_fps(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    use std::path;
    use std::env;

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("Planets", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}