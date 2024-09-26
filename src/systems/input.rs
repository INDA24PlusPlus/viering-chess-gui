use bevy::prelude::*;

use bevy_mod_outline::*;
use bevy_mod_picking::*;
use events::{Click, Pointer};

use crate::{
    board_id_to_world_pos, world_pos_to_board_id, ChessPiece, ChessPiecePart, ChessSquare,
    ClientGameState, PieceModelData, SoundEffects, SquareResourceData,
};

use super::pieces::spawn_piece;

// TODO long ass function that handles all clicking behaviour, should definitely be split up
#[allow(clippy::too_many_arguments)]
pub fn handle_picking(
    mut commands: Commands,
    mut events: EventReader<Pointer<Click>>,
    mut query: Query<(&Parent, &mut OutlineVolume), With<ChessPiecePart>>,
    mut piece_query: Query<(Entity, &mut Transform, &ChessPiece, &Children)>,
    mut tile_query: Query<(&mut Handle<StandardMaterial>, &ChessSquare)>,
    mut game_state: ResMut<ClientGameState>,
    square_resource_data: Res<SquareResourceData>,
    piece_model_data: Res<PieceModelData>,
    sound_effects: Res<SoundEffects>,
) {
    let mut square = None;
    // Handle selection and deselection
    for ev in events.read() {
        // Clicked a piece (might have selected, deselected, attempted to move, etc)
        let mut might_move_piece = false;
        if let Ok((parent, _)) = query.get_mut(ev.target) {
            let parent_entity = commands.entity(**parent);

            // set selected piece
            if let Ok((_, transform, chess_piece, _)) = piece_query.get_mut(parent_entity.id()) {
                if chess_piece.piece.color != game_state.board_state.current_side() {
                    square = Some(world_pos_to_board_id(transform.translation));
                    might_move_piece = true;

                    if game_state.selected_piece.is_none() {
                        commands.spawn(AudioBundle {
                            source: sound_effects.illegal_move.clone(),
                            ..default()
                        });
                    }
                } else if game_state.selected_piece == Some(chess_piece.id) {
                    game_state.selected_piece = None;
                } else {
                    commands.spawn(AudioBundle {
                        source: sound_effects.select.clone(),
                        ..default()
                    });
                    game_state.selected_piece = Some(chess_piece.id);
                }
            }
        } else {
            might_move_piece = true;
        }

        // attempting to move a piece
        if might_move_piece {
            let mut square: Option<u32> = square;

            if let Ok((_, square_val)) = tile_query.get_mut(ev.target) {
                square = Some(square_val.id)
            }

            if let Some(square) = square {
                if let Some(selected_piece_id) = game_state.selected_piece {
                    // we clicked a square and a piece is selected

                    // get square id of the selected piece
                    let (piece_square_id, mut piece_transform) = piece_query
                        .iter_mut()
                        .find_map(|(_, transform, piece, _)| {
                            if piece.id == selected_piece_id {
                                Some((world_pos_to_board_id(transform.translation), transform))
                            } else {
                                None
                            }
                        })
                        .unwrap();

                    let possible_moves: Vec<u32> = game_state
                        .board_state
                        .moves_for_square(piece_square_id)
                        .iter()
                        .map(|m| m.to())
                        .collect();

                    if possible_moves.contains(&square) {
                        let possible_move =
                            game_state.board_state.get_move(piece_square_id, square);

                        if let Some(m) = possible_move {
                            // Here we make the move
                            game_state.board_state.make_move(m);

                            // update position of the moved piece
                            piece_transform.translation = board_id_to_world_pos(square);

                            // TODO handle check, checkmate, draw, promotion, etc

                            let pieces: Vec<(Entity, Mut<'_, Transform>, &ChessPiece)> =
                                piece_query
                                    .iter_mut()
                                    .map(|(entity, transform, piece, _)| (entity, transform, piece))
                                    .collect();

                            let mut played_sound = false;
                            // despawn all pieces that aren't supposed to exist
                            for &(entity, ref transform, piece) in &pieces {
                                let board_id = world_pos_to_board_id(transform.translation);

                                if game_state.board_state.piece_on(board_id).map_or(
                                    true,
                                    |correct_piece| {
                                        piece.piece.color != correct_piece.color
                                            || piece.piece.t != correct_piece.t
                                    },
                                ) {
                                    commands.spawn(AudioBundle {
                                        source: sound_effects.capture.clone(),
                                        ..default()
                                    });
                                    played_sound = true;
                                    commands.entity(entity).despawn_recursive();
                                }
                            }

                            if !played_sound {
                                commands.spawn(AudioBundle {
                                    source: sound_effects.valid_move.clone(),
                                    ..default()
                                });
                            }

                            // spawn all pieces that are supposed to exist but don't
                            for i in 0..64 {
                                if let Some(piece) = game_state.board_state.piece_on(i) {
                                    if !pieces.iter().any(|(_, transform, _)| {
                                        world_pos_to_board_id(transform.translation) == i
                                    }) {
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
                    } else {
                        commands.spawn(AudioBundle {
                            source: sound_effects.illegal_move.clone(),
                            ..default()
                        });
                    }
                }
            }
            game_state.selected_piece = None;
        }

        // Update color etc. for all pieces after the change
        let mut selected_translation: Option<Vec3> = None;
        for (_, transform, piece, children) in piece_query.iter_mut() {
            let selected = if let Some(selected_piece) = game_state.selected_piece {
                piece.id == selected_piece
            } else {
                false
            };

            if selected {
                selected_translation = Some(transform.translation);
            }

            for child in children.iter() {
                if let Ok(mut lol) = query.get_mut(*child) {
                    lol.1.colour = if selected {
                        Color::srgb_u8(232, 61, 132)
                    } else {
                        Color::srgb(0.0, 0.0, 0.0)
                    };
                }
            }
        }

        let selected_square: Option<u32> = selected_translation.map(world_pos_to_board_id);

        let possible_moves: Vec<u32> = selected_square.map_or_else(Vec::new, |square| {
            game_state
                .board_state
                .moves_for_square(square)
                .iter()
                .map(|m| m.to())
                .collect()
        });

        for (mut material, square) in tile_query.iter_mut() {
            let possible = possible_moves.contains(&square.id);

            *material = if possible {
                square_resource_data.selected_square.clone()
            } else if square.offset {
                square_resource_data.white_square.clone()
            } else {
                square_resource_data.black_square.clone()
            };
        }
    }
}
