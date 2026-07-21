use crate::app::game::logic::CoinCount;
use crate::app::game::player::Player;
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
            damage_cost: 5,
            reload_cost: 5,
            multishot_cost: 50,
        }
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
    mut q_player: Query<&mut Player>,
    mut coin_count: ResMut<CoinCount>,
    mut upgrade_costs: ResMut<UpgradeCosts>,
) {
    for (interaction, button_action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                UpgradeButton::OpenUpgrades => {
                    show_upgrade.0 = true;
                }
                UpgradeButton::Close => {
                    show_upgrade.0 = false;
                }
                UpgradeButton::UpgradeDamage => {
                    if coin_count.0 >= upgrade_costs.damage_cost
                        && let Ok(mut player) = q_player.single_mut()
                    {
                        coin_count.0 -= upgrade_costs.damage_cost;
                        player.damage += 1;
                        upgrade_costs.damage_cost += 2;
                    }
                }
                UpgradeButton::UpgradeReloadSpeed => {
                    if coin_count.0 >= upgrade_costs.reload_cost
                        && let Ok(mut player) = q_player.single_mut()
                    {
                        let current_duration = player.reload.duration().as_secs_f32();
                        if current_duration > 0.15 {
                            coin_count.0 -= upgrade_costs.reload_cost;
                            let new_duration = (current_duration - 0.1).max(0.1);
                            player
                                .reload
                                .set_duration(std::time::Duration::from_secs_f32(new_duration));
                            upgrade_costs.reload_cost += 2;
                        }
                    }
                }
                UpgradeButton::AbilityMultishot => {
                    if coin_count.0 >= upgrade_costs.multishot_cost
                        && let Ok(mut player) = q_player.single_mut()
                        && player.multishot == false
                    {
                        coin_count.0 -= upgrade_costs.multishot_cost;
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

// System to update dynamic text in the upgrade menu
pub fn update_upgrade_menu_text(
    upgrade_costs: Res<UpgradeCosts>,
    q_player: Query<&Player>,
    mut q_damage_text: Query<&mut Text, (With<DamageText>, Without<ReloadText>)>,
    mut q_reload_text: Query<&mut Text, (With<ReloadText>, Without<DamageText>)>,
    mut q_multishot_btn_text: Query<&mut Text, MultishotBtnTextFilter>,
    mut q_damage_btn_text: Query<&mut Text, DamageBtnTextFilter>,
    mut q_reload_btn_text: Query<&mut Text, ReloadBtnTextFilter>,
) {
    if let Ok(player) = q_player.single() {
        for mut text in &mut q_damage_text {
            let val = format!("DAMAGE: {}", player.damage);
            if text.0 != val {
                text.0 = val;
            }
        }
        for mut text in &mut q_reload_text {
            let secs = player.reload.duration().as_secs_f32();
            let val = format!("RELOAD: {:.1}s", secs);
            if text.0 != val {
                text.0 = val;
            }
        }
        for mut text in &mut q_multishot_btn_text {
            let val = format!("MultiShot is active");
            if text.0 != val && player.multishot == true {
                text.0 = val;
            }
        }
    }
    if upgrade_costs.is_changed() {
        for mut text in &mut q_damage_btn_text {
            let val = format!("+1 ({} COINS)", upgrade_costs.damage_cost);
            if text.0 != val {
                text.0 = val;
            }
        }
        for mut text in &mut q_reload_btn_text {
            let val = format!("-0.1s ({} COINS)", upgrade_costs.reload_cost);
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
            time.unpause();
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
                                            Text::new("+1 (5 COINS)"),
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
                                            Text::new("-0.1s (5 COINS)"),
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
                                            Text::new("Unlock MultiShot: 50 COINS"),
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
