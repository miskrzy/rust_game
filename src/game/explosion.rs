use bevy::{
    app::Update,
    prelude::{App, IntoScheduleConfigs, Plugin, Startup},
    state::{condition::in_state, state::OnExit},
};

mod components;
pub mod constants;
pub mod events;
mod resources;
pub mod systems;

use events::Explode;
use resources::AssetHandles;
use systems::{animate, despawn, despawn_on_finish, hit_targets, spawn, startup, step_explosion};

use crate::states::AppState;

use super::states::GameState;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Explode>()
            .init_resource::<AssetHandles>()
            .add_systems(Startup, startup)
            .add_systems(OnExit(AppState::Game), despawn)
            .add_systems(
                Update,
                (
                    spawn,
                    step_explosion,
                    animate,
                    despawn_on_finish,
                    hit_targets,
                )
                    .run_if(in_state(GameState::Play)),
            );
    }
}
