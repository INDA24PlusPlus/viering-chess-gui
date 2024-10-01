use bevy::prelude::*;
use bevy_simple_text_input::TextInputBundle;

use crate::general::resources::SoundEffects;

use super::OnMainMenuScreen;

#[derive(Copy, Clone, PartialEq, Component, Debug)]
pub(crate) enum MenuAction {
    Host,
    Join,
}

const BUTTON_COLOR: Color = Color::srgb(100.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0);
const BUTTON_HOVER_COLOR: Color = Color::srgb(150.0 / 255.0, 150.0 / 255.0, 150.0 / 255.0);

pub(crate) fn menu_setup(mut commands: Commands) {
    // general setup
    commands.spawn((Camera2dBundle::default(), OnMainMenuScreen));

    // ui setup
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            // menu panel
            parent
                .spawn(NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(32.0)),
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(16.0),
                        ..Default::default()
                    },
                    border_radius: BorderRadius::all(Val::Px(12.0)),
                    background_color: Srgba::rgb_u8(50, 50, 50).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // title
                    parent.spawn(TextBundle::from_section(
                        "Very cool chess game",
                        TextStyle {
                            font_size: 42.0,
                            ..default()
                        },
                    ));

                    let button_bundle = ButtonBundle {
                        style: Style {
                            width: Val::Px(326.0),
                            padding: UiRect::all(Val::Px(8.0)),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        background_color: BUTTON_COLOR.into(),
                        ..default()
                    };

                    // host button
                    parent
                        .spawn((
                            {
                                let mut button = button_bundle.clone();
                                button.style.margin = UiRect {
                                    left: Val::Px(0.0),
                                    right: Val::Px(0.0),
                                    top: Val::Px(32.0),
                                    bottom: Val::Px(0.0),
                                };
                                button
                            },
                            MenuAction::Host,
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("Host", TextStyle { ..default() }));
                        });

                    // join area
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                display: Display::Flex,
                                flex_direction: FlexDirection::Row,
                                column_gap: Val::Px(6.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // text field
                            parent.spawn((
                                NodeBundle {
                                    style: Style {
                                        width: Val::Px(256.0),
                                        padding: UiRect::all(Val::Px(5.0)),
                                        display: Display::Flex,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                    background_color: Srgba::rgb_u8(100, 100, 100).into(),
                                    ..default()
                                },
                                TextInputBundle::default()
                                    .with_text_style(TextStyle { ..default() }),
                            ));

                            // button
                            parent
                                .spawn((
                                    {
                                        let mut bundle = button_bundle.clone();
                                        bundle.style.width = Val::Px(64.0);
                                        bundle
                                    },
                                    MenuAction::Join,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Join",
                                        TextStyle { ..default() },
                                    ));
                                });
                        });
                });
        });
}

pub(crate) fn menu_update(
    mut button_query: Query<
        (&MenuAction, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    sound_effects: Res<SoundEffects>,
) {
    for (action, interaction, mut background_color) in &mut button_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Pressed {:?} button", action);

                commands.spawn(AudioBundle {
                    source: sound_effects.click.clone(),
                    ..default()
                });
            }
            Interaction::Hovered => {
                *background_color = BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *background_color = BUTTON_COLOR.into();
            }
        }
    }
}
