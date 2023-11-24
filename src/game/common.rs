//! A simplified implementation of the classic game "Breakout".

use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vec2::ZERO)
    }
}
pub const GRAVITY: f32 = -9.8 * 10.0; // 100.0

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

pub fn apply_gravity(mut query: Query<&mut Velocity>) {
    for mut velocity in &mut query {
        velocity.y += GRAVITY;
    }
}
