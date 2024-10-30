use bevy::app::Update;
use bevy::{app::Startup, prelude::Plugin};

mod components;
mod constants;
pub mod resources;
mod systems;

use resources::Score;
use systems::{spawn, update_health_bar, update_score};

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Score { score: 0 })
            .add_systems(Startup, spawn)
            .add_systems(Update, (update_health_bar, update_score));
    }
}
