use crate::prelude::*;

use rand::Rng;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParticleUpdatesSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParticleStartupSet;

pub fn particle_plugin(app: &mut App) {
    app.init_resource::<ParticleSpawnDefaults>()
        .add_systems(Startup, spawn_particles.in_set(ParticleStartupSet))
        .add_systems(
            Update,
            (update_positions).chain().in_set(ParticleUpdatesSet),
        )
        .add_event::<CollisionEvent>();
}

/* -- Update -- */
fn update_positions(time: Res<Time>, query: Query<(&Velocity, &mut Transform)>) {
    let delta_time = time.delta_secs();
    for (velocity, mut transform) in query {
        transform.translation.x += velocity.0.x * delta_time;
        transform.translation.y += velocity.0.y * delta_time;
    }
}

fn handle_particle_collisions(
    mut query: Query<(Entity, &mut Transform, &mut Velocity)>,
    _event: EventWriter<CollisionEvent>,
) {
    // Have to specify the generic const parameter K = 2, likely means the number of elements in a combination.
    let mut combinations = query.iter_combinations_mut::<2>();
    while let Some(
        [(entity1, mut transform1, mut velocity1), (entity2, mut transform2, mut velocity2)],
    ) = combinations.fetch_next()
    {
        if entity1 == entity2 {
            continue;
        };
        let (pos1, pos2) = (transform1.translation, transform2.translation);
        let (rad1, rad2) = (transform1.scale.x, transform2.scale.x);
        let dpos = (pos1 - pos2).xy();
        if dpos.length() < rad1 + rad2 {
            let normal = dpos.normalize();
            let overlap = (rad1 + rad2 - dpos.length()) * normal;
            transform1.translation += overlap.extend(0.0) / 2.0;
            transform2.translation -= overlap.extend(0.0) / 2.0;
            
            // Normal components of velocities with respect to the collision.
            let v1_n = velocity1.0.dot(normal)*normal;
            let v2_n = velocity2.0.dot(normal)*normal;
            let v1_t = velocity1.0 - v1_n;
            let v2_t = velocity2.0 - v2_n;

            velocity1.0 = v1_t + v2_n;
            velocity2.0 = v2_t + v1_n;
        }
    }
}

/* -- Startup -- */
// Not a system, just a helper to make a grid.
fn grid_1d(width: f32, num_points: u32) -> Vec<f32> {
    let step = width / (num_points as f32 + 1.0);
    let half_width = width / 2.0;
    (0..num_points)
        .map(|i| -(half_width - step) + (i as f32) * step)
        .collect()
}

fn grid_2d(width: f32, height: f32, num_x: u32, num_y: u32) -> Vec<Vec<f32>> {
    let points_x = grid_1d(width, num_x);
    let points_y = grid_1d(height, num_y);

    points_y
        .iter()
        .flat_map(|&y| points_x.iter().map(move |&x| vec![x, y]))
        .collect()
}

fn spawn_particles(
    mut commands: Commands,
    defaults: Res<ParticleSpawnDefaults>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let grid = grid_2d(
        defaults.width,
        defaults.height,
        defaults.num_x,
        defaults.num_y,
    );
    let mut rng = rand::rng();

    for point in grid {
        let (x, y) = (point[0], point[1]);
        commands.spawn((
            ParticleBundle::new(
                Transform::from_translation(vec3(x, y, 0.) + defaults.center)
                    .with_scale(Vec3::splat(defaults.radius)),
                Velocity(Vec2::from_angle(rng.random::<f32>() * 2.0 * PI) * 100.0),
                Mesh2d(meshes.add(Circle::new(1.))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(defaults.color))),
            ),
            Mass(10.0),
        ));
    }
}
