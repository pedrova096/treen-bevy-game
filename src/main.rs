#![allow(clippy::type_complexity)]

use bevy::prelude::*;

mod menu;
use menu::*;

mod game;
use game::{common::*, player::*, wagon::*};

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
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
        .add_systems(Update, menu_sys.run_if(in_state(AppState::Menu)))
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(OnEnter(AppState::InGame), (setup_wagon, setup_player))
        .add_systems(Update, camera_follow)
        .add_systems(
            FixedUpdate,
            (
                move_player,
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
    commands.spawn(Camera2dBundle::default());
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
