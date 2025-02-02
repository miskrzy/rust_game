use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}
