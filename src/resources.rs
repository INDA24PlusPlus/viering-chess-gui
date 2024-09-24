use bevy::{
    asset::Handle,
    pbr::StandardMaterial,
    prelude::{Mesh, Resource},
};
use vhultman_chess::Position;

#[derive(Resource)]
pub(crate) struct PieceModelData {
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
pub(crate) struct SquareResourceData {
    pub white_square: Handle<StandardMaterial>,
    pub black_square: Handle<StandardMaterial>,
    pub selected_square: Handle<StandardMaterial>,
}

#[derive(Resource)]
pub(crate) struct ClientGameState {
    pub board_state: Position,
    pub selected_piece: Option<u32>,
}
