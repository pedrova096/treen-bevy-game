mod state_machine;

use state_machine::PlayerState;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::collision::Collider;

use self::state_machine::PlayerEvent;

use super::{move_towards, Velocity};

#[derive(Component)]
pub struct Player;

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let capsule = shape::Capsule {
        radius: 20.,
        depth: 50.,
        ..default()
    };

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(capsule.into()).into(),
            material: materials.add(Color::rgb(1., 1., 1.).into()),
            transform: Transform::from_translation(Vec3::new(-100., 0., 1.)),
            ..default()
        },
        Player,
        PlayerState::default(),
        Velocity::default(), // This should be context
        Collider::Quad(Vec2::new(
            capsule.depth - capsule.radius / 2.,
            capsule.depth + 2. * capsule.radius,
        )),
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PlayerState, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    let (mut player_state, mut player_velocity) = query.single_mut();
    let mut direction = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::Left) {
        direction.x -= 1.;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        direction.x += 1.;
    }

    if direction.length() > 0. {
        direction = direction.normalize();

        if player_state.transition(PlayerEvent::Move) {
            let max_speed = 10.;
            let acceleration = 100.;
            let delta = time.delta_seconds();
            player_velocity.x = move_towards(
                player_velocity.x,
                direction.x * max_speed,
                acceleration,
                delta,
            );
        }
    } else {
        player_state.transition(PlayerEvent::Stop);
        player_velocity.x = 0.;
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
