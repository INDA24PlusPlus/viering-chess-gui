use bevy::prelude::*;
use vhultman_chess::Color as PieceColor;

use crate::ClientGameState;

#[derive(Component)]
pub struct TurnText;

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
        });
}

pub fn update_ui(
    mut turn_text_query: Query<&mut Text, With<TurnText>>,
    game_state: Res<ClientGameState>,
) {
    // Turn text
    turn_text_query
        .iter_mut()
        .next()
        .expect("No turn text found")
        .sections[0]
        .value = format!(
        "{}'s turn",
        match game_state.board_state.current_side() {
            PieceColor::White => "White",
            PieceColor::Black => "Black",
        }
    );
}
