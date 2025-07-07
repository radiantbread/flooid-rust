use crate::prelude::*;

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct BoundaryBox;

#[derive(Bundle)]
pub struct ParticleBundle {
    pub transform: Transform,
    pub velocity: Velocity,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

#[derive(Bundle)]
pub struct BoundaryBoxBundle {
    marker: BoundaryBox,
    transform: Transform,
}

impl BoundaryBoxBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            marker: BoundaryBox,
            transform,
        }
    }
}
