pub mod components;
use components::*;

pub mod resources;
use resources::*;

pub mod networking;

mod systems;
use systems::{board, input, resource_setup, setup};

mod utils;
use utils::*;

mod game_ui;

use bevy::prelude::*;

use crate::{despawn_screen, GameState};

#[derive(Component)]
struct OnGameScreen;

pub fn game_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::InGame),
        (
            resource_setup::setup,
            setup::setup_game_scene.after(resource_setup::setup),
            game_ui::setup_ui.after(resource_setup::setup),
        ),
    )
    .add_systems(
        Update,
        (
            input::handle_picking.run_if(in_state(GameState::InGame)),
            game_ui::update_ui.run_if(in_state(GameState::InGame)),
            game_ui::promotion_menu_action.run_if(in_state(GameState::InGame)),
            board::update_board.run_if(in_state(GameState::InGame)),
        ),
    )
    .insert_resource(ClearColor(Color::srgb_u8(77, 79, 84)))
    .add_systems(OnExit(GameState::InGame), despawn_screen::<OnGameScreen>);
}
