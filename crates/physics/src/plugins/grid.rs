use itertools::Itertools;

use crate::{plugins::particle::ParticleUpdatesSet, prelude::*};

pub fn grid_plugin(app: &mut App) {
    app.init_resource::<EntityGrid2d>()
        .add_systems(Update, (fill_grid, collide_in_grid, clear_grid).chain().after(ParticleUpdatesSet));
}

fn fill_grid(
    mut grid: ResMut<EntityGrid2d>,
    query: Query<(Entity, &GlobalTransform, &mut GridCell2d), With<Particle>>,
    mut gizmos: Gizmos,
) {
    for (entity, gl_transform, mut cell) in query {
        cell.0 = grid.insert_point(gl_transform.translation().xy(), entity);
    }
    
    /*
    for (x, y) in grid.cells.keys() {
        let (x, y) = (*x as f32, *y as f32);
        // gizmos.cross_2d(Isometry2d::from_xy(x as f32*grid.size, y as f32*grid.size), 5.0, Color::linear_rgb(0.3, 0.9, 0.1));
        gizmos.rect_2d(Isometry2d::from_xy((x - 0.5*x.signum())*grid.size, (y - 0.5*y.signum())*grid.size), Vec2::splat(grid.size), Color::linear_rgba(0.4, 0.9, 0.2, 0.9));
    }
    */
}

fn enforce_bounds_in_grid(
    grid: Res<EntityGrid2d>,
    particle_query: Query<(&mut Transform, &mut Velocity), With<Particle>>,
    boundary_query: Single<&Transform, (With<BoundaryBox>, Without<Particle>)>,
    defaults: Res<BoundaryBoxDefaults>,
) {
    let damping_factor = defaults.damping;
    // TODO first: recreate bound box to always have the dimensionality to fit the grid. Then boundary checks become even easier!
    
}

fn collide_in_grid(
    mut grid: ResMut<EntityGrid2d>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Particle>>,
) {
    for cells in grid.cells.keys() {
        let mut all_entities: Vec<Entity> = vec![];
        for x in -1..2 {
            for y in -1..2 {
                if let Some(vector) = grid.cells.get(&(cells.0 + x, cells.1 + y)) && (x, y) != (0, 0) {
                    all_entities.extend(vector);
                }
            }
        }
        
        let all_entities_cartesian = all_entities.iter().cartesian_product(&all_entities);

        for (entity1, entity2) in all_entities_cartesian {
            if let Ok([
                (mut transform1, mut velocity1), 
                (mut transform2, mut velocity2)
            ]) = query.get_many_mut([*entity1, *entity2]) {
                if entity1 == entity2 { continue };

                let (pos1, pos2) = (transform1.translation, transform2.translation);
                let (rad1, rad2) = (transform1.scale.x, transform2.scale.x);
                let dpos = (pos1 - pos2).xy();
                if dpos.length() < rad1 + rad2 {
                    let normal = dpos.normalize();
                    let overlap = (rad1 + rad2 - dpos.length()) * normal;
                    transform1.translation += overlap.extend(0.0) / 2.0;
                    transform2.translation -= overlap.extend(0.0) / 2.0;
                    
                    // Normal components of the velocities with respect to collision.
                    let v1_n = velocity1.0.dot(normal)*normal;
                    let v2_n = velocity2.0.dot(normal)*normal;
                    // Tangential components.
                    let v1_t = velocity1.0 - v1_n;
                    let v2_t = velocity2.0 - v2_n;

                    velocity1.0 = v1_t + v2_n;
                    velocity2.0 = v2_t + v1_n;
                }
            }
        }
    }
}

fn clear_grid(mut grid: ResMut<EntityGrid2d>) { grid.cells.clear() }