use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use super::{Player, Velocity};

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Component)]
pub enum Collider {
    Quad(Vec2),
}

pub fn check_for_collisions(
    mut player_query: Query<(&mut Velocity, &mut Transform, &Collider), With<Player>>,
    collider_query: Query<(&Transform, &Collider), (With<Collider>, Without<Player>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut velocity, mut transform, collider) = player_query.single_mut();
    // offset/scaled value, not actual size
    let offset = 2.;
    let player_size = match collider {
        Collider::Quad(size) => *size + Vec2::new(offset, offset),
    };
    let player_translation = transform.translation + Vec3::new(offset / 2., offset / 2., 0.);
    for (other_transform, other_collider) in collider_query.iter() {
        let other_size = match other_collider {
            Collider::Quad(size) => *size,
        };
        // is colliding
        let collision = collide(
            player_translation,
            player_size,
            other_transform.translation,
            other_size,
        );

        if let Some(collision) = collision {
            match collision {
                Collision::Left if velocity.x > 0. => velocity.x = 0.,
                Collision::Right if velocity.x < 0. => velocity.x = 0.,
                Collision::Top if velocity.y < 0. => velocity.y = 0.,
                Collision::Bottom if velocity.y > 0. => velocity.y = 0.,
                _ => {}
            }

            collision_events.send(CollisionEvent);
        }

        let player_position = player_translation + Vec3::new(velocity.x, velocity.y, 0.);

        // will collide
        let will_collide: Option<Collision> = collide(
            player_position,
            player_size,
            other_transform.translation,
            other_size,
        );

        if let Some(collision) = will_collide {
            match collision {
                Collision::Top => {
                    let diff = player_position.y
                        - other_transform.translation.y
                        - other_size.y / 2.
                        - player_size.y / 2.;
                    println!("diff: {}", diff);
                    if diff.abs() > 3. {
                        velocity.y = -diff;
                    }
                }
                _ => {}
            }

            collision_events.send(CollisionEvent);
        }
    }
}
