use bevy::prelude::*;
use bevy_mod_outline::*;
use bevy_mod_picking::*;
use bevy_simple_text_input::*;

pub mod components;
use components::*;

pub mod resources;
use resources::*;

mod systems;
use systems::{board, input, resource_setup, setup};

mod utils;
use utils::*;

mod game_ui;

mod general;
use general::resources::SoundEffects;

mod main_menu;
mod splash;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug, States, Default)]
enum GameState {
    #[default]
    Splash,
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            OutlinePlugin,
            DefaultPickingPlugins,
            TextInputPlugin,
        ))
        .init_state::<GameState>()
        .add_systems(Startup, general::setup::setup_resources)
        .add_plugins((
            splash::splash_plugin,
            main_menu::menu_plugin,
            game::game_plugin,
        ))
        //.add_systems(Startup, resource_setup::setup)
        //.add_systems(PostStartup, (setup::setup_game_scene, game_ui::setup_ui))
        //.add_systems(
        //    Update,
        //    (
        //        input::handle_picking,
        //        game_ui::update_ui,
        //        game_ui::promotion_menu_action,
        //        board::update_board,
        //    ),
        //)
        //.insert_resource(ClearColor(Color::srgb_u8(77, 79, 84)))
        .run();
}

mod game {
    use bevy::prelude::*;

    use super::GameState;

    pub fn game_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), game_setup)
            .add_systems(Update, game_update);
    }

    fn game_setup() {}

    fn game_update() {}
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
