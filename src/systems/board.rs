use bevy::prelude::*;
use bevy_mod_outline::{OutlineBundle, OutlineMode, OutlineVolume};
use bevy_mod_picking::PickableBundle;
use vhultman_chess::{Color as PieceColor, Piece, PieceType};

use std::f32::consts::PI;

use bevy::prelude::Color;

use crate::{
    board_id_to_world_pos, world_pos_to_board_id, ChessPiece, ChessPiecePart, ClientGameState,
    PieceModelData, SoundEffects,
};

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
                    colour: Color::srgb(0.0, 0.0, 0.0),
                    width: 2.0,
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

pub(crate) fn update_board(
    mut commands: Commands,
    mut piece_query: Query<(Entity, &mut Transform, &ChessPiece, &Children)>,
    mut game_state: ResMut<ClientGameState>,
    piece_model_data: Res<PieceModelData>,
    sound_effects: Res<SoundEffects>,
) {
    if !game_state.board_dirty {
        return;
    }

    game_state.board_dirty = false;

    let should_play_sound = game_state.last_move.is_some();

    let pieces: Vec<(Entity, Mut<'_, Transform>, &ChessPiece)> = piece_query
        .iter_mut()
        .map(|(entity, transform, piece, _)| (entity, transform, piece))
        .collect();

    let mut entities_to_despawn: Vec<Entity> = Vec::new();

    // move pieces and identify pieces to despawn
    for (entity, mut transform, piece) in pieces {
        if let Some(last_move) = game_state.last_move {
            if transform.translation == board_id_to_world_pos(last_move.from()) {
                transform.translation = board_id_to_world_pos(last_move.to());
            }
        }

        let board_id = world_pos_to_board_id(transform.translation);

        if game_state
            .board_state
            .piece_on(board_id)
            .map_or(true, |correct_piece| {
                piece.piece.color != correct_piece.color || piece.piece.t != correct_piece.t
            })
        {
            // queue the entity for despawning
            entities_to_despawn.push(entity);
        }
    }

    // spawn sound for despawning or valid move
    if should_play_sound {
        commands.spawn(AudioBundle {
            source: if !entities_to_despawn.is_empty() {
                sound_effects.capture.clone()
            } else {
                sound_effects.valid_move.clone()
            },
            ..default()
        });
    }

    // despawn pieces
    for entity in entities_to_despawn {
        commands.entity(entity).despawn_recursive();
    }

    // spawn pieces that should exist but don't
    for i in 0..64 {
        if let Some(piece) = game_state.board_state.piece_on(i) {
            // Check if a piece already exists in the right spot
            let piece_exists = piece_query
                .iter()
                .any(|(_, transform, _, _)| world_pos_to_board_id(transform.translation) == i);

            if !piece_exists {
                spawn_piece(
                    &mut commands,
                    &piece_model_data,
                    piece.t,
                    piece.color,
                    board_id_to_world_pos(i),
                    &mut game_state,
                );
            }
        }
    }
}
