use crate::prelude::*;

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct Mass(pub f32);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct BoundaryBox;

#[derive(Component)]
pub struct GridCell2d(pub (i32, i32));

#[derive(Bundle)]
pub struct ParticleBundle {
    particle: Particle,
    pub cell: GridCell2d,
    pub transform: Transform,
    pub velocity: Velocity,
    pub mesh: Mesh2d,
    pub material: MeshMaterial2d<ColorMaterial>,
}

impl ParticleBundle {
    pub fn new(
        transform: Transform,
        velocity: Velocity,
        mesh: Mesh2d,
        material: MeshMaterial2d<ColorMaterial>,
    ) -> Self {
        Self {
            particle: Particle,
            cell: GridCell2d((0, 0)),
            transform,
            velocity,
            mesh,
            material,
        }
    }
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
