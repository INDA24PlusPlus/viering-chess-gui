use bevy::prelude::*;
use vhultman_chess::{Color as PieceColor, GameState};

use crate::ClientGameState;

#[derive(Component)]
pub struct TurnText;

#[derive(Component)]
pub struct GameStateText;

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                aspect_ratio: Some(2.0),
                padding: UiRect {
                    left: Val::Px(12.0),
                    right: Val::Px(12.0),
                    top: Val::Px(12.0),
                    bottom: Val::Px(12.0),
                },
                margin: UiRect {
                    left: Val::Px(12.0),
                    top: Val::Px(12.0),
                    ..Default::default()
                },
                flex_direction: FlexDirection::Column,
                display: Display::Flex,
                row_gap: Val::Px(6.0),
                ..Default::default()
            },
            border_radius: BorderRadius::all(Val::Px(6.0)),
            background_color: Srgba::rgba_u8(255, 255, 255, 100).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "turn text",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb_u8(0, 0, 0),
                        ..default()
                    },
                ),
                TurnText,
            ));
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "game state text",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb_u8(0, 0, 0),
                        ..default()
                    },
                ),
                GameStateText,
            ));
        });
}

pub fn update_ui(
    mut text_query: Query<(&mut Text, Option<&TurnText>, Option<&GameStateText>)>,
    mut game_state: ResMut<ClientGameState>,
) {
    for (mut text, turn_text, game_state_text) in text_query.iter_mut() {
        // Update turn text
        if turn_text.is_some() {
            text.sections[0].value = format!(
                "{}'s turn",
                match game_state.board_state.current_side() {
                    PieceColor::White => "White",
                    PieceColor::Black => "Black",
                }
            );
        }

        // Update game state text TODO how to get check????? SHOULD BE SHOWN HERE
        if game_state_text.is_some() {
            text.sections[0].value = match game_state.board_state.check_game_state() {
                GameState::Playing => "balls",
                GameState::Checkmate => "Checkmate",
                GameState::Stalemate => "Stalemate",
                GameState::DrawByRepetition => "Draw",
                GameState::DrawByInsufficientMaterial => "Draw",
            }
            .to_string();
        }
    }
}
