use super::{Collider, Velocity};
use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};

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
        Collider::Quad(Vec2::new(
            capsule.depth - capsule.radius / 2.,
            capsule.depth + 2. * capsule.radius,
        )),
        Velocity::default(),
    ));
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let mut player_velocity = query.single_mut();
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

    if direction.length() > 0. {
        direction = direction.normalize();
    }

    player_velocity.x = direction.x * 500.;
    player_velocity.y = direction.y * 500.;
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
