use bevy::prelude::*;
use bevy_mod_outline::*;
use bevy_mod_picking::*;

pub mod components;
use components::*;

pub mod resources;
use resources::*;

mod systems;
use systems::{input, resource_setup, setup};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, OutlinePlugin, DefaultPickingPlugins))
        .add_systems(Startup, resource_setup::setup)
        .add_systems(PostStartup, setup::setup)
        .add_systems(Update, input::handle_picking)
        .run();
}
