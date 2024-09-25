use bevy::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineMode, OutlineVolume};
use bevy_mod_picking::PickableBundle;
use vhultman_chess::{Color as PieceColor, Piece, PieceType};

use std::f32::consts::PI;

use bevy::prelude::Color;

use crate::{ChessPiece, ChessPiecePart, ClientGameState, PieceModelData};

pub(crate) fn spawn_piece(
    commands: &mut Commands,
    piece_model_data: &PieceModelData,
    piece_type: PieceType,
    color: PieceColor,
    position: Vec3,
    game_state: &mut ClientGameState,
) {
    let material = if color == PieceColor::White {
        piece_model_data.white_material.clone()
    } else {
        piece_model_data.black_material.clone()
    };

    let parts: &Vec<Handle<Mesh>>;
    let scale: Vec3;
    let rotation: Quat =
        Quat::from_rotation_y(-PI / 2.0 + if color == PieceColor::White { 0.0 } else { PI });

    match piece_type {
        PieceType::Pawn => {
            parts = &piece_model_data.pawn_parts;
            scale = Vec3::splat(0.24);
        }
        PieceType::Rook => {
            parts = &piece_model_data.rook_parts;
            scale = Vec3::splat(0.23);
        }
        PieceType::Knight => {
            parts = &piece_model_data.knight_parts;
            scale = Vec3::splat(0.32);
        }
        PieceType::Bishop => {
            parts = &piece_model_data.bishop_parts;
            scale = Vec3::splat(0.24);
        }
        PieceType::Queen => {
            parts = &piece_model_data.queen_parts;
            scale = Vec3::splat(0.23);
        }
        PieceType::King => {
            parts = &piece_model_data.king_parts;
            scale = Vec3::splat(0.18);
        }
    };

    let parent = commands
        .spawn((SpatialBundle {
            transform: Transform {
                translation: position,
                scale,
                rotation,
            },
            ..Default::default()
        },))
        .insert(PickableBundle::default())
        .insert(ChessPiece {
            piece: Piece {
                t: piece_type,
                color,
            },
            id: game_state.spawned_pieces,
        })
        .id();

    for part in parts.iter() {
        let child = commands
            .spawn((PbrBundle {
                mesh: part.clone(),
                material: material.clone(),
                ..Default::default()
            },))
            .insert(OutlineBundle {
                outline: OutlineVolume {
                    visible: true,
                    colour: Color::srgb(1.0, 1.0, 1.0),
                    width: 1.5,
                },
                mode: OutlineMode::RealVertex,
                ..Default::default()
            })
            .insert(ChessPiecePart)
            .id();

        commands.entity(parent).push_children(&[child]);
    }

    game_state.spawned_pieces += 1;
}
