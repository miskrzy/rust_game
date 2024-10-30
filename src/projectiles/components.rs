use bevy::{math::Vec3, prelude::Component};

#[derive(Component)]
pub struct Projectile {
    target: Vec3,
    speed: f32,
}

impl Projectile {
    pub fn init_with_target(target: Vec3, speed: f32) -> Self {
        Self {
            target: target,
            speed: speed,
        }
    }

    pub fn new_position(&self, current_position: Vec3, time_delta_in_seconds: f32) -> Vec3 {
        current_position.move_towards(self.target, self.speed * time_delta_in_seconds)
    }

    pub fn is_finished(&self, current_position: Vec3) -> bool {
        current_position == self.target
    }
}
