use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::default())
    }
}
impl AnimationTimer {
    pub fn from_seconds(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Repeating))
    }
}

pub struct AnimationStraight {
    pub rate: f32,
    pub first: usize,
    pub last: usize,
    pub repeat_from: Option<usize>,
    pub cool_down: Option<f32>,
}

impl Default for AnimationStraight {
    fn default() -> Self {
        Self {
            rate: 0.1,
            first: 0,
            last: 0,
            repeat_from: None,
            cool_down: None,
        }
    }
}

#[derive(Component)]
pub enum AnimationIndices {
    Straight(AnimationStraight),
}

pub trait AnimationState {
    fn get_animation(&self) -> AnimationIndices;
}

impl AnimationIndices {
    pub fn get_cool_down(&self) -> Option<f32> {
        match self {
            Self::Straight(anim) => anim.cool_down,
        }
    }
}
