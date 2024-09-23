use bevy::prelude::Component;
use vhultman_chess::Color as PieceColor;
use vhultman_chess::PieceType;

#[derive(Component)]
pub(crate) struct ChessPiece {
    pub _piece_type: PieceType,
    pub _color: PieceColor,
    pub id: usize,
}

#[derive(Component)]
pub(crate) struct ChessPiecePart;

#[derive(Component)]
pub(crate) struct ChessSquare {
    pub id: u32,
    pub offset: bool,
}
