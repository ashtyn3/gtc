use bevy::prelude::{
    Bundle, Commands, Component, DespawnRecursiveExt, Entity, Query, SpriteBundle, States, With,
};
use strum::{Display, EnumString};

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
pub struct Piece(pub PieceType, pub PieceStatus);

#[derive(Component, Debug, Copy, Clone)]
pub struct Square {
    pub move_entity: Option<Entity>,
}

#[derive(Component, Debug, Clone)]
pub struct PossibleMove(pub String);

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Bundle)]
pub struct PieceBundle {
    pub side: Side,
    pub piece: Piece,
    pub position: Position,
    pub sprite: SpriteBundle,
}

#[derive(Component, Debug, Clone)]
pub struct SelectionText;

#[derive(Component, Debug, Clone)]
pub struct GameEntity;

#[derive(Component, Debug, Clone)]
pub struct MenuEntity;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
