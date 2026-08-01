use crate::app::game::enemy::{Enemy, Frozen};
use crate::app::game::logic::{CoinCount, KillCount};
use bevy::prelude::*;

// Interactive ability buttons enum
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub enum AbilityButton {
    DamageAll,
    Freeze,
    Coins,
}

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

// Freeze all enemies ability
#[derive(Component)]
pub struct Freeze {
    pub duration: f32,
    pub reload: Timer,
}

impl Default for Freeze {
    fn default() -> Self {
        let mut reload = Timer::from_seconds(8.0, TimerMode::Once);
        reload.set_elapsed(std::time::Duration::from_secs(8));
        Self {
            duration: 3.0,
            reload,
        }
    }
}

// Coins bonus ability
#[derive(Component)]
pub struct CoinBonus {
    pub coins: u32,
    pub reload: Timer,
}

impl Default for CoinBonus {
    fn default() -> Self {
        let mut reload = Timer::from_seconds(10.0, TimerMode::Once);
        reload.set_elapsed(std::time::Duration::from_secs(10));
        Self { coins: 20, reload }
    }
}

#[allow(clippy::type_complexity)]
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
                    let abilities: [(
                        &str,
                        Color,
                        Color,
                        Option<AbilityButton>,
                        Option<DamageAll>,
                        Option<Freeze>,
                        Option<CoinBonus>,
                    ); 3] = [
                        (
                            "DAMAGE",
                            Color::srgb(0.2, 0.6, 0.9),
                            Color::srgb(0.1, 0.4, 0.8),
                            Some(AbilityButton::DamageAll),
                            Some(DamageAll::default()),
                            None,
                            None,
                        ),
                        (
                            "FREEZE",
                            Color::srgb(0.3, 0.8, 0.9),
                            Color::srgb(0.1, 0.4, 0.5),
                            Some(AbilityButton::Freeze),
                            None,
                            Some(Freeze::default()),
                            None,
                        ),
                        (
                            "COINS",
                            Color::srgb(0.9, 0.7, 0.2),
                            Color::srgb(0.5, 0.35, 0.1),
                            Some(AbilityButton::Coins),
                            None,
                            None,
                            Some(CoinBonus::default()),
                        ),
                    ];

                    for (
                        label,
                        border_col,
                        bg_col,
                        btn_ability_opt,
                        dmg_all_opt,
                        freeze_opt,
                        coin_bonus_opt,
                    ) in abilities
                    {
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

                        if let Some(btn_ability) = btn_ability_opt {
                            entity.insert(btn_ability);
                        }
                        if let Some(dmg_all) = dmg_all_opt {
                            entity.insert(dmg_all);
                        }
                        if let Some(freeze) = freeze_opt {
                            entity.insert(freeze);
                        }
                        if let Some(coin_bonus) = coin_bonus_opt {
                            entity.insert(coin_bonus);
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

#[allow(clippy::type_complexity)]
pub fn handle_ability_button_clicks(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &AbilityButton,
            Option<&mut DamageAll>,
            Option<&mut Freeze>,
            Option<&mut CoinBonus>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut q_enemies: Query<(Entity, &mut Enemy)>,
    mut kill_count: ResMut<KillCount>,
    mut coin_count: ResMut<CoinCount>,
) {
    for (interaction, button_action, mut damage_all_opt, mut freeze_opt, mut coin_bonus_opt) in
        &mut interaction_query
    {
        if *interaction == Interaction::Pressed {
            match button_action {
                AbilityButton::DamageAll => {
                    if let Some(ref mut damage_all) = damage_all_opt
                        && damage_all.reload.is_finished()
                    {
                        // Damage scales with score/kills: base damage (10) + 0.5 damage per kill
                        let scaled_dmg = (damage_all.dmg + kill_count.0 / 2) as i32;
                        for (enemy_entity, mut enemy) in &mut q_enemies {
                            if enemy.hp > 0 {
                                enemy.hp -= scaled_dmg;
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
                AbilityButton::Freeze => {
                    if let Some(ref mut freeze) = freeze_opt
                        && freeze.reload.is_finished()
                    {
                        // Freeze duration stays constant (3.0s) throughout the game
                        for (enemy_entity, _) in &q_enemies {
                            commands
                                .entity(enemy_entity)
                                .insert(Frozen(Timer::from_seconds(
                                    freeze.duration,
                                    TimerMode::Once,
                                )));
                        }
                        freeze.reload.reset();
                    }
                }
                AbilityButton::Coins => {
                    if let Some(ref mut coin_bonus) = coin_bonus_opt
                        && coin_bonus.reload.is_finished()
                    {
                        // Coin reward scales with kills: base coins (20) + 2 coins per kill
                        let scaled_coins = coin_bonus.coins + (kill_count.0 * 2);
                        coin_count.0 += scaled_coins;
                        coin_bonus.reload.reset();
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn tick_ability_cooldowns(
    time: Res<Time>,
    mut q_damage_all: Query<
        (&mut DamageAll, &mut BackgroundColor),
        (Without<Freeze>, Without<CoinBonus>),
    >,
    mut q_freeze: Query<
        (&mut Freeze, &mut BackgroundColor),
        (Without<DamageAll>, Without<CoinBonus>),
    >,
    mut q_coin_bonus: Query<
        (&mut CoinBonus, &mut BackgroundColor),
        (Without<DamageAll>, Without<Freeze>),
    >,
) {
    for (mut damage_all, mut bg_color) in &mut q_damage_all {
        damage_all.reload.tick(time.delta());
        if damage_all.reload.is_finished() {
            *bg_color = BackgroundColor(Color::srgb(0.1, 0.4, 0.8));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.2));
        }
    }

    for (mut freeze, mut bg_color) in &mut q_freeze {
        freeze.reload.tick(time.delta());
        if freeze.reload.is_finished() {
            *bg_color = BackgroundColor(Color::srgb(0.1, 0.4, 0.5));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.2));
        }
    }

    for (mut coin_bonus, mut bg_color) in &mut q_coin_bonus {
        coin_bonus.reload.tick(time.delta());
        if coin_bonus.reload.is_finished() {
            *bg_color = BackgroundColor(Color::srgb(0.5, 0.35, 0.1));
        } else {
            *bg_color = BackgroundColor(Color::srgb(0.15, 0.15, 0.2));
        }
    }
}
