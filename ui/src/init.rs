use crate::components::{
    GameEntity, Piece, PieceBundle, PieceStatus, PieceType, Position, SelectionText, Side, Square,
};
use crate::resources::Board;
use bevy::asset::{AssetServer, Assets};
use bevy::hierarchy::BuildChildren;
use bevy::math::Vec3;
use bevy::prelude::{
    default, shape, Camera2dBundle, Color, ColorMaterial, Commands, Component, DespawnRecursiveExt,
    Entity, Mesh, NodeBundle, Query, Res, ResMut, SpriteBundle, TextBundle, Transform, With,
};
use bevy::render::view::RenderLayers;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::ui::{AlignContent, AlignSelf, JustifyContent, PositionType, UiRect, Val};
use bevy_mod_picking::events::{Click, Pointer};
use bevy_mod_picking::prelude::On;
use bevy_mod_picking::PickableBundle;
use gtc::position::Normalizable;

pub fn create_piece(piece: Piece, side: Side, pos: gtc::position::Position) -> PieceBundle {
    let x_pos: f32 = (pos.normal() % 8) as f32 * BOARD_SCALE as f32;
    let y_pos: f32 = (pos.normal() / 8 * 7) as f32 * BOARD_SCALE as f32;
    let mut bundle = PieceBundle {
        side,
        piece,
        position: Position {
            x: pos.0 as usize,
            y: pos.1 as usize,
        },
        sprite: SpriteBundle { ..default() },
    };
    bundle.sprite.transform = Transform::from_xyz(
        x_pos - (BOARD_SCALE as f32 * 8. / 2.) + (BOARD_SCALE as f32 / 2.),
        y_pos - (BOARD_SCALE as f32 * 8. / 2.) + (BOARD_SCALE as f32 / 2.),
        1.,
    );

    bundle
}
pub fn create_pieces(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..16 {
        let mut bun = PieceBundle {
            side: Side::Orange,
            piece: Piece(PieceType::None, PieceStatus::Both),
            position: Position {
                x: (i % 8) + 1,
                y: (i / 8 * 7) + 1,
            },
            sprite: SpriteBundle {
                ..Default::default()
            },
        };
        if i / 8 == 0 {
            bun.side = Side::White;
        }
        match i % 8 {
            0 => {
                bun.piece = Piece(PieceType::Goat, PieceStatus::Passive);
                bun.sprite.texture = asset_server.load("goat.png")
            }
            1 => {
                bun.piece = Piece(PieceType::Horse, PieceStatus::Passive);
                bun.sprite.texture = asset_server.load("horse.png")
            }
            2 => {
                bun.piece = Piece(PieceType::Tiger, PieceStatus::Aggressive);
                bun.sprite.texture = asset_server.load("tiger.png")
            }
            3 => {
                bun.piece = Piece(PieceType::Otter, PieceStatus::Aggressive);
                bun.sprite.texture = asset_server.load("otter.png")
            }
            4 => {
                bun.piece = Piece(PieceType::Snake, PieceStatus::Aggressive);
                bun.sprite.texture = asset_server.load("snake.png")
            }
            5 => {
                bun.piece = Piece(PieceType::MantisShrimp, PieceStatus::Aggressive);
                bun.sprite.texture = asset_server.load("shrimp.png")
            }
            6 => {
                bun.piece = Piece(PieceType::Sloth, PieceStatus::Passive);
                bun.sprite.texture = asset_server.load("sloth.png")
            }
            7 => {
                bun.piece = Piece(PieceType::Bird, PieceStatus::Both);
                bun.sprite.texture = asset_server.load("bird.png")
            }
            _ => unreachable!(),
        }
        let x_pos: f32 = (i % 8) as f32 * BOARD_SCALE as f32;
        let y_pos: f32 = (i / 8 * 7) as f32 * BOARD_SCALE as f32;
        bun.sprite.transform = Transform::from_xyz(
            x_pos - (BOARD_SCALE as f32 * 8. / 2.) + (BOARD_SCALE as f32 / 2.),
            y_pos - (BOARD_SCALE as f32 * 8. / 2.) + (BOARD_SCALE as f32 / 2.),
            1.,
        )
        .with_scale(Vec3::splat(0.14));

        let color = if bun.side == Side::Orange {
            Color::ORANGE.set_a(0.9).clone()
        } else {
            Color::rgba(0., 0., 0., 0.)
        };

        let marker = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::default()
                        .with_scale(Vec3::new(BOARD_SCALE as f32 * 5., 20., 1.))
                        .with_translation(Vec3::new(0., BOARD_SCALE as f32 * -3.3, 1.)),
                    material: materials.add(ColorMaterial::from(color)),
                    ..default()
                },
                RenderLayers::layer(2),
                GameEntity,
            ))
            .id();
        commands
            .spawn((
                bun,
                RenderLayers::layer(2),
                GameEntity,
                PickableBundle::default(),
                On::<Pointer<Click>>::run(crate::events::piece_click),
            ))
            .push_children(&[marker]);
    }
}

pub fn create_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for i in 0..64 {
        let x_pos = (i % 8) * BOARD_SCALE;
        let y_pos = (i / 8) * BOARD_SCALE;
        let color = if (i % 8 + (i / 8)) % 2 == 1 {
            Color::BLUE
        } else {
            Color::GRAY
        };

        let _x = (x_pos - (BOARD_SCALE * 8 / 2) + (BOARD_SCALE / 2)) as f32;
        let _y = (y_pos - (BOARD_SCALE * 8 / 2) + (BOARD_SCALE / 2)) as f32;
        let square_id = commands
            .spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(BOARD_SCALE as f32))
                        .with_translation(Vec3::new(
                            (x_pos - (BOARD_SCALE * 8 / 2) + (BOARD_SCALE / 2)) as f32,
                            (y_pos - (BOARD_SCALE * 8 / 2) + (BOARD_SCALE / 2)) as f32,
                            0.0,
                        )),
                    material: materials.add(ColorMaterial::from(color)),
                    ..default()
                },
                Square { move_entity: None },
                GameEntity,
                RenderLayers::layer(0),
                PickableBundle::default(),
                On::<Pointer<Click>>::run(crate::events::square_click),
            ))
            .id();
        board.0.push(square_id);
    }
}

pub fn create_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: bevy::ui::Style {
                    width: Val::Percent(100.0),
                    height: Val::Auto,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    // align_items: AlignItems::Center,
                    align_content: AlignContent::Start,
                    align_self: AlignSelf::Start,
                    margin: UiRect {
                        top: Val::Px(10.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                "Goats and Tigers",
                bevy::text::TextStyle {
                    font_size: 20.,
                    ..default()
                },
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: bevy::ui::Style {
                    width: Val::Auto,
                    height: Val::Auto,
                    // position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            GameEntity,
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "",
                    bevy::text::TextStyle {
                        font_size: 20.,
                        ..default()
                    },
                ),
                SelectionText,
            ));
        });
}

pub const BOARD_SCALE: i32 = 80;
