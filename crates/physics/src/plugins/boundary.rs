use crate::plugins::particle::ParticleUpdatesSet;
use crate::prelude::*;

pub fn boundary_plugin(app: &mut App) {
    app.add_systems(Startup, spawn_boundary_box)
        .add_systems(
            Update,
            (
                handle_out_of_bounds.before(ParticleUpdatesSet),
                draw_boundary_box,
            ),
        )
        .init_resource::<BoundaryBoxDefaults>();
}

/* -- Update -- */
// Physical
fn handle_out_of_bounds(
    boundary_box_query: Single<&Transform, With<BoundaryBox>>,
    particle_query: Query<(&mut Transform, &mut Velocity), Without<BoundaryBox>>,
    defaults: Res<BoundaryBoxDefaults>,
    // collision_event: EventWriter<OutOfBoundsEvent>
) {
    let damping_factor = defaults.damping;
    let box_transform = boundary_box_query.into_inner();
    for (mut particle_transform, mut velocity) in particle_query {
        let (box_width, box_height) = (box_transform.scale.x, box_transform.scale.y);
        let box_center = box_transform.translation;

        let particle_radius = particle_transform.scale.x;
        let particle_position = &mut particle_transform.translation;
        let particle_velocity = &mut velocity.0;

        if particle_position.x.abs() + particle_radius > box_width / 2.0 {
            particle_position.x =
                (box_width / 2.0 - particle_radius) * particle_position.x.signum() + box_center.x;
            particle_velocity.x = -particle_velocity.x * damping_factor;
            particle_velocity.y *= damping_factor;
        }
        if particle_position.y.abs() + particle_radius > box_height / 2.0 {
            particle_position.y =
                (box_height / 2.0 - particle_radius) * particle_position.y.signum() + box_center.y;
            particle_velocity.x *= damping_factor;
            particle_velocity.y = -particle_velocity.y * damping_factor;
        }
    }
}

// Visual
fn draw_boundary_box(mut gizmos: Gizmos, defaults: Res<BoundaryBoxDefaults>) {
    gizmos.rect_2d(Isometry2d::from_translation(defaults.center.xy()), defaults.size, defaults.color);
}

/* -- Startup -- */
fn spawn_boundary_box(mut commands: Commands, defaults: Res<BoundaryBoxDefaults>) {
    let box_transform = Transform {
        translation: defaults.center,
        rotation: Quat::default(),
        scale: defaults.size.extend(0.0),
    };

    commands.spawn(BoundaryBoxBundle::new(box_transform));
}
