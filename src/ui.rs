use bevy::prelude::*;
use vhultman_chess::{Color as PieceColor, GameState, PieceType};

use crate::ClientGameState;

#[derive(Component)]
pub struct TurnText;

#[derive(Component)]
pub struct GameStateText;

#[derive(Component)]
pub struct GameStatePopupWindow;

#[derive(Component)]
pub struct PromotionPopupWindow;

#[derive(Component, Clone, Copy, Debug)]
pub enum PromotionMenuAction {
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                padding: UiRect::all(Val::Px(12.0)),
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
        });

    // game state window
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..Default::default()
            },
            GameStatePopupWindow,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(12.0)),
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
        });

    // promotion window
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    display: Display::None,
                    ..Default::default()
                },
                border_radius: BorderRadius::all(Val::Px(6.0)),
                ..Default::default()
            },
            PromotionPopupWindow,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(12.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        display: Display::Flex,
                        row_gap: Val::Px(12.0),
                        ..default()
                    },
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    background_color: Srgba::rgba_u8(255, 255, 255, 100).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Choose promotion",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::srgb_u8(0, 0, 0),
                            ..default()
                        },
                    ));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                display: Display::Flex,
                                column_gap: Val::Px(6.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            let button_style = Style {
                                width: Val::Px(64.0),
                                aspect_ratio: Some(1.0),
                                padding: UiRect::all(Val::Px(8.0)),
                                ..default()
                            };

                            let buttons = [
                                (PromotionMenuAction::Knight, "sprites/white_knight.png"),
                                (PromotionMenuAction::Bishop, "sprites/white_bishop.png"),
                                (PromotionMenuAction::Rook, "sprites/white_rook.png"),
                                (PromotionMenuAction::Queen, "sprites/white_queen.png"),
                            ];

                            for (action, texture) in buttons.iter() {
                                parent
                                    .spawn((
                                        ButtonBundle {
                                            style: button_style.clone(),
                                            border_radius: BorderRadius::all(Val::Px(6.0)),
                                            background_color: Srgba::rgb_u8(255, 255, 255).into(),
                                            ..default()
                                        },
                                        *action,
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn(ImageBundle {
                                            image: UiImage::new(asset_server.load(*texture)),
                                            ..default()
                                        });
                                    });
                            }
                        });
                });
        });
}

pub fn update_ui(
    mut text_query: Query<(&mut Text, Option<&TurnText>, Option<&GameStateText>)>,
    mut windows_query: Query<(
        &mut Style,
        Option<&GameStatePopupWindow>,
        Option<&PromotionPopupWindow>,
    )>,
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

        // Update game state text
        if game_state_text.is_some() {
            text.sections[0].value = match game_state.board_state.check_game_state() {
                GameState::Playing => "",
                GameState::Checkmate => "Checkmate",
                GameState::Stalemate => "Stalemate",
                GameState::DrawByRepetition => "Draw",
                GameState::DrawByInsufficientMaterial => "Draw",
            }
            .to_string();
        }
    }

    for (mut style, game_state_wnd, promotion_wnd) in windows_query.iter_mut() {
        if game_state_wnd.is_some() {
            // popup window logic
            style.display = match game_state.board_state.check_game_state() {
                GameState::Playing => Display::None,
                _ => Display::Flex,
            };
        }

        if promotion_wnd.is_some() {
            if game_state.pending_promotion_move.is_some() {
                style.display = Display::Flex;
            } else {
                style.display = Display::None;
            }
        }
    }
}

pub(crate) fn promotion_menu_action(
    menu_action_query: Query<(&PromotionMenuAction, &Interaction), With<Button>>,
    mut game_state: ResMut<ClientGameState>,
) {
    for (action, interaction) in &menu_action_query {
        if *interaction == Interaction::Pressed {
            if let Some(mut m) = game_state.pending_promotion_move {
                m.set_promotion_piece(match action {
                    PromotionMenuAction::Knight => PieceType::Knight,
                    PromotionMenuAction::Bishop => PieceType::Bishop,
                    PromotionMenuAction::Rook => PieceType::Rook,
                    PromotionMenuAction::Queen => PieceType::Queen,
                });
                game_state.board_state.make_move(m);
                game_state.board_dirty = true;
                game_state.pending_promotion_move = None;
            }
        }
    }
}
