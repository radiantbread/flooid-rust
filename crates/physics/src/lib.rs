mod components;
mod events;
mod plugins;
mod resources;

pub use plugins::physics_plugin;
pub mod prelude {
    use super::*;
    pub use bevy::prelude::*;
    pub use rayon::prelude::*;
    pub use std::f32::consts::PI;
    pub use {components::*, events::*, plugins::*, resources::*};
}
