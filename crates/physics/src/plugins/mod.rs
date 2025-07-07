use crate::prelude::*;

mod boundary;
mod gravity;
mod particle;

pub fn physics_plugin(app: &mut App) {
    app.add_plugins((
        particle::particle_plugin,
        boundary::boundary_plugin,
        gravity::gravity_plugin,
    ));
}
