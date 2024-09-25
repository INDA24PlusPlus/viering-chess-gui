use bevy::prelude::*;
use vhultman_chess::Position;

use crate::{ClientGameState, PieceModelData, SquareResourceData};

pub(crate) fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // piece meshes
    let knight: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh0/Primitive0");
    let queen: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh1/Primitive0");
    let king: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh2/Primitive0");
    let pawn: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh3/Primitive0");
    let bishop_p1: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh4/Primitive0");
    let bishop_p2: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh5/Primitive0");
    let rook_p1: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh6/Primitive0");
    let rook_p2: Handle<Mesh> = asset_server.load("chess_pieces.glb#Mesh7/Primitive0");

    // piece color materials
    let white_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 1.0, 1.0),
        metallic: 0.2,
        reflectance: 1.0,
        ..Default::default()
    });
    let black_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.0, 0.0),
        metallic: 0.2,
        reflectance: 1.0,
        ..Default::default()
    });

    // Setup piece resources
    commands.insert_resource(PieceModelData {
        pawn_parts: vec![pawn],
        knight_parts: vec![knight],
        bishop_parts: vec![bishop_p1, bishop_p2],
        queen_parts: vec![queen],
        king_parts: vec![king],
        rook_parts: vec![rook_p1, rook_p2],
        white_material,
        black_material,
    });

    // Setup square resources
    commands.insert_resource(SquareResourceData {
        white_square: materials.add(Color::srgb_u8(255, 255, 255)),
        black_square: materials.add(Color::srgb_u8(0, 0, 0)),
        selected_square: materials.add(StandardMaterial {
            base_color: Color::srgb_u8(0, 255, 255),
            unlit: true,
            ..Default::default()
        }),
    });

    // Setup game state and more
    commands.insert_resource(ClientGameState {
        board_state: Position::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR").unwrap(),
        selected_piece: None,
        spawned_pieces: 0,
    });
}
