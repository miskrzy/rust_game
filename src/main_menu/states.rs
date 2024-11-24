use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainMenuState {
    Home,
    Controls,
    None,
}
