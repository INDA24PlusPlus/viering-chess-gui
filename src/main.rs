use bevy::prelude::*;
use bevy_mod_outline::*;
use bevy_mod_picking::*;

pub mod components;
use components::*;

pub mod resources;
use resources::*;

mod systems;
use systems::{input, resource_setup, setup::setup_game_scene};

mod utils;
use utils::*;

mod ui;
use ui::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, OutlinePlugin, DefaultPickingPlugins))
        .add_systems(Startup, resource_setup::setup)
        .add_systems(PostStartup, (setup_game_scene, setup_ui))
        .add_systems(Update, (input::handle_picking, update_ui))
        .run();
}
