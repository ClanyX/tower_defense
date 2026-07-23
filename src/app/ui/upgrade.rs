use crate::app::game::logic::{CoinCount, KillCount};
use crate::app::game::player::Player;
use crate::app::ui::settings::ShowInGameSettings;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

// Resource to track dynamic upgrade costs
#[derive(Resource)]
pub struct UpgradeCosts {
    pub damage_cost: u32,
    pub reload_cost: u32,
    pub multishot_cost: u32,
}

impl Default for UpgradeCosts {
    fn default() -> Self {
        Self {
            damage_cost: 2,
            reload_cost: 2,
            multishot_cost: 25,
        }
    }
}

impl UpgradeCosts {
    pub fn get_damage_cost(&self, score: u32) -> u32 {
        let late_game_scaling = (score as f32 * 0.15).powi(2) as u32;
        self.damage_cost + late_game_scaling + (score / 3)
    }

    pub fn get_reload_cost(&self, score: u32) -> u32 {
        let late_game_scaling = (score as f32 * 0.15).powi(2) as u32;
        self.reload_cost + late_game_scaling + (score / 3)
    }

    pub fn get_multishot_cost(&self, score: u32) -> u32 {
        let late_game_scaling = (score as f32 * 0.2).powi(2) as u32;
        self.multishot_cost + late_game_scaling + score
    }
}

// Resource to track whether the upgrade menu is currently shown
#[derive(Resource, Default)]
pub struct ShowUpgrade(pub bool);

// Marker for in-game upgrades modal container
#[derive(Component)]
pub struct InGameUpgradesModal;

// Markers for dynamic text fields in upgrade menu
#[derive(Component)]
pub struct DamageText;

#[derive(Component)]
pub struct ReloadText;

#[derive(Component)]
pub struct MultishotText;

#[derive(Component)]
pub struct DamageButtonText;

#[derive(Component)]
pub struct ReloadButtonText;

#[derive(Component)]
pub struct MultishotButtonText;

// Interactive upgrade buttons enum
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpgradeButton {
    OpenUpgrades,
    UpgradeDamage,
    UpgradeReloadSpeed,
    AbilityMultishot,
    Close,
}

type UpgradeInteractionComponents = (&'static Interaction, &'static UpgradeButton);
type UpgradeInteractionFilter = (Changed<Interaction>, With<Button>);

type UpgradeColorComponents = (
    &'static Interaction,
    &'static UpgradeButton,
    &'static mut BackgroundColor,
    &'static mut BorderColor,
    &'static Children,
);
type UpgradeColorFilter = (Changed<Interaction>, With<Button>);

// System to handle button clicks inside/for the Upgrade Menu
pub fn handle_upgrade_button_clicks(
    mut interaction_query: Query<UpgradeInteractionComponents, UpgradeInteractionFilter>,
    mut show_upgrade: ResMut<ShowUpgrade>,
    mut show_settings: ResMut<ShowInGameSettings>,
    mut q_player: Query<&mut Player>,
    mut coin_count: ResMut<CoinCount>,
    mut upgrade_costs: ResMut<UpgradeCosts>,
    kill_count: Res<KillCount>,
) {
    let score = kill_count.0;
    for (interaction, button_action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                UpgradeButton::OpenUpgrades => {
                    show_upgrade.0 = !show_upgrade.0;
                    if show_upgrade.0 {
                        show_settings.0 = false;
                    }
                }
                UpgradeButton::Close => {
                    show_upgrade.0 = false;
                }
                UpgradeButton::UpgradeDamage => {
                    let cost = upgrade_costs.get_damage_cost(score);
                    if coin_count.0 >= cost
                        && let Ok(mut player) = q_player.single_mut()
                    {
                        coin_count.0 -= cost;
                        player.damage += 1;
                        upgrade_costs.damage_cost += 2;
                    }
                }
                UpgradeButton::UpgradeReloadSpeed => {
                    let cost = upgrade_costs.get_reload_cost(score);
                    if coin_count.0 >= cost
                        && let Ok(mut player) = q_player.single_mut()
                    {
                        let current_duration = player.reload.duration().as_secs_f32();
                        if current_duration > 0.15 {
                            coin_count.0 -= cost;
                            let new_duration = (current_duration - 0.1).max(0.1);
                            player
                                .reload
                                .set_duration(std::time::Duration::from_secs_f32(new_duration));
                            upgrade_costs.reload_cost += 2;
                        }
                    }
                }
                UpgradeButton::AbilityMultishot => {
                    let cost = upgrade_costs.get_multishot_cost(score);
                    if coin_count.0 >= cost
                        && let Ok(mut player) = q_player.single_mut()
                        && !player.multishot
                    {
                        coin_count.0 -= cost;
                        player.multishot = true;
                    }
                }
            }
        }
    }
}

type DamageBtnTextFilter = (
    With<DamageButtonText>,
    Without<ReloadButtonText>,
    Without<MultishotButtonText>,
    Without<DamageText>,
    Without<ReloadText>,
);

type ReloadBtnTextFilter = (
    With<ReloadButtonText>,
    Without<DamageButtonText>,
    Without<MultishotButtonText>,
    Without<DamageText>,
    Without<ReloadText>,
);

type MultishotBtnTextFilter = (
    With<MultishotButtonText>,
    Without<ReloadButtonText>,
    Without<DamageButtonText>,
    Without<DamageText>,
    Without<ReloadText>,
);

#[derive(SystemParam)]
pub struct UpgradeMenuTextQueries<'w, 's> {
    pub upgrade_costs: Res<'w, UpgradeCosts>,
    pub kill_count: Res<'w, KillCount>,
    pub q_player: Query<'w, 's, &'static Player>,
    pub q_damage_text: Query<'w, 's, &'static mut Text, (With<DamageText>, Without<ReloadText>)>,
    pub q_reload_text: Query<'w, 's, &'static mut Text, (With<ReloadText>, Without<DamageText>)>,
    pub q_multishot_btn_text: Query<'w, 's, &'static mut Text, MultishotBtnTextFilter>,
    pub q_damage_btn_text: Query<'w, 's, &'static mut Text, DamageBtnTextFilter>,
    pub q_reload_btn_text: Query<'w, 's, &'static mut Text, ReloadBtnTextFilter>,
}

// System to update dynamic text in the upgrade menu
pub fn update_upgrade_menu_text(mut queries: UpgradeMenuTextQueries) {
    let score = queries.kill_count.0;
    if let Ok(player) = queries.q_player.single() {
        for mut text in &mut queries.q_damage_text {
            let val = format!("DAMAGE: {}", player.damage);
            if text.0 != val {
                text.0 = val;
            }
        }
        for mut text in &mut queries.q_reload_text {
            let secs = player.reload.duration().as_secs_f32();
            let val = format!("RELOAD: {:.1}s", secs);
            if text.0 != val {
                text.0 = val;
            }
        }
        for mut text in &mut queries.q_multishot_btn_text {
            if player.multishot {
                let val = "MultiShot is active".to_string();
                if text.0 != val {
                    text.0 = val;
                }
            } else {
                let cost = queries.upgrade_costs.get_multishot_cost(score);
                let val = format!("Unlock MultiShot: {} COINS", cost);
                if text.0 != val {
                    text.0 = val;
                }
            }
        }
    }
    if queries.upgrade_costs.is_changed() || queries.kill_count.is_changed() {
        for mut text in &mut queries.q_damage_btn_text {
            let cost = queries.upgrade_costs.get_damage_cost(score);
            let val = format!("+1 ({} COINS)", cost);
            if text.0 != val {
                text.0 = val;
            }
        }
        for mut text in &mut queries.q_reload_btn_text {
            let cost = queries.upgrade_costs.get_reload_cost(score);
            let val = format!("-0.1s ({} COINS)", cost);
            if text.0 != val {
                text.0 = val;
            }
        }
    }
}

// System to update button colors on Hover/Press for Upgrade Buttons
pub fn update_upgrade_button_colors(
    mut query: Query<UpgradeColorComponents, UpgradeColorFilter>,
    mut q_children: Query<&mut TextColor>,
) {
    for (interaction, button_type, mut bg_color, mut border_color, children) in &mut query {
        let (bg, border, text_col) = match button_type {
            UpgradeButton::Close => match interaction {
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
            UpgradeButton::OpenUpgrades => match interaction {
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
            UpgradeButton::UpgradeDamage
            | UpgradeButton::UpgradeReloadSpeed
            | UpgradeButton::AbilityMultishot => match interaction {
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
            },
        };

        *bg_color = BackgroundColor(bg);
        *border_color = BorderColor::all(border);

        for child in children.iter() {
            if let Ok(mut text_color) = q_children.get_mut(child) {
                *text_color = TextColor(text_col);
            }
        }
    }
}

// System to toggle visibility of the upgrade menu overlay and pause/unpause time
pub fn toggle_upgrade_visibility(
    show_upgrade: Res<ShowUpgrade>,
    show_settings: Res<ShowInGameSettings>,
    mut query: Query<&mut Node, With<InGameUpgradesModal>>,
    mut time: ResMut<Time<Virtual>>,
) {
    if show_upgrade.is_changed()
        && let Ok(mut node) = query.single_mut()
    {
        if show_upgrade.0 {
            node.display = Display::Flex;
            time.pause();
        } else {
            node.display = Display::None;
            if !show_settings.0 {
                time.unpause();
            }
        }
    }
}

// Spawns the upgrade overlay modal panel (Hidden by default)
pub fn spawn_upgrade_modal(parent: &mut ChildSpawnerCommands) {
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
            InGameUpgradesModal,
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
                    BorderColor::all(Color::srgb(0.2, 0.7, 0.9)), // Cyan accent border
                ))
                .with_children(|modal| {
                    modal.spawn((
                        Text::new("UPGRADES & ABILITY"),
                        TextFont {
                            font_size: FontSize::Px(26.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.2, 0.7, 0.9)),
                    ));

                    // Reusable container panel for upgrade items
                    modal
                        .spawn((Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(16.0),
                            width: Val::Percent(100.0),
                            ..default()
                        },))
                        .with_children(|body| {
                            // UPGRADE 1: DAMAGE
                            body.spawn((Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                padding: UiRect::horizontal(Val::Px(10.0)),
                                ..default()
                            },))
                                .with_children(|row| {
                                    row.spawn((
                                        Text::new("DAMAGE: 2"),
                                        TextFont {
                                            font_size: FontSize::Px(16.0),
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 0.95)),
                                        DamageText,
                                    ));

                                    row.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(160.0),
                                            height: Val::Px(36.0),
                                            border: UiRect::all(Val::Px(2.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border_radius: BorderRadius::all(Val::Px(6.0)),
                                            ..default()
                                        },
                                        Interaction::None,
                                        BackgroundColor(Color::srgb(0.1, 0.4, 0.8)),
                                        BorderColor::all(Color::srgb(0.2, 0.6, 0.9)),
                                        UpgradeButton::UpgradeDamage,
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new("+1 (2 COINS)"),
                                            TextFont {
                                                font_size: FontSize::Px(13.0),
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                            DamageButtonText,
                                        ));
                                    });
                                });

                            // UPGRADE 2: RELOAD SPEED
                            body.spawn((Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                padding: UiRect::horizontal(Val::Px(10.0)),
                                ..default()
                            },))
                                .with_children(|row| {
                                    row.spawn((
                                        Text::new("RELOAD: 1.0s"),
                                        TextFont {
                                            font_size: FontSize::Px(16.0),
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 0.95)),
                                        ReloadText,
                                    ));

                                    row.spawn((
                                        Button,
                                        Node {
                                            width: Val::Px(160.0),
                                            height: Val::Px(36.0),
                                            border: UiRect::all(Val::Px(2.0)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            border_radius: BorderRadius::all(Val::Px(6.0)),
                                            ..default()
                                        },
                                        Interaction::None,
                                        BackgroundColor(Color::srgb(0.1, 0.4, 0.8)),
                                        BorderColor::all(Color::srgb(0.2, 0.6, 0.9)),
                                        UpgradeButton::UpgradeReloadSpeed,
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new("-0.1s (2 COINS)"),
                                            TextFont {
                                                font_size: FontSize::Px(13.0),
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                            ReloadButtonText,
                                        ));
                                    });
                                });
                            // ABILITY 1: MULTISHOT
                            body.spawn((Node {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                padding: UiRect::horizontal(Val::Px(10.0)),
                                ..default()
                            },))
                                .with_children(|row| {
                                    row.spawn((
                                        Text::new("MULTISHOT"),
                                        TextFont {
                                            font_size: FontSize::Px(16.0),
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.9, 0.9, 0.95)),
                                        MultishotText,
                                    ));

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
                                        BackgroundColor(Color::srgb(0.1, 0.4, 0.8)),
                                        BorderColor::all(Color::srgb(0.2, 0.6, 0.9)),
                                        UpgradeButton::AbilityMultishot,
                                    ))
                                    .with_children(|btn| {
                                        btn.spawn((
                                            Text::new("Unlock MultiShot: 25 COINS"),
                                            TextFont {
                                                font_size: FontSize::Px(13.0),
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                            MultishotButtonText,
                                        ));
                                    });
                                });
                        });

                    // Close Button
                    modal
                        .spawn((
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
                            BackgroundColor(Color::srgb(0.4, 0.15, 0.15)),
                            BorderColor::all(Color::srgb(0.6, 0.2, 0.2)),
                            UpgradeButton::Close,
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new("CLOSE"),
                                TextFont {
                                    font_size: FontSize::Px(14.0),
                                    ..default()
                                },
                                TextColor(Color::srgb(0.85, 0.85, 0.85)),
                            ));
                        });
                });
        });
}
