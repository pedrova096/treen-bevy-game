use bevy::{prelude::*, sprite::collide_aabb::Collision};

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
    let offset = 4.;
    let player_size = match collider {
        Collider::Quad(size) => *size + Vec2::new(offset, offset),
    };
    let player_translation = transform.translation + Vec3::new(offset / 2., offset / 2., 0.);
    for (other_transform, other_collider) in collider_query.iter() {
        let other_size = match other_collider {
            Collider::Quad(size) => *size,
        };
        // is colliding
        let collision = collide_v2(
            player_translation,
            player_size,
            other_transform.translation,
            other_size,
        );

        if let Some((collision, diff)) = collision {
            match collision {
                Collision::Left if velocity.x > 0. => velocity.x = 0.,
                Collision::Right if velocity.x < 0. => velocity.x = 0.,
                Collision::Top if velocity.y < 0. => {
                    velocity.y = 0.;
                    let offset = 0.1;
                    if diff < -offset - 0.01 {
                        println!("diff: {}", diff);
                        transform.translation.y -= diff + offset;
                    }
                }
                Collision::Bottom if velocity.y > 0. => velocity.y = 0.,
                _ => {}
            }

            collision_events.send(CollisionEvent);
        }
    }
}

pub fn collide_v2(
    a_pos: Vec3,
    a_size: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
) -> Option<(Collision, f32)> {
    let a_min = a_pos.truncate() - a_size / 2.0;
    let a_max = a_pos.truncate() + a_size / 2.0;

    let b_min = b_pos.truncate() - b_size / 2.0;
    let b_max = b_pos.truncate() + b_size / 2.0;

    // check to see if the two rectangles are intersecting
    if a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y {
        // check to see if we hit on the left or right side
        let (x_collision, x_depth) = if a_min.x < b_min.x && a_max.x > b_min.x && a_max.x < b_max.x
        {
            (Collision::Left, b_min.x - a_max.x)
        } else if a_min.x > b_min.x && a_min.x < b_max.x && a_max.x > b_max.x {
            (Collision::Right, a_min.x - b_max.x)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        // check to see if we hit on the top or bottom side
        let (y_collision, y_depth) = if a_min.y < b_min.y && a_max.y > b_min.y && a_max.y < b_max.y
        {
            (Collision::Bottom, b_min.y - a_max.y)
        } else if a_min.y > b_min.y && a_min.y < b_max.y && a_max.y > b_max.y {
            (Collision::Top, a_min.y - b_max.y)
        } else {
            (Collision::Inside, -f32::INFINITY)
        };

        // if we had an "x" and a "y" collision, pick the "primary" side using penetration depth
        if y_depth.abs() < x_depth.abs() {
            Some((y_collision, y_depth))
        } else {
            Some((x_collision, x_depth))
        }
    } else {
        None
    }
}
