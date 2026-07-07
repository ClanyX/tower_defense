use bevy::prelude::*;

use crate::app::game::logic::*;
use crate::app::ui::settings::*;

mod app;

#[derive(States, Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    StartupMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game Launcher".to_string(),
                resizable: false, // Keep fixed size
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .insert_resource(GameSettings::default())
        .insert_resource(ShowInGameSettings::default())
        // Startup launcher setup / cleanup
        .add_systems(OnEnter(AppState::StartupMenu), spawn_launcher_ui)
        .add_systems(OnExit(AppState::StartupMenu), despawn_launcher_ui)
        // Gameplay setups
        .add_systems(OnEnter(AppState::InGame), spawn_game)
        .add_systems(OnExit(AppState::InGame), despawn_game)
        // Global systems
        .add_systems(
            Update,
            (update_launcher_button_colors, handle_button_clicks),
        )
        // In-game updates
        .add_systems(
            Update,
            (in_game_input, toggle_settings_visibility).run_if(in_state(AppState::InGame)),
        )
        .run();
}
