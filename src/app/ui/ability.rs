use crate::app::game::enemy::Enemy;
use crate::app::game::logic::{CoinCount, KillCount};
use bevy::prelude::*;

// Damage to all enemy ability
#[derive(Component)]
pub struct DamageAll {
    pub dmg: u32,
    pub reload: Timer,
}

impl Default for DamageAll {
    fn default() -> Self {
        let mut reload = Timer::from_seconds(5.0, TimerMode::Once);
        reload.set_elapsed(std::time::Duration::from_secs(5));
        Self { dmg: 10, reload }
    }
}

pub fn spawn_ability_modal(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                padding: UiRect {
                    left: Val::Px(15.0),
                    bottom: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|container| {
            container
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(14.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.09, 0.12, 0.65)),
                ))
                .with_children(|row| {
                    let abilities = [
                        (
                            "ALL",
                            Color::srgb(0.2, 0.6, 0.9),
                            Color::srgb(0.1, 0.4, 0.8),
                            Some(DamageAll::default()),
                        ),
                        (
                            "NONE",
                            Color::srgb(0.9, 0.5, 0.2),
                            Color::srgb(0.5, 0.25, 0.1),
                            None,
                        ),
                        (
                            "NONE",
                            Color::srgb(0.7, 0.3, 0.9),
                            Color::srgb(0.4, 0.15, 0.5),
                            None,
                        ),
                    ];

                    for (label, border_col, bg_col, dmg_all_opt) in abilities {
                        let mut entity = row.spawn((
                            Button,
                            Node {
                                width: Val::Px(60.0),
                                height: Val::Px(60.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(10.0)),
                                ..default()
                            },
                            Interaction::None,
                            BackgroundColor(bg_col),
                            BorderColor::all(border_col),
                        ));

                        if let Some(dmg_all) = dmg_all_opt {
                            entity.insert(dmg_all);
                        }

                        entity.with_children(|btn| {
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

pub fn handle_ability_button_clicks(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut DamageAll),
        (Changed<Interaction>, With<Button>),
    >,
    mut q_enemies: Query<(Entity, &mut Enemy)>,
    mut kill_count: ResMut<KillCount>,
    mut coin_count: ResMut<CoinCount>,
) {
    for (interaction, mut damage_all) in &mut interaction_query {
        if *interaction == Interaction::Pressed && damage_all.reload.is_finished() {
            let dmg = damage_all.dmg as i32;
            for (enemy_entity, mut enemy) in &mut q_enemies {
                if enemy.hp > 0 {
                    enemy.hp -= dmg;
                    if enemy.hp <= 0 {
                        commands.entity(enemy_entity).despawn();
                        kill_count.0 += 1;
                        coin_count.0 += enemy.reward;
                    }
                }
            }
            damage_all.reload.reset();
        }
    }
}

pub fn tick_ability_cooldowns(
    time: Res<Time>,
    mut query: Query<(&mut DamageAll, &mut BackgroundColor)>,
) {
    for (mut damage_all, mut bg_color) in &mut query {
        damage_all.reload.tick(time.delta());
        if damage_all.reload.is_finished() {
            *bg_color = BackgroundColor(Color::srgb(0.1, 0.4, 0.8));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.2));
        }
    }
}
