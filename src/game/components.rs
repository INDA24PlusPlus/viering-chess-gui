use bevy::prelude::Component;
use vhultman_chess::Piece;

#[derive(Component)]
pub(crate) struct ChessPiece {
    pub piece: Piece,
    pub id: u32,
}

#[derive(Component)]
pub(crate) struct ChessPiecePart;

#[derive(Component)]
pub(crate) struct ChessSquare {
    pub id: u32,
    pub offset: bool,
}
