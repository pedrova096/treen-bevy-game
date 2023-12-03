mod animation;
mod state_machine;

use std::time::Duration;

use state_machine::PlayerState;

use bevy::{prelude::*, utils::petgraph::matrix_graph::Zero};

use crate::collision::Collider;

use self::{
    animation::{AnimationIndices, AnimationState, AnimationTimer},
    state_machine::PlayerEvent,
};

use super::{move_towards, Gravity, Velocity};

#[derive(Component, Reflect)]
pub struct Player {
    #[reflect_value]
    pub max_speed: f32,
    pub acceleration: f32,
    pub time_jump_peak: f32,
    pub jump_height: f32,
    pub jump_velocity: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct StateTriggerTimer(Option<Timer>);

impl Default for Player {
    fn default() -> Self {
        Self {
            max_speed: 2.5,
            acceleration: 50.,
            time_jump_peak: 0.3,
            jump_height: 4.0,
            jump_velocity: 0.,
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut gravity: ResMut<Gravity>,
) {
    let player_size = Vec2::new(22.0, 26.0);

    let player_state = PlayerState::default();
    let mut player = Player::default();
    gravity.0 = (2. * player.jump_height) / player.time_jump_peak.powi(2);
    player.jump_velocity = gravity.0 * player.time_jump_peak;

    let texture_handle = asset_server.load("textures/player.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, player_size, 8, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(0),
            ..default()
        },
        player,
        player_state,
        player_state.get_animation(),
        AnimationTimer::default(),
        StateTriggerTimer(None),
        Velocity::default(), // This should be context
        Collider::Quad(player_size),
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut PlayerState, &mut Velocity)>,
    time: Res<Time>,
) {
    let (player, mut player_state, mut player_velocity) = query.single_mut();
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        direction.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        direction.y -= 1.;
    }

    if !direction.x.is_zero() {
        if player_state.transition(PlayerEvent::Move) {
            let delta = time.delta_seconds();
            player_velocity.x = move_towards(
                player_velocity.x.clone(),
                direction.x * player.max_speed,
                player.acceleration,
                delta,
            );
        }
    }

    if direction.y < 0. {
        player_state.transition(PlayerEvent::Pull);
    } else if direction.y > 0. {
        if player_state.can(PlayerEvent::Jump) && player_state.transition(PlayerEvent::Jump) {
            player_velocity.y = player.jump_velocity;
        }
    }

    if direction.length().is_zero() && player_state.can(PlayerEvent::Stop) {
        player_state.transition(PlayerEvent::Stop);
        player_velocity.x = 0.;
    }
}

pub fn player_state_trigger_timer(
    mut query: Query<(&mut PlayerState, &mut StateTriggerTimer)>,
    time: Res<Time>,
) {
    let (mut player_state, mut stt) = query.single_mut();
    // destructuring the tuple
    let timer = stt.0.as_mut();

    match (*player_state, timer) {
        (PlayerState::Idle, None) => {
            stt.0 = Some(Timer::from_seconds(2.5, TimerMode::Once));
        }
        (PlayerState::Idle, Some(t)) => {
            if t.tick(time.delta()).just_finished() {
                player_state.transition(PlayerEvent::Push);
                stt.0 = None;
            }
        }
        (_, Some(_)) => {
            stt.0 = None;
        }
        _ => {}
    }
}

pub fn push_player(mut query: Query<(&PlayerState, &mut Velocity)>, time: Res<Time>) {
    let (player_state, mut player_velocity) = query.single_mut();
    if player_state.is(PlayerState::Pushing) {
        player_velocity.x = move_towards(player_velocity.x, -20., 25., time.delta_seconds());
    }
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        if timer.tick(time.delta()).just_finished() {
            sprite.index = match indices {
                AnimationIndices::Straight(anim) => {
                    if sprite.index == anim.last {
                        anim.repeat_from.unwrap_or(anim.first)
                    } else {
                        sprite.index + 1
                    }
                }
            };

            println!("Sprite index: {}", sprite.index);
        }
    }
}

pub fn animate_change(
    mut query: Query<
        (
            &mut AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &PlayerState,
        ),
        Changed<PlayerState>,
    >,
) {
    for (mut indices, mut timer, mut sprite, state) in &mut query {
        *indices = state.get_animation();
        let only_borrow_indices = &*indices;
        match only_borrow_indices {
            AnimationIndices::Straight(anim) => {
                *timer = AnimationTimer::from_seconds(anim.rate);
                sprite.index = anim.first;
            }
        }
    }
}
