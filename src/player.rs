use bevy::{
    app::{Startup, Update},
    prelude::{IntoSystemConfigs, Plugin},
};

pub mod components;
pub mod constants;
mod systems;

use systems::{despawn, movement, regen, restrict_movement, spawn};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, (movement, restrict_movement).chain())
            .add_systems(Update, (despawn, regen));
    }
}
