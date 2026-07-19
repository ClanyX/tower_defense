use bevy::prelude::*;

use crate::app::game::enemy::{move_enemy, update_enemy_hp_text};
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
        .add_plugins(GamePlugin)
        .init_state::<AppState>()
        .insert_resource(GameSettings::default())
        .insert_resource(ShowInGameSettings::default())
        .insert_resource(ShowUpgrade::default())
        .insert_resource(KillCount(0))
        .insert_resource(CoinCount(0))
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
            (
                in_game_input,
                toggle_settings_visibility,
                move_enemy,
                update_enemy_hp_text,
                player_shooting,
                update_kill_count_text,
                update_coin_count_text,
                update_tower_hp_text,
                check_game_over,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .run();
}
