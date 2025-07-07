use crate::prelude::*;
use std::f32::consts::PI;

#[derive(Resource)]
pub struct Gravity {
    pub magnitude: f32,
    pub direction: f32,
}

impl Default for Gravity {
    fn default() -> Self {
        Self {
            magnitude: 30.,
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
            radius: 2.,
            center: vec3(0., 0., 0.),
            mass: Mass(10.),
            width: 1000.,
            height: 600.,
            num_x: 30,
            num_y: 18,
            color: Color::linear_rgb(0.87, 0.85, 0.80),
        }
    }
}
