use bevy::prelude::*;

use crate::collision::Collider;

#[derive(Clone, Copy, PartialEq)]
enum WagonsType {
    Head,
    Small,
    Medium,
    Large,
}

mod wagon {
    use super::WagonsType;

    type AssetInfo<'a> = (&'a str, f32, f32);

    pub const HEAD: AssetInfo = ("textures/train/wagon/head.png", 120., 63.);
    pub const SMALL: AssetInfo = ("textures/train/wagon/small.png", 73., 63.);
    pub const MEDIUM: AssetInfo = ("textures/train/wagon/medium.png", 122., 63.);
    pub const LARGE: AssetInfo = ("textures/train/wagon/large.png", 287., 63.);
    pub const WHEEL: AssetInfo = ("textures/train/wheel.png", 16., 16.);
    pub const UNION: AssetInfo = ("textures/train/union.png", 32., 8.);
    pub const TRACK: AssetInfo = ("textures/train/track.png", 16., 8.);

    pub fn get_asset_info(wagon_type: WagonsType) -> AssetInfo<'static> {
        match wagon_type {
            WagonsType::Head => HEAD,
            WagonsType::Small => SMALL,
            WagonsType::Medium => MEDIUM,
            WagonsType::Large => LARGE,
        }
    }

    pub fn get_wheel_position(
        wagon_type: WagonsType,
        wagon_width: f32,
        wagon_height: f32,
    ) -> Vec<(f32, f32)> {
        let wheel_width = WHEEL.1;
        let wheel_y = -wagon_height / 2. + 2.;
        match wagon_type {
            WagonsType::Head => vec![
                (-wagon_width / 2. + 20., wheel_y),
                (-wagon_width / 2. + wheel_width + 24., wheel_y),
                (wagon_width / 2. - 18., wheel_y),
            ],
            WagonsType::Medium => vec![
                (wagon_width / 2. - 22. - wheel_width, wheel_y),
                (wagon_width / 2. - 18., wheel_y),
            ],
            WagonsType::Small => vec![(wagon_width / 2. - 18., wheel_y)],
            WagonsType::Large => vec![
                (-wagon_width / 2. + 20., wheel_y),
                (-wagon_width / 2. + wheel_width + 24., wheel_y),
                (-10., wheel_y),
                (10., wheel_y),
                (wagon_width / 2. - 22. - wheel_width, wheel_y),
                (wagon_width / 2. - 18., wheel_y),
            ],
            _ => vec![],
        }
    }
}

const WAGONS: [WagonsType; 6] = [
    WagonsType::Head,
    WagonsType::Medium,
    WagonsType::Small,
    WagonsType::Medium,
    WagonsType::Large,
    WagonsType::Medium,
];

pub fn setup_train(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    let mut x = -200.;

    let wheel_handle = asset_server.load(wagon::WHEEL.0);

    let windows_resolution = &windows.single().resolution;

    let train_entity = commands.spawn(SpatialBundle::default()).id();

    for wagon in WAGONS.iter() {
        let (texture, width, height) = wagon::get_asset_info(*wagon);
        let origin_x = width / 2.;
        let wagon_entity = commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(
                        x + origin_x,
                        (-windows_resolution.height() * 0.3 + height) / 2. + 12., // TODO: 0.3 is from camera projection scale
                        0.,
                    ),
                    ..default()
                },
                Collider::Quad(Vec2::new(width, height)),
            ))
            .with_children(|parent| {
                // wagon
                parent.spawn(SpriteBundle {
                    texture: asset_server.load(texture),
                    transform: Transform::from_xyz(0., 0., 1.),
                    ..default()
                });

                for (x, y) in wagon::get_wheel_position(*wagon, width, height) {
                    parent.spawn(SpriteBundle {
                        texture: wheel_handle.clone(),
                        transform: Transform::from_xyz(x, y, 0.),
                        ..default()
                    });
                }

                if *wagon != WagonsType::Head {
                    let (union_texture, union_width, _) = wagon::UNION;
                    parent.spawn(SpriteBundle {
                        texture: asset_server.load(union_texture),
                        transform: Transform::from_xyz(
                            (-width - union_width) / 2.,
                            -height / 2. + 4.,
                            0.,
                        ),
                        ..default()
                    });
                }
            })
            .id();

        commands.entity(train_entity).push_children(&[wagon_entity]);

        x += width + wagon::UNION.1;
    }
}
