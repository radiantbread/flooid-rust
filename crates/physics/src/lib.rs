mod components;
mod plugins;
mod resources;

pub use plugins::physics_plugin;
pub mod prelude {
    use super::*;
    pub use bevy::prelude::*;
    pub use {components::*, plugins::*, resources::*};
}
