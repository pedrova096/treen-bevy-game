use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

use super::{Player, Velocity};

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Component)]
pub enum Collider {
    Quad(Vec2),
}

pub fn check_for_collisions(
    mut player_query: Query<(&mut Velocity, &Transform, &Collider), With<Player>>,
    collider_query: Query<(&Transform, &Collider), (With<Collider>, Without<Player>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut velocity, transform, collider) = player_query.single_mut();
    // force to get the size of quad from collider
    let size = match collider {
        Collider::Quad(size) => *size,
    };
    for (other_transform, other_collider) in collider_query.iter() {
        let other_size = match other_collider {
            Collider::Quad(size) => *size,
        };
        let collision = collide(
            transform.translation,
            size,
            other_transform.translation,
            other_size,
        );

        if let Some(collision) = collision {
            //commands.entity(entity).despawn();
            //commands.entity(other_entity).despawn();

            match collision {
                Collision::Left if velocity.x > 0. => velocity.x = 0.,
                Collision::Right if velocity.x < 0. => velocity.x = 0.,
                Collision::Top if velocity.y < 0. => velocity.y = 0.,
                Collision::Bottom if velocity.y > 0. => velocity.y = 0.,
                _ => {}
            }
            collision_events.send(CollisionEvent);
        }
    }
}
