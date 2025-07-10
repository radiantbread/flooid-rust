use bevy::reflect::Map;

use crate::prelude::*;
use std::collections::HashMap;

pub const BOX_CENTER: Vec3 = Vec3::ZERO;
pub const BOX_SIZE: Vec2 = vec2(1200.0, 800.0);
pub const BOX_DAMPING: f32 = 1.0;
pub const PARTICLE_RADIUS: f32 = 1.0;
pub const PARTICLE_NUM: UVec2 = uvec2(600, 30);

#[derive(Resource)]
pub struct Gravity {
    pub magnitude: f32,
    pub direction: f32,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            magnitude: 0.,
            direction: -PI / 2.,
        }
    }
}

#[derive(Resource)]
pub struct ParticleSpawnDefaults {
    pub radius: f32,
    pub center: Vec3,
    pub mass: Mass,
    pub width: f32,
    pub height: f32,
    pub num_x: u32,
    pub num_y: u32,
    pub color: Color,
}

impl Default for ParticleSpawnDefaults {
    fn default() -> Self {
        Self {
            radius: PARTICLE_RADIUS,
            center: vec3(0., 0., 0.),
            mass: Mass(10.),
            width: BOX_SIZE.x,
            height: BOX_SIZE.y,
            num_x: PARTICLE_NUM.x,
            num_y: PARTICLE_NUM.y,
            color: Color::linear_rgba(0.87, 0.85, 0.80, 1.0),
        }
    }
}

#[derive(Resource)]
pub struct BoundaryBoxDefaults {
    pub center: Vec3,
    pub size: Vec2,
    pub color: Color,
    pub damping: f32,
}

impl Default for BoundaryBoxDefaults {
    fn default() -> Self {
        Self {
            center: BOX_CENTER,
            size: BOX_SIZE,
            color: Color::linear_rgb(1.0, 0.15, 0.1),
            damping: BOX_DAMPING,
        }
    }
}

#[derive(Resource)]
pub struct EntityGrid2d {
    pub cells: HashMap<(i32, i32), Vec<Entity>>,
    pub size: f32,
}

impl Default for EntityGrid2d {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
            size: PARTICLE_RADIUS * 2.0,
        }
    }
}

impl EntityGrid2d {
    pub fn insert_point(&mut self, point: Vec2, entity: Entity) -> (i32, i32) {
        let coords = ((point.x / self.size + point.x.signum()) as i32, (point.y / self.size + point.y.signum()) as i32);
        match self.cells.get_mut(&coords) {
            Some(vector) => vector.push(entity),
            None => {
                let vector = vec![entity];
                self.cells.insert(coords, vector);
            }
        };
        coords
    }
    
    pub fn extract_entities(&mut self, cell: (i32, i32)) -> Vec<Entity> {
        let mut entities: Vec<Entity> = vec![];
        for x in -1..2 {
            for y in -1..2 {
                if let Some(vector) = self.cells.get(&(cell.0 + x, cell.1 + y)) {
                    entities.extend(vector);
                }
            }
        }
        self.cells.remove(&cell);
        entities
    }
    
    pub fn get_bounds_as_cells(&self, box_size: Vec2) -> (i32, i32) {
        ((box_size.x / self.size).round() as i32, (box_size.y / self.size).round() as i32)
    }
}