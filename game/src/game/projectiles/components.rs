use super::constants::DURATION;
use bevy::{
    math::Vec3,
    prelude::Component,
    time::{Timer, TimerMode},
};
use std::time::Duration;

#[derive(Component)]
pub struct Projectile {
    direction: Vec3,
    speed: f32,
    timer: Timer,
}

impl Projectile {
    pub fn new(current: Vec3, target: Vec3, speed: f32) -> Self {
        let direction = (target - current).normalize_or_zero();
        let timer = Timer::from_seconds(DURATION, TimerMode::Once);
        Self {
            direction: direction,
            speed: speed,
            timer: timer,
        }
    }

    pub fn step(&mut self, time_delta: Duration) -> Vec3 {
        self.timer.tick(time_delta);
        self.direction * self.speed * time_delta.as_secs_f32()
    }

    pub fn is_finished(&self) -> bool {
        self.timer.finished()
    }
}
