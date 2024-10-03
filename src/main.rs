use bevy::prelude::*;
use bevy_mod_outline::*;
use bevy_mod_picking::*;
use bevy_simple_text_input::*;

mod game;

mod general;
use general::resources::SoundEffects;

mod main_menu;
mod splash;

// warning code is a mess, first time using bevy so everything is a mess, also networking lib and
// my gui game structure didn't work too well together meaning even more spaghetti :D

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
        .run();
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
