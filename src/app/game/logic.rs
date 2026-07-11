use crate::AppState;
use crate::app::game::enemy::*;
use crate::app::game::player::*;
use crate::app::ui::settings::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::window::PrimaryWindow;
use std::time::Duration;

#[derive(Component)]
pub struct CleanupGame;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            enemy_push
                .run_if(on_timer(Duration::from_secs(2)).and_then(in_state(AppState::InGame))),
        );
    }
}

pub fn spawn_game(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut time: ResMut<Time<Virtual>>,
) {
    time.unpause();

    let window = match q_window.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let window_width = window.width();
    let window_height = window.height();

    //instance of the player
    let player: Player = Player::new(&window_width, &window_height);

    // Camera to render the UI
    commands.spawn((Camera2d, CleanupGame));

    // SPAWN HUD & SETTINGS MODAL OVERLAY
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            CleanupGame,
        ))
        .with_children(|parent| {
            // TOP HUD BAR
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(30.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.09, 0.12, 0.85)),
                ))
                .with_children(|tb| {
                    tb.spawn((
                        Text::new("TOWER DEFENSE"),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.95)),
                    ));

                    // settings toggle button
                    tb.spawn((
                        Button,
                        Node {
                            width: Val::Px(120.0),
                            height: Val::Px(36.0),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            border_radius: BorderRadius::all(Val::Px(6.0)),
                            ..default()
                        },
                        Interaction::None,
                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                        BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                        SettingsButton::OpenSettings,
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("SETTINGS"),
                            TextFont {
                                font_size: FontSize::Px(13.0),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
                });

            // SETTINGS OVERLAY MODAL (Hidden by default)
            parent
                .spawn((
                    Node {
                        display: Display::None,
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                    InGameSettingsModal,
                ))
                .with_children(|overlay| {
                    overlay
                        .spawn((
                            Node {
                                width: Val::Px(450.0),
                                height: Val::Px(460.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(30.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                border_radius: BorderRadius::all(Val::Px(12.0)),
                                row_gap: Val::Px(8.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.08, 0.09, 0.12)),
                            BorderColor::all(Color::srgb(0.5, 0.3, 0.9)), // Purple border accent
                        ))
                        .with_children(|modal| {
                            modal.spawn((
                                Text::new("GAME SETTINGS"),
                                TextFont {
                                    font_size: FontSize::Px(26.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.6, 0.4, 1.0)),
                            ));

                            // REUSABLE SETTINGS SECTION
                            modal
                                .spawn((Node {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    row_gap: Val::Px(12.0),
                                    ..default()
                                },))
                                .with_children(|s_parent| {
                                    spawn_settings_ui(s_parent);
                                });

                            // MODAL BACK TO GAME / RETURN TO LAUNCHER
                            modal
                                .spawn((Node {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    row_gap: Val::Px(10.0),
                                    ..default()
                                },))
                                .with_children(|row| {
                                    row.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(220.0),
                                            height: Val::Px(42.0),
                                            border: UiRect::all(Val::Px(2.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border_radius: BorderRadius::all(Val::Px(6.0)),
                                            ..default()
                                        },
                                        Interaction::None,
                                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                                        BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                                        SettingsButton::BackToGame,
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new("BACK TO SCENE"),
                                            TextFont {
                                                font_size: FontSize::Px(14.0),
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });

                                    row.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(220.0),
                                            height: Val::Px(36.0),
                                            border: UiRect::all(Val::Px(2.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border_radius: BorderRadius::all(Val::Px(6.0)),
                                            ..default()
                                        },
                                        Interaction::None,
                                        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                                        BorderColor::all(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                                        SettingsButton::ReturnToMenu,
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new("RETURN TO LAUNCHER"),
                                            TextFont {
                                                font_size: FontSize::Px(13.0),
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                                });
                        });
                });
        });

    // SPAWN PLAYER
    Player::spawn_player(&mut commands, player);
}

pub fn despawn_game(mut commands: Commands, query: Query<Entity, With<CleanupGame>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// In-game key inputs (Escape to toggle settings)
pub fn in_game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut show_settings: ResMut<ShowInGameSettings>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        show_settings.0 = !show_settings.0;
    }
}

//Automatic player shooting
pub fn player_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut q_player: Query<(&Transform, &mut Player)>,
    mut q_enemies: Query<(Entity, &Transform, &mut Enemy)>,
) {
    // Only shoot if the player is spawned
    let (player_transform, mut player) = match q_player.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    // If there are no enemies, do not shoot or tick reload timer
    if q_enemies.is_empty() {
        return;
    }

    // Tick the player's reload timer
    player.reload.tick(time.delta());

    // When the reload timer finishes, shoot the nearest enemy
    if player.reload.is_finished() {
        let player_pos = player_transform.translation;
        let mut nearest_enemy: Option<(Entity, f32)> = None;

        for (enemy_entity, enemy_transform, _) in &q_enemies {
            let distance = player_pos.distance(enemy_transform.translation);
            if let Some((_, min_dist)) = nearest_enemy {
                if distance < min_dist {
                    nearest_enemy = Some((enemy_entity, distance));
                }
            } else {
                nearest_enemy = Some((enemy_entity, distance));
            }
        }

        // Apply damage to the nearest enemy
        if let Some((enemy_entity, _)) = nearest_enemy
            && let Ok((_, _, mut enemy)) = q_enemies.get_mut(enemy_entity)
        {
            if enemy.hp > 0 {
                //TODO: change value depend on the player damage
                enemy.hp -= player.damage as i32;
                // If enemy dies, despawn it (Bevy 0.19 automatically handles hierarchy cleanup)
                if enemy.hp <= 0 {
                    commands.entity(enemy_entity).despawn();
                }
            }
            // Reset reload timer for the next shot
            player.reload.reset();
        }
    }
}

//Spawning enemy rows
pub fn enemy_push(mut commands: Commands, q_window: Query<&Window, With<PrimaryWindow>>) {
    let window = match q_window.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let window_width = window.width();
    let window_height = window.height();

    let enemy = Enemy::new(&window_width, &window_height);
    Enemy::spawn_enemy(&mut commands, enemy);
}
