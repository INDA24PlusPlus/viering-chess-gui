pub mod splash;

use bevy::prelude::*;

use crate::{despawn_screen, GameState};

#[derive(Component)]
struct OnSplashScreen;

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), splash::splash_setup)
        .add_systems(
            Update,
            splash::splash_update.run_if(in_state(GameState::Splash)),
        )
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}
