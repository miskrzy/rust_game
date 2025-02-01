use bevy::{
    prelude::Component,
    time::{Timer, TimerMode},
};
use std::time::Duration;

#[derive(Clone, Component)]
pub struct Explosion {
    timer: Timer,
    current_frame: usize,
    total_frames: usize,
    current_repetition: u8,
    total_repetition: u8,
}

impl Explosion {
    pub fn new(duration: f32, frames: usize, repeat: u8) -> Self {
        let timer = Timer::from_seconds(duration / frames as f32, TimerMode::Repeating);
        Self {
            timer: timer,
            current_frame: 1,
            total_frames: frames,
            current_repetition: 1,
            total_repetition: repeat,
        }
    }

    pub fn step(&mut self, time_delta: Duration) {
        if self.timer.finished() {
            if self.current_frame < self.total_frames {
                self.current_frame += 1;
            } else if self.current_repetition < self.total_repetition {
                self.current_frame = 1;
                self.current_repetition += 1;
            }
        }
        self.timer.tick(time_delta);
    }

    pub fn current_frame(&self) -> usize {
        self.current_frame - 1
    }

    pub fn hit(&self) -> bool {
        self.timer.finished()
    }

    pub fn finished(&self) -> bool {
        self.timer.finished()
            && self.current_repetition == self.total_repetition
            && self.current_frame == self.total_frames
    }
}
