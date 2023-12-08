use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
    str::FromStr,
};

use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    input::common_conditions::input_just_pressed,
    prelude::*,
    render::view::RenderLayers,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};
use bevy_mod_picking::{
    prelude::{Click, ListenerMut, On, Pointer},
    DefaultPickingPlugins, PickableBundle,
};
use bevy_simple_2d_outline::{OutlineAndTextureMaterial, OutlineMaterial};
use gtc::position::Normalizable;
use strum::{Display, EnumString};

fn gtc_startup(mut commands: Commands) {
    let cmd = match Command::new("gtc")
        .arg("--mode")
        .arg("protocol")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(mut child) => commands.insert_resource(Instance {
            reader: child.stdout.take().unwrap(),
            input: child.stdin.take().unwrap(),
        }),
        Err(err) => {
            panic!("{}", err)
        }
    };
}

fn cmd(mut engine: &mut ResMut<Instance>, cmd: &str) -> String {
    let _ = engine
        .input
        .write((cmd.to_owned() + &"\n".to_owned()).as_bytes());
    let _ = engine.input.write(b"ping\n");
    let mut buf = String::new();
    for line in BufReader::new(engine.reader.by_ref()).lines() {
        if line.as_ref().unwrap() == "ok" {
            break;
        }
        if line.as_ref().unwrap() == "gtc 0.1.0" {
            continue;
        }
        if line.as_ref().unwrap().trim() == "" {
            continue;
        }
        buf.push_str(line.as_ref().unwrap().as_str());
        buf.push_str("\n");
    }

    println!("({}):\n{}\n", cmd, buf);
    return buf.trim().to_string();
}

fn main() {
    App::new()
        .add_systems(Startup, gtc_startup)
        .add_systems(Startup, (create_board, create_pieces))
        .insert_resource(Board(vec![]))
        .add_plugins(DefaultPickingPlugins)
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Goats and Tigers (the game)".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),))
        .run()
}

#[derive(Debug, Resource)]
pub struct Instance {
    reader: ChildStdout,
    input: ChildStdin,
}

#[derive(Debug, Resource)]
pub struct Board(Vec<Entity>);

#[derive(Debug, Resource)]
pub struct Selected {
    piece: Piece,
    encoded: String,
    entity: Entity,
    square: Entity,
    moves: Vec<usize>,
}

pub const BOARD_SCALE: i32 = 80;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Component, EnumString, Display)]
pub enum Side {
    Orange,
    White,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy, EnumString, Display)]
pub enum PieceType {
    None,
    Goat,
    Horse,
    Sloth,
    Bird,
    Tiger,
    Otter,
    Snake,
    MantisShrimp,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum PieceStatus {
    Both,
    Aggressive,
    Passive,
}

#[derive(Component, Debug, Copy, Clone)]
struct Piece(PieceType, PieceStatus);

#[derive(Component, Debug, Copy, Clone)]
struct Square;

#[derive(Component, Debug, Clone)]
struct PossibleMove(String);

#[derive(Component, Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Bundle)]
struct PieceBundle {
    side: Side,
    piece: Piece,
    position: Position,
    sprite: SpriteBundle,
}

fn create_pieces(
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
                        .with_scale(Vec3::new(BOARD_SCALE as f32 * 5., 12., 1.))
                        .with_translation(Vec3::new(0., BOARD_SCALE as f32 * -3.3, 1.)),
                    material: materials.add(ColorMaterial::from(color)),
                    ..default()
                },
                RenderLayers::layer(2),
            ))
            .id();
        commands
            .spawn((
                bun,
                RenderLayers::layer(2),
                PickableBundle::default(),
                On::<Pointer<Click>>::run(piece_click),
            ))
            .push_children(&[marker]);
    }
}

fn piece_click(
    mut commands: Commands,
    e: ListenerMut<Pointer<Click>>,
    board: Res<Board>,
    mut set: ParamSet<(
        Query<(&Piece, &Side, &Position), With<Piece>>,
        Query<&mut Transform>,
        Query<(Entity), With<PossibleMove>>,
    )>,
    mut engine: Option<ResMut<Instance>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if cmd(&mut engine.as_mut().unwrap(), "set") == "false" {
        cmd(&mut engine.as_mut().unwrap(), "l");
    }
    let mut s = set;
    for p in s.p2().iter() {
        commands.entity(p).despawn()
    }
    let pieces = s.p0();

    // let piece = pieces.get_mut(e.target).unwrap();
    let piece_pos = pieces.get(e.target).unwrap().2;
    let piece_side = pieces.get(e.target).unwrap().1;
    if piece_side.to_string() != cmd(&mut engine.as_mut().unwrap(), "t") {
        return;
    }

    let normal_piece_pos = (piece_pos.x as u64, piece_pos.y as u64).normal();

    let square = *board.0.get(normal_piece_pos).unwrap();

    let mut enc =
        gtc::piece::Piece::from_str(pieces.get(e.target).unwrap().0 .0.to_string().as_str())
            .unwrap()
            .encode();

    enc = enc.to_lowercase();
    if piece_side.to_owned() == Side::Orange {
        enc = enc.to_uppercase()
    }

    let raw_command = cmd(&mut engine.as_mut().unwrap(), format!("g {}", enc).as_str()).clone();
    let move_str: Vec<&str> = raw_command
        .split("\n")
        .map(|x| x.split("-").collect::<Vec<&str>>()[1])
        .map(|x| x.get(1..).unwrap())
        .collect();
    let moves = move_str
        .iter()
        .map(|x| {
            gtc::position::decode_position(x.to_string())
                .unwrap()
                .normal()
        })
        .collect::<Vec<usize>>();

    moves.iter().enumerate().for_each(|(i, x)| {
        let mut position = s
            .p1()
            .get(*board.0.get(*x).unwrap())
            .unwrap()
            .translation
            .clone();
        position.z = 100.0;
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                transform: Transform::default()
                    .with_scale(Vec3::splat(BOARD_SCALE as f32))
                    .with_translation(position),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                ..default()
            },
            PickableBundle::default(),
            RenderLayers::layer(3),
            On::<Pointer<Click>>::run(square_click),
            PossibleMove(move_str[i].to_string()),
            Square,
        ));
    });
    // let square = all.get(*board.0.get(normal_piece_pos).unwrap());
    commands.insert_resource(Selected {
        encoded: enc,
        piece: *s.p0().get(e.target).unwrap().0,
        entity: e.target,
        moves,
        square,
    })
    // match piece.1 {
    //     Side::Orange => {
    //         // piece.3.translation.y += BOARD_SCALE as f32;
    //     }
    //     Side::White => {
    //         // piece.3.translation.y -= BOARD_SCALE as f32;
    //     }
    // }
}

fn square_click(
    mut commands: Commands,
    e: ListenerMut<Pointer<Click>>,
    selected: Option<ResMut<Selected>>,
    board: ResMut<Board>,
    mut set: ParamSet<(
        Query<(&Piece, &Side, &mut Transform), With<Piece>>,
        Query<&Transform, With<Square>>,
        Query<(&Transform, Entity, &PossibleMove), With<PossibleMove>>,
    )>,
    mut engine: Option<ResMut<Instance>>,
) {
    let _ = commands;
    if cmd(&mut engine.as_mut().unwrap(), "set") == "false" {
        cmd(&mut engine.as_mut().unwrap(), "l");
    }
    if selected.is_some() {
        if set
            .p0()
            .get(selected.as_ref().unwrap().entity)
            .unwrap()
            .1
            .to_string()
            != cmd(&mut engine.as_mut().unwrap(), "t")
        {
            return;
        }
        // let p = pieces
        //     .p0()
        //     .get(selected.as_ref().unwrap().entity)
        //     .unwrap()
        //     .0
        //      .0
        //     .to_string();

        let possible = set.p1().get(e.target).unwrap().translation;
        let mut new_position: Option<Vec3> = None;
        let mut move_string: String = String::new();
        set.p2().iter().for_each(|(x, E, m)| {
            if x.translation == possible {
                commands.entity(E).despawn();
                new_position = Some(x.translation);
                move_string = m.to_owned().0;
            }
            commands.entity(E).despawn();
        });
        if new_position.is_some() {
            new_position.as_mut().unwrap().z = 1.;
            set.p0()
                .get_mut(selected.as_ref().unwrap().entity)
                .as_mut()
                .unwrap()
                .2
                .translation = new_position.unwrap();
            cmd(
                &mut engine.as_mut().unwrap(),
                format!("m {} {}", selected.unwrap().encoded, move_string).as_str(),
            );
        }
    }
}

fn create_board(
    mut commands: Commands,
    mut board: ResMut<Board>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut cam = Camera2dBundle::default();
    cam.camera.hdr = true;
    commands.spawn((cam, RenderLayers::from_layers(&[0, 1, 2, 3])));

    for i in 0..64 {
        let x_pos = (i % 8) * BOARD_SCALE;
        let y_pos = (i / 8) * BOARD_SCALE;
        let color = if (i % 8 + (i / 8)) % 2 == 1 {
            Color::BLUE
        } else {
            Color::GRAY
        };

        let x = (x_pos - (BOARD_SCALE * 8 / 2) + (BOARD_SCALE / 2)) as f32;
        let y = (y_pos - (BOARD_SCALE * 8 / 2) + (BOARD_SCALE / 2)) as f32;
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
                Square,
                RenderLayers::layer(0),
                PickableBundle::default(),
                On::<Pointer<Click>>::run(square_click),
            ))
            .id();
        board.0.push(square_id);
    }
}
