use bevy::prelude::*;

use bevy_mod_outline::*;
use bevy_mod_picking::*;
use events::{Click, Pointer};

use crate::{ChessPiece, ChessPiecePart, ChessSquare, ClientGameState, SquareResourceData};

pub(crate) fn handle_picking(
    mut commands: Commands,
    mut events: EventReader<Pointer<Click>>,
    mut query: Query<(&Parent, &mut OutlineVolume), With<ChessPiecePart>>,
    mut piece_query: Query<(&mut Transform, &ChessPiece, &Children)>,
    mut tile_query: Query<(&mut Handle<StandardMaterial>, &ChessSquare)>,
    mut game_state: ResMut<ClientGameState>,
    square_resource_data: Res<SquareResourceData>,
) {
    // Handle selection and deselection
    for ev in events.read() {
        if let Ok((parent, _)) = query.get_mut(ev.target) {
            let parent_entity = commands.entity(**parent);

            // set selected piece
            if let Ok((_, chess_piece, _)) = piece_query.get_mut(parent_entity.id()) {
                if game_state.selected_piece == Some(chess_piece.id) {
                    game_state.selected_piece = None;
                } else {
                    game_state.selected_piece = Some(chess_piece.id);
                }
            }
        } else {
            game_state.selected_piece = None;
        }

        // Update color etc. for all pieces after the change
        let mut selected_translation: Option<Vec3> = None;
        for (transform, piece, children) in piece_query.iter_mut() {
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

        let selected_square: Option<u32> = selected_translation
            .map(|translation| ((translation.z + 3.5) * 8.0 + translation.x + 3.5) as u32);

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
