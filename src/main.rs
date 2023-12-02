#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod menu;
use menu::*;

mod game;
use game::{common::*, player::*, train::*};

mod collision;
use collision::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Menu,
    #[default]
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .add_state::<AppState>()
        .add_event::<CollisionEvent>()
        .insert_resource(Gravity::default())
        .insert_resource(TrainForce::default())
        // 143, 222, 93 -> 0.56, 0.87, 0.36
        .insert_resource(ClearColor(Color::rgb(0.56, 0.87, 0.36)))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_sys.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(OnEnter(AppState::InGame), (setup_train, setup_player))
        .add_systems(Update, camera_follow)
        .add_systems(
            FixedUpdate,
            (
                move_player,
                push_player,
                player_state_trigger_timer,
                apply_gravity,
                check_for_collisions,
                apply_velocity,
            )
                // `chain`ing systems toether runs them in order
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.3;
    commands.spawn(camera);
}

fn camera_follow(
    players: Query<&Transform, With<Player>>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    // TODO: Make a smooth camera follow system
    let player_transform = players.single();

    let pos = player_transform.translation;

    for mut transform in &mut cameras {
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}
