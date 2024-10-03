use bevy::{
    asset::Handle,
    pbr::StandardMaterial,
    prelude::{Mesh, Resource},
};
use vhultman_chess::ChessMove;
use vhultman_chess::Color as PieceColor;
use vhultman_chess::Position;

#[derive(Resource)]
pub struct PieceModelData {
    pub pawn_parts: Vec<Handle<Mesh>>,
    pub rook_parts: Vec<Handle<Mesh>>,
    pub knight_parts: Vec<Handle<Mesh>>,
    pub bishop_parts: Vec<Handle<Mesh>>,
    pub queen_parts: Vec<Handle<Mesh>>,
    pub king_parts: Vec<Handle<Mesh>>,
    pub white_material: Handle<StandardMaterial>,
    pub black_material: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub struct SquareResourceData {
    pub white_square: Handle<StandardMaterial>,
    pub black_square: Handle<StandardMaterial>,
    pub selected_square: Handle<StandardMaterial>,
}

#[derive(Debug, PartialEq)]
pub enum NetworkState {
    Normal,
    AwaitingMove,
    AwaitingAck,
}

#[derive(Resource)]
pub struct ClientGameState {
    pub board_state: Position,
    pub selected_piece: Option<u32>,
    pub spawned_pieces: u32,
    pub board_dirty: bool,
    pub last_move: Option<ChessMove>,
    pub pending_promotion_move: Option<ChessMove>,
    pub own_color: PieceColor,
    pub network_state: NetworkState,
}
