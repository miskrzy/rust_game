use bevy::app::Update;
use bevy::{app::Startup, prelude::Plugin};

mod components;
mod constants;
mod systems;

use systems::spawn;
use systems::update_health_bar;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn)
            .add_systems(Update, update_health_bar);
    }
}
