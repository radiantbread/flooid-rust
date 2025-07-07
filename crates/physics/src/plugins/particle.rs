use crate::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParticleUpdatesSet;

pub fn particle_plugin(app: &mut App) {
    app.init_resource::<ParticleSpawnDefaults>()
        .add_systems(Startup, spawn_particles)
        .add_systems(
            Update,
            (update_positions, handle_particle_collisions)
                .chain()
                .in_set(ParticleUpdatesSet),
        );
}

/* -- Update -- */
fn update_positions(time: Res<Time>, query: Query<(&Velocity, &mut Transform)>) {
    let delta_time = time.delta_secs();
    for (velocity, mut transform) in query {
        transform.translation.x += velocity.0.x * delta_time;
        transform.translation.y += velocity.0.y * delta_time;
    }
}

fn handle_particle_collisions(mut query: Query<(Entity, &mut Transform, &mut Velocity)>) {
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
        let dpos = pos1 - pos2;
        let normal = dpos.normalize();
        if dpos.length() < rad1 + rad2 {
            let overlap = (rad1 + rad2 - dpos.length()) * normal;
            transform1.translation += overlap / 2.0;
            transform2.translation -= overlap / 2.0;

            velocity1.0 = velocity1.0.reflect(normal.xy()).normalize() * velocity2.0.length();
            velocity2.0 = velocity2.0.reflect(normal.xy()).normalize() * velocity1.0.length();
        }
    }
}

/* -- Startup -- */
// Not a system, just a helper to make a grid.
fn grid_1d(width: f32, num_points: u32) -> Vec<f32> {
    let step = width / (num_points as f32 - 1.0);
    let half_width = (width - 2.*step) / 2.0;
    (0..num_points).map(|i| -half_width + (i as f32) * step).collect()
}

fn grid_2d(width: f32, height: f32, num_x: u32, num_y: u32) -> Vec<Vec<f32>> {
    let points_x = grid_1d(width, num_x);
    let points_y = grid_1d(height, num_y);
    
    points_y.iter()
        .flat_map(|&y| points_x.iter().map(move |&x| vec![x, y]))
        .collect()
}

fn spawn_particles(mut commands: Commands, defaults: Res<ParticleSpawnDefaults>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let grid = grid_2d(defaults.width, defaults.height, defaults.num_x, defaults.num_y);

    for point in grid {
        let (x, y) = (point[0], point[1]);
        commands.spawn((ParticleBundle{
            transform: Transform::from_translation(vec3(x, y, 0.) + defaults.center).with_scale(Vec3::splat(defaults.radius)),
            velocity: Velocity(vec2(0.0, 0.0)),
            mesh: Mesh2d(meshes.add(Circle::new(1.))),
            material: MeshMaterial2d(materials.add(ColorMaterial::from_color(defaults.color))),
        },
        Mass(10.0)));
    }
}