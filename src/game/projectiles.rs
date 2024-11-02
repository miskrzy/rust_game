use bevy::prelude::{Plugin, Update};

mod components;
pub mod constants;
mod systems;

use systems::{hit_target, movement, spawn};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, (spawn, movement, hit_target));
    }
}
