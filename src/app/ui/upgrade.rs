use bevy::prelude::*;

// Resource to track whether the upgrade menu is currently shown
#[derive(Resource, Default)]
pub struct ShowUpgrade(pub bool);

// Marker for in-game upgrades modal container
#[derive(Component)]
pub struct InGameUpgradesModal;

// Interactive upgrade buttons enum
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpgradeButton {
    OpenUpgrades,
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
                        Text::new("UPGRADES"),
                        TextFont {
                            font_size: FontSize::Px(26.0),
                            ..default()
                        },
                        TextColor(Color::srgb(0.2, 0.7, 0.9)),
                    ));

                    // Reusable container panel for upgrade items
                    modal.spawn((Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(12.0),
                        ..default()
                    },));

                    // Close Button
                    modal.spawn((
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
