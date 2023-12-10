mod animation;
mod state_machine;

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

#[derive(Component, Deref, DerefMut)]
pub struct AnimationCoolDownTimer(Option<(Timer, PlayerState)>);

impl Default for AnimationCoolDownTimer {
    fn default() -> Self {
        Self(None)
    }
}

impl AnimationCoolDownTimer {
    pub fn new(timer: Timer, state: PlayerState) -> Self {
        Self(Some((timer, state)))
    }

    pub fn compare_state(&self, state: &PlayerState) -> bool {
        if let Some((_, s)) = self.0 {
            return s == *state;
        }
        false
    }
}

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
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, player_size, 8, 4, Some(Vec2::ONE), None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0., 0., 2.),
                ..default()
            },
            player,
            player_state,
            StateTriggerTimer(None),
            Velocity::default(), // This should be context
            Collider::Quad(player_size),
        ))
        .with_children(|parent| {
            parent.spawn(SpriteBundle {
                texture: asset_server.load("textures/player_left_hand.png"),
                transform: Transform::from_xyz(7.0, -6.0, 0.),
                ..default()
            });

            parent.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..default()
                },
                AnimationTimer::default(),
                player_state.get_animation(),
                AnimationCoolDownTimer(None),
            ));

            parent.spawn(SpriteBundle {
                texture: asset_server.load("textures/player_right_hand.png"),
                transform: Transform::from_xyz(-9.0, -6.0, 2.),
                ..default()
            });
        });
}

// TODO: try Res<PlayerState> instead of Query
pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut PlayerState, &mut Velocity)>,
    mut sprite_query: Query<&mut TextureAtlasSprite, With<AnimationIndices>>,
    time: Res<Time>,
) {
    let (player, mut player_state, mut player_velocity) = query.single_mut();
    let mut sprite = sprite_query.single_mut();
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
        let delta = time.delta_seconds();
        let move_x = move_towards(
            player_velocity.x.clone(),
            direction.x * player.max_speed,
            player.acceleration,
            delta,
        );
        // TODO: refactor
        if player_state.is(PlayerState::Moving) {
            player_velocity.x = move_x;
        } else if player_state.transition(PlayerEvent::Move) {
            player_velocity.x = move_x;
        }

        sprite.flip_x = direction.x < 0.;
    } else {
        // println!("direction.x is zero {:?}", time.delta_seconds());
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

pub fn push_player(mut query: Query<(&PlayerState, &Player, &mut Velocity)>, time: Res<Time>) {
    let (player_state, player, mut player_velocity) = query.single_mut();
    if player_state.is(PlayerState::Pushing) {
        /* player_velocity.x = move_towards(
            player_velocity.x,
            -player.max_speed * 2.,
            2.,
            time.delta_seconds(),
        );
        */
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
            // TODO: refactor
            sprite.index = match indices {
                AnimationIndices::Straight(anim) => {
                    if sprite.index == anim.last {
                        anim.repeat_from.unwrap_or(anim.first)
                    } else {
                        sprite.index + 1
                    }
                }
            };
        }
    }
}

pub fn animate_change(
    state_query: Query<&PlayerState, Changed<PlayerState>>,
    mut query: Query<(
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut AnimationCoolDownTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for state in &state_query {
        let (mut indices, mut timer, mut cool_down_timer, mut sprite) = query.single_mut();

        if cool_down_timer.compare_state(state) {
            cool_down_timer.0 = None;
            continue;
        }

        if let Some(cool_down) = indices.get_cool_down() {
            *cool_down_timer = AnimationCoolDownTimer::new(
                Timer::from_seconds(cool_down, TimerMode::Once),
                state.clone(),
            );
            continue;
        }

        update_animation(state, &mut indices, &mut timer, &mut sprite);
    }
}

pub fn animate_cool_down(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationCoolDownTimer,
        &mut AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
    state_query: Query<&PlayerState>,
) {
    for (mut cool_down_timer, mut indices, mut animation_timer, mut sprite) in &mut query {
        if let Some((timer, _)) = &mut cool_down_timer.0 {
            if timer.tick(time.delta()).just_finished() {
                cool_down_timer.0 = None;
                let state = state_query.single();
                update_animation(state, &mut indices, &mut animation_timer, &mut sprite);
            }
        }
    }
}

fn update_animation(
    state: &PlayerState,
    indices: &mut AnimationIndices,
    timer: &mut AnimationTimer,
    sprite: &mut TextureAtlasSprite,
) {
    *indices = state.get_animation();
    let only_borrow_indices = &*indices;
    match only_borrow_indices {
        AnimationIndices::Straight(anim) => {
            *timer = AnimationTimer::from_seconds(anim.rate);
            sprite.index = anim.first;
        }
    }
}
