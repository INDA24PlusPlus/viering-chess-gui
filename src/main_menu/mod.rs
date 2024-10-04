pub mod main_menu;

use bevy::prelude::*;

use crate::{despawn_screen, GameState};

#[derive(Component)]
struct OnMainMenuScreen;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), main_menu::menu_setup)
        .add_systems(
            Update,
            main_menu::menu_update.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            despawn_screen::<OnMainMenuScreen>,
        );
}
