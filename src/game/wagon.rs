use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::collision::Collider;

#[derive(Clone, Copy)]
enum WagonsType {
    Head,
    Small,
    Medium,
    Large,
}

const WAGON_SIZE: f32 = 250.;
const WAGON_SPACING: f32 = 20.;
impl WagonsType {
    fn size(&self) -> Vec2 {
        match self {
            WagonsType::Head | WagonsType::Medium => Vec2::new(WAGON_SIZE * 2.25, WAGON_SIZE),
            WagonsType::Small => Vec2::new(WAGON_SIZE * 1., WAGON_SIZE),
            WagonsType::Large => Vec2::new(WAGON_SIZE * 3., WAGON_SIZE),
        }
    }
    fn origin(&self) -> f32 {
        let size = &self.size();
        size.x / 2.
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
const WAGON_COLOR: &str = "#4b5bab";
const WAGON_WHEEL_SIZE: f32 = 40.;
const WAGON_WHEEL_COLOR: &str = "#43434f";

pub fn setup_wagon(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut x = 0.;
    for wagon in WAGONS.iter() {
        let size = wagon.size();

        let wheels_trans: [Transform; 2] = [
            Transform::from_translation(Vec3::new(
                -size.x / 2. + WAGON_WHEEL_SIZE + 10.,
                -size.y / 2.,
                1.,
            )),
            Transform::from_translation(Vec3::new(
                size.x / 2. - WAGON_WHEEL_SIZE - 10.,
                -size.y / 2.,
                1.,
            )),
        ];

        commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Quad::new(size).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::hex(WAGON_COLOR).unwrap())),
                    transform: Transform::from_translation(Vec3::new(x + wagon.origin(), 0., 0.)),
                    ..default()
                },
                Collider::Quad(size),
            ))
            .with_children(|parent| {
                for t in wheels_trans.iter() {
                    parent.spawn(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Circle::new(WAGON_WHEEL_SIZE).into())
                            .into(),
                        material: materials
                            .add(ColorMaterial::from(Color::hex(WAGON_WHEEL_COLOR).unwrap())),
                        transform: *t,
                        ..default()
                    });
                }
            });

        x += WAGON_SPACING + size.x;
    }
}
