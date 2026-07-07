use crate::AppState;
use bevy::prelude::*;
use bevy::window::{MonitorSelection, PrimaryWindow, WindowMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionOption {
    HD,     // 1280x720
    FullHD, // 1920x1080
    QHD,    // 2560x1440
}

impl ResolutionOption {
    pub fn dimensions(&self) -> (f32, f32) {
        match self {
            ResolutionOption::HD => (1280.0, 720.0),
            ResolutionOption::FullHD => (1920.0, 1080.0),
            ResolutionOption::QHD => (2560.0, 1440.0),
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct GameSettings {
    pub fullscreen: bool,
    pub resolution: ResolutionOption,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fullscreen: false,
            resolution: ResolutionOption::HD,
        }
    }
}

// Marker components for state cleanup
#[derive(Component)]
pub struct CleanupLauncher;

// Resource to track whether the in-game settings menu is currently shown
#[derive(Resource, Default)]
pub struct ShowInGameSettings(pub bool);

// Interactive buttons enum
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SettingsButton {
    FullscreenToggle(bool),
    ResolutionSelect(ResolutionOption),
    StartGame,
    ExitGame,
    OpenSettings,
    BackToGame,
    ReturnToMenu,
}

// Marker for in-game settings modal container
#[derive(Component)]
pub struct InGameSettingsModal;

// Setup Camera and UI for launcher
pub fn spawn_launcher_ui(mut commands: Commands) {
    commands.spawn((Camera2d, CleanupLauncher));

    // Root UI container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.09, 0.12)),
            CleanupLauncher,
        ))
        .with_children(|parent| {
            // HEADER
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },))
                .with_children(|h_parent| {
                    h_parent.spawn((
                        Text::new("TOWER DEFENSE"),
                        TextFont {
                            font_size: FontSize::Px(40.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.4, 0.7, 1.0)),
                    ));
                    h_parent.spawn((
                        Text::new("LAUNCHER"),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.6, 0.6, 0.7)),
                    ));
                });

            // REUSABLE SETTINGS SECTION
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(24.0),
                    ..default()
                },))
                .with_children(|s_parent| {
                    spawn_settings_ui(s_parent);
                });

            // ACTION BUTTONS
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(12.0),
                    ..default()
                },))
                .with_children(|a_parent| {
                    // Start Game Button
                    a_parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(250.0),
                                height: Val::Px(50.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(8.0)),
                                ..default()
                            },
                            Interaction::None,
                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                            BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                            SettingsButton::StartGame,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("START GAME"),
                                TextFont {
                                    font_size: FontSize::Px(18.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Exit Game Button
                    a_parent
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(35.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                ..default()
                            },
                            Interaction::None,
                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                            BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                            SettingsButton::ExitGame,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("EXIT"),
                                TextFont {
                                    font_size: FontSize::Px(13.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub fn despawn_launcher_ui(mut commands: Commands, query: Query<Entity, With<CleanupLauncher>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// Update the visibility of the settings overlay modal dynamically
pub fn toggle_settings_visibility(
    show_settings: Res<ShowInGameSettings>,
    mut query: Query<&mut Node, With<InGameSettingsModal>>,
) {
    if show_settings.is_changed()
        && let Ok(mut node) = query.single_mut()
    {
        node.display = if show_settings.0 {
            //TODO: Stop the game time
            Display::Flex
        } else {
            Display::None
        };
    }
}

// Reusable UI Builder helper for the screen mode and resolution selection settings
pub fn spawn_settings_ui(parent: &mut ChildSpawnerCommands) {
    // SCREEN MODE ROW
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
            ..default()
        },))
        .with_children(|row| {
            row.spawn((
                Text::new("SCREEN MODE"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
            ));
            row.spawn((Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },))
                .with_children(|btn_row| {
                    // Windowed option
                    btn_row
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(40.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                ..default()
                            },
                            Interaction::None,
                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                            BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                            SettingsButton::FullscreenToggle(false),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("WINDOWED"),
                                TextFont {
                                    font_size: FontSize::Px(13.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Fullscreen option
                    btn_row
                        .spawn((
                            Button,
                            Node {
                                width: Val::Px(150.0),
                                height: Val::Px(40.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(6.0)),
                                ..default()
                            },
                            Interaction::None,
                            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                            BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                            SettingsButton::FullscreenToggle(true),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("FULLSCREEN"),
                                TextFont {
                                    font_size: FontSize::Px(13.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });

    // RESOLUTION ROW
    parent
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0),
            ..default()
        },))
        .with_children(|row| {
            row.spawn((
                Text::new("RESOLUTION"),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.6)),
            ));
            row.spawn((Node {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(10.0),
                ..default()
            },))
                .with_children(|btn_row| {
                    let options = [
                        (ResolutionOption::HD, "1280x720 (HD)"),
                        (ResolutionOption::FullHD, "1920x1080 (FHD)"),
                        (ResolutionOption::QHD, "2560x1440 (QHD)"),
                    ];
                    for (opt, label) in options {
                        btn_row
                            .spawn((
                                Button,
                                Node {
                                    width: Val::Px(130.0),
                                    height: Val::Px(40.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    border_radius: BorderRadius::all(Val::Px(6.0)),
                                    ..default()
                                },
                                Interaction::None,
                                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                                BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                                SettingsButton::ResolutionSelect(opt),
                            ))
                            .with_children(|btn| {
                                btn.spawn((
                                    Text::new(label),
                                    TextFont {
                                        font_size: FontSize::Px(12.0),
                                        ..default()
                                    },
                                    TextColor(Color::WHITE),
                                ));
                            });
                    }
                });
        });
}

type SettingsButtonComponents = (
    &'static Interaction,
    &'static SettingsButton,
    &'static mut BackgroundColor,
    &'static mut BorderColor,
    &'static Children,
);

// System to change background, border, and text colors based on selected options and mouse interactions
pub fn update_launcher_button_colors(
    settings: Res<GameSettings>,
    mut q_buttons: Query<SettingsButtonComponents, With<Button>>,
    mut q_children: Query<&mut TextColor>,
) {
    for (interaction, button_type, mut bg_color, mut border_color, children) in &mut q_buttons {
        // Check if this button represents the active setting
        let is_selected = match button_type {
            SettingsButton::FullscreenToggle(val) => settings.fullscreen == *val,
            SettingsButton::ResolutionSelect(res) => settings.resolution == *res,
            _ => false,
        };

        // Action buttons (Start, Exit, settings toggles) have separate styling
        let is_action = matches!(
            button_type,
            SettingsButton::StartGame
                | SettingsButton::ExitGame
                | SettingsButton::OpenSettings
                | SettingsButton::BackToGame
                | SettingsButton::ReturnToMenu
        );

        let (bg, border, text_col) = if is_action {
            match button_type {
                SettingsButton::StartGame => match interaction {
                    Interaction::Pressed => (
                        Color::srgb(0.1, 0.6, 0.4),
                        Color::srgb(0.2, 0.8, 0.5),
                        Color::srgb(1.0, 1.0, 1.0),
                    ),
                    Interaction::Hovered => (
                        Color::srgb(0.18, 0.7, 0.48),
                        Color::srgb(0.35, 0.9, 0.65),
                        Color::srgb(1.0, 1.0, 1.0),
                    ),
                    Interaction::None => (
                        Color::srgb(0.15, 0.65, 0.45),
                        Color::srgb(0.3, 0.8, 0.6),
                        Color::srgb(0.9, 0.9, 0.95),
                    ),
                },
                SettingsButton::ExitGame | SettingsButton::ReturnToMenu => match interaction {
                    Interaction::Pressed => (
                        Color::srgb(0.5, 0.15, 0.15),
                        Color::srgb(0.8, 0.3, 0.3),
                        Color::srgb(1.0, 1.0, 1.0),
                    ),
                    Interaction::Hovered => (
                        Color::srgb(0.65, 0.2, 0.2),
                        Color::srgb(0.9, 0.4, 0.4),
                        Color::srgb(1.0, 1.0, 1.0),
                    ),
                    Interaction::None => (
                        Color::srgb(0.4, 0.15, 0.15),
                        Color::srgb(0.6, 0.2, 0.2),
                        Color::srgb(0.85, 0.85, 0.85),
                    ),
                },
                _ => match interaction {
                    // OpenSettings, BackToGame
                    Interaction::Pressed => (
                        Color::srgb(0.3, 0.3, 0.35),
                        Color::srgb(0.5, 0.5, 0.6),
                        Color::srgb(1.0, 1.0, 1.0),
                    ),
                    Interaction::Hovered => (
                        Color::srgb(0.25, 0.25, 0.3),
                        Color::srgb(0.4, 0.4, 0.5),
                        Color::srgb(1.0, 1.0, 1.0),
                    ),
                    Interaction::None => (
                        Color::srgb(0.18, 0.18, 0.22),
                        Color::srgb(0.3, 0.3, 0.35),
                        Color::srgb(0.8, 0.8, 0.8),
                    ),
                },
            }
        } else if is_selected {
            // Selected Option Button
            match interaction {
                Interaction::Pressed => (
                    Color::srgb(0.15, 0.45, 0.85),
                    Color::srgb(0.3, 0.7, 1.0),
                    Color::srgb(1.0, 1.0, 1.0),
                ),
                Interaction::Hovered => (
                    Color::srgb(0.2, 0.5, 0.9),
                    Color::srgb(0.4, 0.8, 1.0),
                    Color::srgb(1.0, 1.0, 1.0),
                ),
                Interaction::None => (
                    Color::srgb(0.1, 0.4, 0.8),
                    Color::srgb(0.2, 0.6, 0.9),
                    Color::srgb(1.0, 1.0, 1.0),
                ),
            }
        } else {
            // Unselected Option Button
            match interaction {
                Interaction::Pressed => (
                    Color::srgb(0.25, 0.26, 0.3),
                    Color::srgb(0.4, 0.4, 0.45),
                    Color::srgb(0.9, 0.9, 0.9),
                ),
                Interaction::Hovered => (
                    Color::srgb(0.2, 0.21, 0.25),
                    Color::srgb(0.3, 0.3, 0.35),
                    Color::srgb(0.8, 0.8, 0.8),
                ),
                Interaction::None => (
                    Color::srgb(0.12, 0.13, 0.16),
                    Color::srgb(0.2, 0.21, 0.25),
                    Color::srgb(0.6, 0.6, 0.7),
                ),
            }
        };

        *bg_color = BackgroundColor(bg);
        *border_color = BorderColor::all(border);

        // Update the button's nested text color
        for child in children.iter() {
            if let Ok(mut text_color) = q_children.get_mut(child) {
                *text_color = TextColor(text_col);
            }
        }
    }
}

type ButtonInteractionComponents = (&'static Interaction, &'static SettingsButton);
type ButtonInteractionFilter = (Changed<Interaction>, With<Button>);

// System to handle mouse click inputs on UI buttons
pub fn handle_button_clicks(
    mut interaction_query: Query<ButtonInteractionComponents, ButtonInteractionFilter>,
    mut settings: ResMut<GameSettings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut show_in_game_settings: ResMut<ShowInGameSettings>,
    mut app_exit_events: MessageWriter<AppExit>,
    current_state: Res<State<AppState>>,
) {
    for (interaction, button_action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                SettingsButton::FullscreenToggle(val) => {
                    settings.fullscreen = *val;
                    // If in-game, apply settings immediately
                    if current_state.get() == &AppState::InGame
                        && let Ok(mut window) = q_window.single_mut()
                    {
                        window.mode = if *val {
                            WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
                        } else {
                            WindowMode::Windowed
                        };
                    }
                }
                SettingsButton::ResolutionSelect(res) => {
                    settings.resolution = *res;
                    // If in-game, apply settings immediately
                    if current_state.get() == &AppState::InGame
                        && let Ok(mut window) = q_window.single_mut()
                    {
                        let (w, h) = res.dimensions();
                        window.resolution.set(w, h);
                    }
                }
                SettingsButton::StartGame => {
                    // Apply selected settings when starting
                    if let Ok(mut window) = q_window.single_mut() {
                        let (w, h) = settings.resolution.dimensions();
                        window.resolution.set(w, h);
                        window.mode = if settings.fullscreen {
                            WindowMode::BorderlessFullscreen(MonitorSelection::Primary)
                        } else {
                            WindowMode::Windowed
                        };
                        window.title = "Tower Defense".to_string();
                    }
                    next_state.set(AppState::InGame);
                }
                SettingsButton::ExitGame => {
                    app_exit_events.write(AppExit::Success);
                }
                SettingsButton::OpenSettings => {
                    show_in_game_settings.0 = true;
                }
                SettingsButton::BackToGame => {
                    show_in_game_settings.0 = false;
                }
                SettingsButton::ReturnToMenu => {
                    // Save the settings, but reset the window
                    if let Ok(mut window) = q_window.single_mut() {
                        window.title = "Game Launcher".to_string();
                    }
                    show_in_game_settings.0 = false;
                    next_state.set(AppState::StartupMenu);
                }
            }
        }
    }
}
