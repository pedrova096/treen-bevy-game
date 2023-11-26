mod state_machine;

use state_machine::PlayerState;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, utils::petgraph::matrix_graph::Zero};

use crate::collision::Collider;

use self::state_machine::PlayerEvent;

use super::{move_towards, Gravity, Velocity};

#[derive(Component)]
pub struct Player {
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
            max_speed: 10.,
            acceleration: 100.,
            time_jump_peak: 0.5,
            jump_height: 128.,
            jump_velocity: 0.,
        }
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gravity: ResMut<Gravity>,
) {
    let capsule = shape::Capsule {
        radius: 20.,
        depth: 50.,
        ..default()
    };

    let mut player = Player::default();
    gravity.0 = (2. * player.jump_height) / player.time_jump_peak.powi(2);
    player.jump_velocity = gravity.0 * player.time_jump_peak;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(capsule.into()).into(),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            transform: Transform::from_translation(Vec3::new(-100., 0., 1.)),
            ..default()
        },
        player,
        PlayerState::default(),
        StateTriggerTimer(None),
        Velocity::default(), // This should be context
        Collider::Quad(Vec2::new(
            capsule.depth - capsule.radius / 2.,
            capsule.depth + 2. * capsule.radius,
        )),
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

    println!("direction: {:?}", direction.x);
    // is direction.x NaN?
    if !direction.x.is_zero() {
        println!("moving??");
        if player_state.transition(PlayerEvent::Move) {
            println!("moving!!");
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
        player_state.transition(PlayerEvent::Jump);
        assert!(player_state.is(PlayerState::Jumping));
        player_velocity.y = -player.jump_velocity;
    }

    if player_state.can(PlayerEvent::Stop) && direction.length().is_zero() {
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
                println!("push player")
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
    println!("player state: {:?}", player_state);
    if player_state.is(PlayerState::Pushing) {
        println!("pushing");
        player_velocity.x = move_towards(player_velocity.x, -20., 25., time.delta_seconds());
    }
}

// jump logic
// d = vi * t + 1/2 * a * t^2
// vf = vi + a * t

// jumpHeight = 1/2 * a * timeToJumpPeak^2
// gravity = (2 * jumpHeight) / timeToJumpPeak^2
// jumpVelocity = gravity * timeToJumpPeak

// godot:
// var Velocity = Vector2()
// var InputVector = Vector2()
// var JumpAviable = true
// var JumpBufferPressed = false

// cont MAX_SPEED = 500
// const ACCELERATION = 3000
// const UP = Vector2(0, -1)
// const GRAVITY: float
// const JUMP_SPEED: float

// export(float) var TimeToJumpPeak = 0.5
// export(float) var JumpBufferTime = 0.1
// export(int) var JumpHeight = 128

// func _ready():
// 	GRAVITY = (2 * JumpHeight) / pow(TimeToJumpPeak, 2)
// 	JUMP_SPEED = GRAVITY * TimeToJumpPeak

// func _process(delta):
// 	if is_on_floor():
// 		JumpAviable = true
// 		if jumpBufferPressed == true:
// 			Jump()
// 	elif JumpAviable == true && CoyoteJumpTimer.is_stopped():
// 		CoyoteJumpTimer.start()
// ...
// Velocity.x = move_towards(Velocity.x, InputVector.x * MAX_SPEED, ACCELERATION * delta)

// jump():
// 	Velocity.y = -JUMP_SPEED
