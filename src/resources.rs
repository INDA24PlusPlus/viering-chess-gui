use bevy::{
    asset::Handle,
    audio::AudioSource,
    pbr::StandardMaterial,
    prelude::{Mesh, Resource},
};
use vhultman_chess::Position;
use vhultman_chess::{ChessMove, Color as PieceColor};

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

#[derive(Resource)]
pub struct ClientGameState {
    pub board_state: Position,
    pub selected_piece: Option<u32>,
    pub spawned_pieces: u32,
}

#[derive(Resource)]
pub struct SoundEffects {
    pub select: Handle<AudioSource>,
    pub capture: Handle<AudioSource>,
    pub valid_move: Handle<AudioSource>,
    pub illegal_move: Handle<AudioSource>,
}
