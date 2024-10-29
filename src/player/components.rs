use bevy::prelude::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Health {
    health: f32,
    max_health: f32,
}

impl Health {
    pub fn initialize(health: f32) -> Self {
        Self {
            health: health,
            max_health: health,
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

    pub fn is_dead(&self) -> bool {
        if self.health <= 0. {
            true
        } else {
            false
        }
    }
}
