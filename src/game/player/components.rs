use bevy::prelude::Component;
use bevy::time::Timer;
use std::time::Duration;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health {
    health: f32,
    max_health: f32,
    regen: f32,
}

impl Health {
    pub fn new(health: f32, regen: f32) -> Self {
        Self {
            health: health,
            max_health: health,
            regen: regen,
        }
    }

    pub fn current(&self) -> f32 {
        self.health
    }

    pub fn max(&self) -> f32 {
        self.max_health
    }

    pub fn deal_damage(&mut self, damage: f32) {
        self.health -= damage;
    }

    pub fn tick_regen(&mut self, time_delta: Duration) {
        if self.health < self.max_health {
            self.health += self.regen * time_delta.as_secs_f32();
        }
    }

    pub fn is_dead(&self) -> bool {
        if self.health <= 0. {
            true
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct CastTimer {
    pub timer: Timer,
}
