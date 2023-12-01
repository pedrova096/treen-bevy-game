//! A simplified implementation of the classic game "Breakout".

use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}
#[derive(Debug, Resource)]
pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(0.)
    }
}

// train force Resource
#[derive(Debug, Resource)]
pub struct TrainForce {
    pub force: f32,
    pub acceleration: f32,
}

impl Default for TrainForce {
    fn default() -> Self {
        Self {
            force: 0.,
            acceleration: 0.,
        }
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

pub fn apply_gravity(mut query: Query<&mut Velocity>, gravity: Res<Gravity>, time: Res<Time>) {
    for mut velocity in &mut query {
        velocity.y -= gravity.0 * time.delta_seconds();
    }
}

pub fn move_towards(current: f32, target: f32, acceleration: f32, delta: f32) -> f32 {
    if current < target {
        (current + acceleration * delta).min(target)
    } else {
        (current - acceleration * delta).max(target)
    }
}
