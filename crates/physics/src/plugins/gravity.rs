use crate::plugins::particle::ParticleUpdatesSet;
use crate::prelude::*;
use std::f32::consts::PI;

pub fn gravity_plugin(app: &mut App) {
    app.init_resource::<Gravity>()
        .add_systems(
            Update,
            (apply_gravity.after(ParticleUpdatesSet), rotate_gravity).chain(),
        )
        .add_systems(Update, draw_gravity_arrow);
}

/* -- Update -- */
// Physical
fn apply_gravity(time: Res<Time>, gravity: Res<Gravity>, query: Query<&mut Velocity, With<Mass>>) {
    let delta_time = time.delta_secs();
    for mut velocity in query {
        velocity.0 += Vec2::from_angle(gravity.direction) * gravity.magnitude * delta_time;
    }
}

fn rotate_gravity(time: Res<Time>, mut gravity: ResMut<Gravity>) {
    gravity.direction -= 2.0 * PI / 360.0 * time.delta_secs() * 0.0;
    if gravity.direction >= 2.0 * PI {
        gravity.direction = 0.0;
    }
}

// Visual
fn draw_gravity_arrow(mut gizmos: Gizmos, gravity: Res<Gravity>) {
    let color = Color::linear_rgb(1.0, 0.85, 0.2);
    gizmos.arrow_2d(
        Vec2::ZERO,
        Vec2::from_angle(gravity.direction) * 2.0 * gravity.magnitude,
        color,
    );
}
