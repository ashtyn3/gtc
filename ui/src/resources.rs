use crate::components::Piece;
use bevy::prelude::{Entity, Resource};

#[derive(Debug, Resource)]
pub struct Board(pub Vec<Entity>);

#[derive(Debug, Resource)]
pub struct Selected {
    pub piece: Piece,
    pub encoded: String,
    pub entity: Entity,
    pub square: Entity,
    pub moves: Vec<usize>,
}

#[derive(Debug, Resource)]
pub struct Game {
    notated: String,
}
