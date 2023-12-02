use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::collision::Collider;
use bevy::render::render_resource::Texture;

#[derive(Clone, Copy)]
enum WagonsType {
    Head,
    Small,
    Medium,
    Large,
}
mod WagonTextures {
    use super::WagonsType;
    use bevy::math::Rect;

    type RectTuple = (f32, f32, f32, f32);

    pub const HEAD: RectTuple = (208., 64., 327., 127.);
    pub const SMALL: RectTuple = (128., 64., 200., 127.);
    pub const MEDIUM: RectTuple = (0., 64., 121., 127.);
    pub const BIG: RectTuple = (0., 0., 287., 64.);
    pub const WHEEL: RectTuple = (288., 0., 303., 15.);
    pub const UNION: RectTuple = (292., 23., 299., 59.); // TODO: rotate
    pub const TRACK: RectTuple = (304., 0., 318., 7.);

    pub fn get_vec() -> Vec<Rect> {
        vec![
            Rect::new(HEAD.0, HEAD.1, HEAD.2, HEAD.3),
            Rect::new(SMALL.0, SMALL.1, SMALL.2, SMALL.3),
            Rect::new(MEDIUM.0, MEDIUM.1, MEDIUM.2, MEDIUM.3),
            Rect::new(BIG.0, BIG.1, BIG.2, BIG.3),
            Rect::new(WHEEL.0, WHEEL.1, WHEEL.2, WHEEL.3),
            Rect::new(UNION.0, UNION.1, UNION.2, UNION.3),
            Rect::new(TRACK.0, TRACK.1, TRACK.2, TRACK.3),
        ]
    }

    pub fn get_wagon_rect(wagon_type: &WagonsType) -> Rect {
        match wagon_type {
            WagonsType::Head => Rect::new(HEAD.0, HEAD.1, HEAD.2, HEAD.3),
            WagonsType::Small => Rect::new(SMALL.0, SMALL.1, SMALL.2, SMALL.3),
            WagonsType::Medium => Rect::new(MEDIUM.0, MEDIUM.1, MEDIUM.2, MEDIUM.3),
            WagonsType::Large => Rect::new(BIG.0, BIG.1, BIG.2, BIG.3),
        }
    }

    pub fn get_wagon_index(wagon_type: &WagonsType) -> usize {
        match wagon_type {
            WagonsType::Head => 0,
            WagonsType::Small => 1,
            WagonsType::Medium => 2,
            WagonsType::Large => 3,
        }
    }

    pub fn get_wheel_index() -> usize {
        4
    }

    pub fn get_union_index() -> usize {
        5
    }
    pub fn get_track_index() -> usize {
        6
    }
}

const WAGON_SIZE: f32 = 230.;
const WAGON_SPACING: f32 = 100.;
impl WagonsType {
    fn size(&self) -> Vec2 {
        match self {
            WagonsType::Head | WagonsType::Medium => Vec2::new(WAGON_SIZE * 3., WAGON_SIZE),
            WagonsType::Small => Vec2::new(WAGON_SIZE * 2., WAGON_SIZE),
            WagonsType::Large => Vec2::new(WAGON_SIZE * 5., WAGON_SIZE),
        }
    }
    fn origin(&self) -> f32 {
        let size = &self.size();
        size.x / 2.
    }
}
const WAGONS: [WagonsType; 1] = [
    WagonsType::Head,
    // WagonsType::Medium,
    // WagonsType::Small,
    // WagonsType::Medium,
    // WagonsType::Large,
    // WagonsType::Medium,
];
const WAGON_COLOR: &str = "#4b5bab";
const WAGON_WHEEL_SIZE: f32 = 40.;
const WAGON_WHEEL_COLOR: &str = "#43434f";

trait TextureAtlasFromVec {
    fn from_vec(texture_handle: Handle<Image>, dimensions: Vec2, textures_vec: Vec<Rect>) -> Self;
}

impl TextureAtlasFromVec for TextureAtlas {
    fn from_vec(texture_handle: Handle<Image>, dimensions: Vec2, textures_vec: Vec<Rect>) -> Self {
        let mut texture_atlas = TextureAtlas::new_empty(texture_handle, dimensions);
        textures_vec.iter().for_each(|rect| {
            texture_atlas.add_texture(*rect);
        });
        texture_atlas
    }
}

pub fn setup_wagon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Query<&Window>,
) {
    let texture_handle = asset_server.load("textures/wagon.png");
    let texture_atlas = TextureAtlas::from_vec(
        texture_handle,
        Vec2::new(336., 128.),
        WagonTextures::get_vec(),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let windows_resolution = &windows.single().resolution;

    let mut x = -200.; // offset from the left of the screen
    for wagon in WAGONS.iter() {
        let wagon_index = WagonTextures::get_wagon_index(wagon);
        let wagon_rect: Rect = WagonTextures::get_wagon_rect(wagon);

        commands
            .spawn((SpatialBundle::default()))
            .with_children(|parent| {
                // TextureAtlas
                parent.spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(), // TODO: check
                        sprite: TextureAtlasSprite::new(wagon_index),
                        ..default()
                    },
                    Collider::Quad(wagon_rect.size()),
                ));
                parent.spawn(SpriteSheetBundle {
                    texture_atlas: *texture_atlas_handle, // TODO: check
                    sprite: TextureAtlasSprite::new(WagonTextures::get_wheel_index()),
                    transform: Transform::from_translation(Vec3::new(
                        -wagon_rect.size().x / 2.,
                        -wagon_rect.size().y / 2.,
                        1.,
                    )),
                    ..default()
                });
            });
        // commands.spawn((
        //     MaterialMesh2dBundle {
        //         mesh: meshes
        //             .add(shape::Quad::new(wagon_rect.size()).into())
        //             .into(),
        //         material: materials.add(ColorMaterial::from(Color::hex(WAGON_COLOR).unwrap())),
        //         transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        //         ..Default::default()
        //     },
        //     Collider::Quad(wagon.size()),
        // ));
        /*
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
                    transform: Transform::from_translation(Vec3::new(
                        x + wagon.origin(),
                        -windows_resolution.height() / 2. + size.y / 2. + WAGON_WHEEL_SIZE + 5.,
                        0.,
                    )),
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
             */

        x += WAGON_SPACING + wagon_rect.size().x;
    }
}
