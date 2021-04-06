#[macro_use]
extern crate derive_new;

#[macro_use]
pub mod body;
pub mod star;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use opengl_graphics::GlyphCache;

use nalgebra::{Point2, Vector2,};
use std::time::{Instant};


use body::Body;
use star::Star;


pub type Float = f64;   // Precision of floats

const GRAV_CONST: Float = 0.1; // 6.67408e-11;
const DT: Float = 1.0/60.0;



pub struct App {
    gl: GlGraphics,
    stars: Vec<Star>,
}

impl App {
    fn render(&mut self, args: &RenderArgs, text_glyph_cache: &mut GlyphCache, fps: f32) {
        use graphics::*;

        // Clear screen
        self.gl.draw(args.viewport(), |_c, gl| { clear([0.0; 4], gl) });
     
        for i in 0..self.stars.len() {
            let (x, y) = {
                let p = self.stars[i].position();
                (p[0] as f64, p[1] as f64)
            };

            let rad = self.stars[i].radius as f64;
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c
                    .transform
                    .trans(x, y);

                ellipse([1.0, 1.0, 1.0, 1.0], [rad * 2.0; 4], transform, gl);
            });
        }

        // Draw debug stuff
        let bodies_count = self.stars.len();

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(10.0, 20.0);

            text(
                [0.0, 1.0, 0.0, 1.0],
                12,
                &format!("{:.1} Bodies: {}", fps, bodies_count),
                text_glyph_cache,
                transform,
                gl,
            ).unwrap();
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        Self::apply_gravity(&mut self.stars);
        Self::update_bodies(&mut self.stars);
    }

    fn apply_gravity<B: Body>(bodies: &mut Vec<B>) {
        for i in 0..bodies.len()-1 {
            for j in i+1..bodies.len() {
                let force = bodies[i].newtonian_force(&bodies[j]);
                bodies[i].apply_force(&force, DT);
                bodies[j].apply_force(&(-force), DT);
            }
        }
    }

    fn update_bodies<B: Body>(bodies: &mut Vec<B>) {
        for b in bodies.iter_mut() {
            b.update_position(DT);
        }
    }

}


pub fn main() {
    let stars = vec![
        Star::new(
            Point2::new(100.0, 100.0),
            Vector2::new(10.0, 0.0),
            5000.0,
            10.0,
        ),
        Star::new(
            Point2::new(100.0, 150.0),
            Vector2::new(10.0, 0.0),
            5000.0,
            10.0,
        ),
    ];

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Orbits", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();


    let mut text_glyph_cache = graphics::glyph_cache::rusttype::GlyphCache::new("assets/UbuntuMono.ttf", (), opengl_graphics::TextureSettings::new()).unwrap();


    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        stars,
    };

    let mut events = Events::new(EventSettings::new());

    let mut last_frame_inst = Instant::now();
    let mut dt: f64 = 0.0;

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut text_glyph_cache, (1.0/dt) as f32);

            let curr_inst = Instant::now();
            dt = curr_inst.duration_since(last_frame_inst).as_secs_f64();
            last_frame_inst = curr_inst;
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}