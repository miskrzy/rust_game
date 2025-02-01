use bevy::{
    app::Update,
    ecs::schedule::IntoSystemConfigs,
    prelude::{App, Plugin},
    state::{condition::in_state, state::OnExit},
};

mod components;
pub mod constants;
pub mod events;
pub mod systems;

use events::Explode;
use systems::{animate, despawn, despawn_on_finish, hit_targets, spawn, step_explosion};

use crate::states::AppState;

use super::states::GameState;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Explode>()
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
