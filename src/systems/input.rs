use bevy::prelude::*;

use bevy_mod_outline::*;
use bevy_mod_picking::*;
use events::{Click, Pointer};
use vhultman_chess::Piece;

use crate::{ChessPiece, ChessPiecePart, ChessSquare, ClientGameState, SquareResourceData};

pub(crate) fn handle_picking(
    mut commands: Commands,
    mut events: EventReader<Pointer<Click>>,
    mut query: Query<(&Parent, &mut OutlineVolume), With<ChessPiecePart>>,
    mut piece_query: Query<(Entity, &mut Transform, &ChessPiece, &Children)>,
    mut tile_query: Query<(&mut Handle<StandardMaterial>, &ChessSquare)>,
    mut game_state: ResMut<ClientGameState>,
    square_resource_data: Res<SquareResourceData>,
) {
    // Handle selection and deselection
    for ev in events.read() {
        // Clicked a piece
        if let Ok((parent, _)) = query.get_mut(ev.target) {
            let parent_entity = commands.entity(**parent);

            // set selected piece
            if let Ok((_, _, chess_piece, _)) = piece_query.get_mut(parent_entity.id()) {
                if game_state.selected_piece == Some(chess_piece.id) {
                    game_state.selected_piece = None;
                } else {
                    game_state.selected_piece = Some(chess_piece.id);
                }
            }
        } else {
            // TODO can also click enemy team piece, right?
            if let Ok((_, square)) = tile_query.get_mut(ev.target) {
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

                    if possible_moves.contains(&square.id) {
                        let possible_move =
                            game_state.board_state.get_move(piece_square_id, square.id);

                        if let Some(m) = possible_move {
                            // Here we make the move
                            game_state.board_state.make_move(m);

                            // update position of the moved piece
                            piece_transform.translation = board_id_to_world_pos(square.id);

                            // TODO handle check, checkmate, draw, promotion, etc, and rook during
                            // castling

                            // go through all pieceso on the board, if they aren't supposed to be
                            // where they are they will be despawned
                            for piece in piece_query.iter() {
                                let board_id = world_pos_to_board_id(piece.1.translation);

                                if game_state.board_state.piece_on(board_id).map_or(
                                    true,
                                    |correct_piece| {
                                        piece.2.piece.color != correct_piece.color
                                            || piece.2.piece.t != correct_piece.t
                                    },
                                ) {
                                    commands.entity(piece.0).despawn_recursive();
                                }
                            }
                        }
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
                        Color::srgb(0.0, 1.0, 1.0)
                    } else {
                        Color::srgb(1.0, 1.0, 1.0)
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

fn world_pos_to_board_id(world_pos: Vec3) -> u32 {
    ((world_pos.z + 3.5) * 8.0 + world_pos.x + 3.5) as u32
}

fn board_id_to_world_pos(board_id: u32) -> Vec3 {
    Vec3::new(
        (board_id % 8) as f32 - 3.5,
        0.1,
        (board_id / 8) as f32 - 3.5,
    )
}
