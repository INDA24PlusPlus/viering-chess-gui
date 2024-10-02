use bevy::prelude::*;

use crate::{general::resources::SoundEffects, GameState};

use super::OnSplashScreen;

#[derive(Component)]
pub(crate) struct FadeOverlay;

pub(crate) fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // general setup
    commands.spawn((Camera2dBundle::default(), OnSplashScreen));

    // ui setup
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(16.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgb_u8(25, 25, 25).into(),
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Vw(33.0),
                    ..default()
                },
                image: UiImage::new(asset_server.load("sprites/kebab.png")),
                ..default()
            });
            parent.spawn(TextBundle::from_section(
                "En kebabproduktion",
                TextStyle {
                    font_size: 32.0,
                    ..default()
                },
            ));
        });

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Vw(100.0),
                height: Val::Vw(100.0),
                ..default()
            },
            ..default()
        },
        OnSplashScreen,
        FadeOverlay,
    ));
}

pub(crate) fn splash_update(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut fade_query: Query<&mut BackgroundColor, With<FadeOverlay>>,
    mut opacity: Local<f32>,
    mut timer: Local<f32>,
    sound_effects: Res<SoundEffects>,
    mut commands: Commands,
    mut played_sfx: Local<bool>,
) {
    let mut fade_element = fade_query
        .iter_mut()
        .next()
        .expect("Couldn't find a FadeOverlay element");

    // start as fully faded
    if *timer == 0.0 {
        *opacity = 1.0;
    }

    *timer += time.delta_seconds();

    // reduce fade
    if *timer >= 0.5 && *timer <= 4.0 {
        *opacity = (*opacity - time.delta_seconds() * 2.0).max(0.0);

        // play the amazing kebab splash screen sound effect
        if !*played_sfx {
            *played_sfx = true;
            commands.spawn(AudioBundle {
                source: sound_effects.splash.clone(),
                ..default()
            });
        }
    }

    // fade again after a while
    if *timer >= 4.0 {
        *opacity = (*opacity + time.delta_seconds()).min(1.0);

        if *opacity == 1.0 {
            game_state.set(GameState::MainMenu);
        }
    }

    *fade_element = Color::srgba(0.0, 0.0, 0.0, *opacity).into();
}
