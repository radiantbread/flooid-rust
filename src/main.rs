
use bevy::prelude::*;

use physics::physics_plugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(physics_plugin)
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}