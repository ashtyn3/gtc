use crate::components::{GameState, Piece, Position, PossibleMove, SelectionText, Side, Square};
use crate::init::BOARD_SCALE;
use crate::proc::Instance;
use crate::resources::{Board, Selected};
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{
    default, shape, Color, ColorMaterial, Commands, DespawnRecursiveExt, Entity, Event,
    EventReader, EventWriter, Mesh, NextState, ParamSet, Query, Res, ResMut, Transform, With,
    World,
};
use bevy::render::view::RenderLayers;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::text::Text;
use bevy_mod_picking::events::{Click, Pointer};
use bevy_mod_picking::prelude::{ListenerMut, On};
use bevy_mod_picking::PickableBundle;
use gtc::position::Normalizable;
use std::str::FromStr;

#[derive(Event, Debug)]
pub struct TakeEvent(Entity, Position);

pub fn take(
    mut commands: Commands,
    mut ev: EventReader<TakeEvent>,
    selected: Option<ResMut<Selected>>,
    mut engine: Option<ResMut<Instance>>,
    mut pieces: Query<(&mut Position, &Side, &mut Transform)>,
) {
    let mut engine = engine.unwrap();
    for ev in ev.read() {
        if pieces.get(ev.0).unwrap().1 == pieces.get(selected.as_ref().unwrap().entity).unwrap().1 {
            return;
        }
        engine.cmd(
            format!(
                "m {} {}",
                selected.as_ref().unwrap().encoded,
                (ev.1.x as u64, ev.1.y as u64).encode().unwrap()
            )
            .as_str(),
        );
        *pieces
            .get_mut(commands.entity(selected.as_ref().unwrap().entity).id())
            .as_mut()
            .unwrap()
            .2
            .as_mut() = pieces.get(ev.0).unwrap().2.clone();

        *pieces
            .get_mut(commands.entity(selected.as_ref().unwrap().entity).id())
            .as_mut()
            .unwrap()
            .0 = ev.1;

        commands.entity(ev.0).despawn_recursive()
    }
}

pub fn state(mut engine: Option<ResMut<Instance>>, mut game_state: ResMut<NextState<GameState>>) {
    if engine.as_mut().unwrap().as_mut().cmd("set") == "true" {
        // println!("{:?}", engine.unwrap().state());
        if engine.unwrap().as_mut().state().win == true {
            game_state.set(GameState::Menu);
        }
    }
}

pub fn piece_click(
    mut commands: Commands,
    e: ListenerMut<Pointer<Click>>,
    board: Res<Board>,
    mut set: ParamSet<(
        Query<(&Piece, &Side, &Position), With<Piece>>,
        Query<&mut Transform>,
        Query<(Entity, &mut PossibleMove), With<PossibleMove>>,
    )>,
    mut text: Query<&mut Text, With<SelectionText>>,
    mut squares: Query<&mut Square, With<Square>>,
    mut engine: Option<ResMut<Instance>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    selected: Option<Res<Selected>>,
    mut taker: EventWriter<TakeEvent>,
) {
    if engine.as_mut().unwrap().as_mut().cmd("set") == "false" {
        engine.as_mut().unwrap().as_mut().cmd("l");
    }
    for p in set.p2().iter() {
        commands.entity(p.0).despawn();
        let move_position_normal = gtc::position::decode_position(p.1.clone().0)
            .unwrap()
            .normal();

        **squares
            .get_mut(
                commands
                    .entity(*board.0.get(move_position_normal).unwrap())
                    .id(),
            )
            .as_mut()
            .unwrap() = Square { move_entity: None };
    }
    let pieces = set.p0();

    // let piece = pieces.get_mut(e.target).unwrap();
    if pieces.get(e.target).is_err() {
        return;
    }
    let piece_pos = pieces.get(e.target).unwrap().2;
    let piece_side = pieces.get(e.target).unwrap().1;
    if piece_side.to_string() != engine.as_mut().unwrap().as_mut().cmd("t") {
        if selected.is_some() {
            if selected
                .as_ref()
                .unwrap()
                .moves
                .contains(&(piece_pos.x as u64, piece_pos.y as u64).normal())
            {
                taker.send(TakeEvent(e.target, *piece_pos));
            }
        }
        return;
    }

    let normal_piece_pos = (piece_pos.x as u64, piece_pos.y as u64).normal();

    let square = *board.0.get(normal_piece_pos).unwrap();

    if !text.is_empty() {
        println!("updating");
        text.single_mut().sections[0].value = pieces.get(e.target).unwrap().0 .0.to_string();
    }
    // let mut s = set;
    // if squares
    //     .get(commands.entity(square).id())
    //     .unwrap()
    //     .move_entity
    //     .is_some()
    // {
    //     println!("is move");
    // commands.entity(e.target).despawn();
    // let move_entity = commands.entity(squares.get(square).unwrap().move_entity.unwrap());
    // move_entity
    // }

    let mut enc =
        gtc::piece::Piece::from_str(pieces.get(e.target).unwrap().0 .0.to_string().as_str())
            .unwrap()
            .encode();

    enc = enc.to_lowercase();
    if piece_side.to_owned() == Side::Orange {
        enc = enc.to_uppercase()
    }

    let raw_command = engine
        .unwrap()
        .as_mut()
        .cmd(format!("g {}", enc).as_str())
        .clone();
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
        let mut position = set
            .p1()
            .get(*board.0.get(*x).unwrap())
            .unwrap()
            .translation
            .clone();
        position.z = 100.0;
        // squares
        //     .get(*board.0.get(*x).unwrap()).unwrap().move_entity =
        let pos_move = commands
            .spawn((
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
                Square { move_entity: None },
            ))
            .id();
        commands.entity(*board.0.get(*x).unwrap()).insert(Square {
            move_entity: Some(pos_move),
        });
    });
    // let square = all.get(*board.0.get(normal_piece_pos).unwrap());
    commands.insert_resource(Selected {
        encoded: enc,
        piece: *set.p0().get(e.target).unwrap().0,
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

pub fn square_click(
    mut commands: Commands,
    e: ListenerMut<Pointer<Click>>,
    selected: Option<ResMut<Selected>>,
    _board: ResMut<Board>,
    mut set: ParamSet<(
        Query<(&Piece, &Side, &mut Transform, &mut Position), With<Piece>>,
        Query<&Transform, With<Square>>,
        Query<(&Transform, Entity, &PossibleMove), With<PossibleMove>>,
    )>,
    mut engine: Option<ResMut<Instance>>,
) {
    let _ = commands;
    if engine.as_mut().unwrap().as_mut().cmd("set") == "false" {
        engine.as_mut().unwrap().as_mut().cmd("l");
    }
    if selected.is_some() {
        if set.p0().get(selected.as_ref().unwrap().entity).is_err() {
            return;
        }
        if set
            .p0()
            .get(selected.as_ref().unwrap().entity)
            .unwrap()
            .1
            .to_string()
            != engine.as_mut().unwrap().as_mut().cmd("t")
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
        set.p2().iter().for_each(|(x, entity, m)| {
            if x.translation == possible {
                commands.entity(entity).despawn();
                new_position = Some(x.translation);
                move_string = m.to_owned().0;
            }
            commands.entity(entity).despawn();
        });
        if new_position.is_some() {
            new_position.as_mut().unwrap().z = 1.;
            set.p0()
                .get_mut(selected.as_ref().unwrap().entity)
                .as_mut()
                .unwrap()
                .2
                .translation = new_position.unwrap();
            let decoded = gtc::position::decode_position(move_string.clone());
            *set.p0()
                .get_mut(selected.as_ref().unwrap().entity)
                .as_mut()
                .unwrap()
                .3 = Position {
                x: decoded.unwrap().0 as usize,
                y: decoded.unwrap().1 as usize,
            };

            println!("{:?}", decoded);
            // board.0.get(decoded.unwrap().normal()).unwrap()
            engine
                .unwrap()
                .as_mut()
                .cmd(format!("m {} {}", selected.unwrap().encoded, move_string).as_str());
        }
    }
}

pub fn game_exit(mut commands: Commands) {
    commands.remove_resource::<Board>();
    commands.insert_resource::<Board>(Board(vec![]));
}
