use nalgebra::{Point2, Vector2};

use crate::{Float, GRAV_CONST};

pub trait Body {
    fn position(&self) -> &Point2<Float>;
    fn position_mut(&mut self) -> &mut Point2<Float>;
    fn velocity(&self) -> &Vector2<Float>;
    fn velocity_mut(&mut self) -> &mut Vector2<Float>;
    fn mass(&self) -> &Float;
    fn mass_mut(&mut self) -> &mut Float;

    fn newtonian_force(&self, other: &dyn Body) -> Vector2<Float> {
        // F = GmM/r^2
        // F_vec = (GmM/r^2) * r_norm
        // F_vec = (GmM/r^3) * r_vec
        let r = other.position() - self.position();
        let distance = r.norm();

        r * GRAV_CONST * ((*self.mass() * *other.mass()) / (distance * distance * distance))
    }

    fn apply_force(&mut self, force: &Vector2<Float>, dt: Float) {
        // F = dp/dt
        // dp = F dt
        // dp = m dv = F dt
        // dv = F dt/m
        let dv: Vector2<Float> = force * (dt / self.mass());
        *self.velocity_mut() += dv;
    }

    fn update_position(&mut self, dt: Float) {
        // v = dx/dt
        // v dt = dx
        let dr = self.velocity() * dt;
        *self.position_mut() += dr;
    }
}

// Macro for implementing simple functions for Body trait
macro_rules! default_body_gets {
    ($position:ident, $velocity:ident, $mass:ident) => {
        fn position(&self) -> &Point2<Float> { &self.$position }
        fn position_mut(&mut self) -> &mut Point2<Float> { &mut self.$position }
        fn velocity(&self) -> &Vector2<Float> { &self.$velocity }
        fn velocity_mut(&mut self) -> &mut Vector2<Float> { &mut self.$velocity }
        fn mass(&self) -> &Float { &self.$mass }
        fn mass_mut(&mut self) -> &mut Float { &mut self.$mass }
    };
}
