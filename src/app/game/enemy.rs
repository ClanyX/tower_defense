use crate::app::game::logic::CleanupGame;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::RngExt;

#[derive(Component)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub size: u32,
    pub hp: i32,
    pub direction: Vec2,
    pub speed: f32,
    pub reward: u32,
}

#[derive(Component)]
pub struct EnemyHpText;

impl Enemy {
    pub fn new(window_width: &f32, window_height: &f32) -> Enemy {
        let mut rng = rand::rng();
        let half_width = window_width / 2.0;
        let half_height = window_height / 2.0;
        let hud_height = 60.0;

        // Spawn completely off-screen on the left (50px padding to avoid sudden pop-in)
        let spawn_x = -half_width - 50.0;

        // Random Y position in the playable area (below the 60px HUD and above the bottom edge)
        // Adjust with 25.0 (half of the enemy's 50.0 size) to prevent clipping top/bottom edges
        let min_y = -half_height + 25.0;
        let max_y = half_height - hud_height - 25.0;
        let rnd: f32 = rng.random_range(min_y..max_y);

        Enemy {
            x: spawn_x,
            y: rnd,
            size: 50,
            hp: 5,
            direction: Vec2::new(1.0, 0.0),
            speed: 100.0,
            reward: 2,
        }
    }

    pub fn spawn_enemy(commands: &mut Commands, enemy: Enemy) {
        let initial_hp = enemy.hp;
        let size_f = enemy.size as f32;
        let x_pos = enemy.x;
        let y_pos = enemy.y;

        commands
            .spawn((
                Sprite {
                    color: Color::from(Srgba::RED),
                    custom_size: Some(Vec2::new(size_f, size_f)),
                    ..default()
                },
                CleanupGame,
                Transform::from_xyz(x_pos, y_pos, 0.0),
                enemy,
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text2d::new(initial_hp.to_string()),
                    TextFont {
                        font_size: FontSize::Px(20.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Transform::from_xyz(0.0, 0.0, 1.0),
                    EnemyHpText,
                ));
            });
    }
}

pub fn move_enemy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Enemy)>,
    time: Res<Time>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = if let Ok(w) = q_window.single() {
        w
    } else {
        return;
    };

    // Despawn boundary is 50px past the right edge of the screen
    let right_limit = window.width() / 2.0 + 50.0;

    for (entity, mut transform, enemy) in &mut query {
        let displacement = enemy.direction * enemy.speed * time.delta_secs();
        transform.translation += displacement.extend(0.0);

        // Despawn enemy once it moves fully off-screen to the right
        if transform.translation.x > right_limit {
            commands.entity(entity).despawn();
        }
    }
}

// System to update the text value on the enemy with the current HP of the struct
pub fn update_enemy_hp_text(
    q_enemies: Query<(&Enemy, &Children)>,
    mut q_text: Query<&mut Text2d, With<EnemyHpText>>,
) {
    for (enemy, children) in &q_enemies {
        for &child in children {
            if let Ok(mut text) = q_text.get_mut(child) {
                let hp_str = enemy.hp.to_string();
                if text.0 != hp_str {
                    text.0 = hp_str;
                }
            }
        }
    }
}
