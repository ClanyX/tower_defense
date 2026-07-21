use crate::AppState;
use crate::app::game::enemy::*;
use crate::app::game::player::*;
use crate::app::game::tower::*;
use crate::app::ui::settings::*;
use crate::app::ui::upgrade::*;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::window::PrimaryWindow;
use std::time::Duration;

#[derive(Component)]
pub struct CleanupGame;

#[derive(Component)]
pub struct KillCountText;

#[derive(Resource)]
pub struct KillCount(pub u32);

#[derive(Component)]
pub struct CoinCountText;

#[derive(Resource)]
pub struct CoinCount(pub u32);

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                tower_damage,
                enemy_push.run_if(on_timer(Duration::from_secs(2))),
            )
                .run_if(in_state(AppState::InGame)),
        );
    }
}

pub fn spawn_game(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut time: ResMut<Time<Virtual>>,
) {
    time.unpause();

    // Reset/insert the kill/coin count resources on game start
    commands.insert_resource(KillCount(0));
    commands.insert_resource(CoinCount(0));
    commands.insert_resource(UpgradeCosts::default());

    let window = match q_window.single() {
        Ok(w) => w,
        Err(_) => return,
    };

    let window_width = window.width();
    let window_height = window.height();

    //instance of the player
    let player: Player = Player::new(&window_width);
    //instance of the tower
    let tower: Tower = Tower::new(&window_width, &window_height);

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

                    // KILL COUNT TEXT
                    tb.spawn((
                        Text::new("KILLS: 0"),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.95)),
                        KillCountText,
                    ));

                    // COINS COUNT TEXT
                    tb.spawn((
                        Text::new("COINS: 0"),
                        TextFont {
                            font_size: FontSize::Px(18.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.95)),
                        CoinCountText,
                    ));

                    // upgrade toggle button
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
                        BackgroundColor(Color::srgb(0.18, 0.18, 0.22)),
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.35)),
                        UpgradeButton::OpenUpgrades,
                    ))
                    .with_children(|btn| {
                        btn.spawn((
                            Text::new("UPGRADE"),
                            TextFont {
                                font_size: FontSize::Px(13.0),
                                ..default()
                            },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        ));
                    });

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

            // UPGRADE OVERLAY MODAL (Hidden by default)
            spawn_upgrade_modal(parent);
        });

    // SPAWN TOWER
    Tower::spawn_tower(&mut commands, tower);

    // SPAWN PLAYER
    Player::spawn_player(&mut commands, player);
}

pub fn despawn_game(mut commands: Commands, query: Query<Entity, With<CleanupGame>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

// In-game key inputs (U for upgrades, Escape for settings/close)
pub fn in_game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut show_settings: ResMut<ShowInGameSettings>,
    mut show_upgrade: ResMut<ShowUpgrade>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyU) {
        show_upgrade.0 = !show_upgrade.0;
    }
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if show_upgrade.0 {
            show_upgrade.0 = false;
        } else {
            show_settings.0 = !show_settings.0;
        }
    }
}

//Automatic player shooting
pub fn player_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut q_player: Query<(&Transform, &mut Player)>,
    mut q_enemies: Query<(Entity, &Transform, &mut Enemy)>,
    mut kill_count: ResMut<KillCount>,
    mut coin_count: ResMut<CoinCount>,
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

    // When the reload timer finishes, shoot nearest enemy (or 2 nearest enemies if multishot is active)
    if player.reload.is_finished() {
        let player_pos = player_transform.translation;
        let mut nearest_enemy: Option<(Entity, f32)> = None;
        let mut second_nearest_enemy: Option<(Entity, f32)> = None;

        for (enemy_entity, enemy_transform, _) in &q_enemies {
            let distance = player_pos.distance(enemy_transform.translation);
            if let Some((_, min_dist)) = nearest_enemy {
                if distance < min_dist {
                    second_nearest_enemy = nearest_enemy;
                    nearest_enemy = Some((enemy_entity, distance));
                } else if let Some((_, min_dist2)) = second_nearest_enemy {
                    if distance < min_dist2 {
                        second_nearest_enemy = Some((enemy_entity, distance));
                    }
                } else {
                    second_nearest_enemy = Some((enemy_entity, distance));
                }
            } else {
                nearest_enemy = Some((enemy_entity, distance));
            }
        }

        let mut targets = Vec::new();
        if let Some((e1, _)) = nearest_enemy {
            targets.push((e1, player.damage));
        }
        if player.multishot {
            if let Some((e2, _)) = second_nearest_enemy {
                targets.push((e2, player.damage / 2));
            }
        }

        if !targets.is_empty() {
            for (enemy_entity, damage) in targets {
                if let Ok((_, _, mut enemy)) = q_enemies.get_mut(enemy_entity) {
                    if enemy.hp > 0 {
                        enemy.hp -= damage as i32;
                        if enemy.hp <= 0 {
                            commands.entity(enemy_entity).despawn();
                            kill_count.0 += 1;
                            coin_count.0 += enemy.reward;
                        }
                    }
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

// System to update the KILLS text in the HUD dynamically
pub fn update_kill_count_text(
    kill_count: Res<KillCount>,
    mut q_text: Query<&mut Text, With<KillCountText>>,
) {
    if kill_count.is_changed() {
        for mut text in &mut q_text {
            text.0 = format!("KILLS: {}", kill_count.0);
        }
    }
}

// System to update COINS text in the HUD dynamically
pub fn update_coin_count_text(
    coin_count: Res<CoinCount>,
    mut q_text: Query<&mut Text, With<CoinCountText>>,
) {
    if coin_count.is_changed() {
        for mut text in &mut q_text {
            text.0 = format!("COINS: {}", coin_count.0);
        }
    }
}

//Detect damage to the tower
pub fn tower_damage(
    mut commands: Commands,
    q_enemies: Query<(Entity, &Transform, &Enemy)>,
    mut q_tower: Query<(&Transform, &mut Tower)>,
) {
    //Check if the Tower is spawned
    let (tower_transform, mut tower) = match q_tower.single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let tower_left_edge = tower_transform.translation.x - tower.width / 2.0;

    for (enemy_entity, enemy_transform, enemy) in &q_enemies {
        let enemy_right_edge = enemy_transform.translation.x + enemy.size as f32 / 2.0;
        if enemy_right_edge >= tower_left_edge {
            commands.entity(enemy_entity).despawn();
            if tower.hp > 0 {
                tower.hp -= 1;
            }
        }
    }
}

// System to update the text value on the tower with the current HP of the struct
pub fn update_tower_hp_text(
    q_towers: Query<(&Tower, &Children)>,
    mut q_text: Query<&mut Text2d, With<TowerHpText>>,
) {
    for (tower, children) in &q_towers {
        for &child in children {
            if let Ok(mut text) = q_text.get_mut(child) {
                let hp_str = tower.hp.to_string();
                if text.0 != hp_str {
                    text.0 = hp_str;
                }
            }
        }
    }
}

// System to detect game over condition (tower HP <= 0) and return to launcher
pub fn check_game_over(
    q_tower: Query<&Tower>,
    mut next_state: ResMut<NextState<AppState>>,
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
    mut show_in_game_settings: ResMut<ShowInGameSettings>,
) {
    if let Ok(tower) = q_tower.single()
        && tower.hp <= 0
    {
        if let Ok(mut window) = q_window.single_mut() {
            window.title = "Game Launcher".to_string();
        }
        show_in_game_settings.0 = false;
        next_state.set(AppState::StartupMenu);
    }
}
