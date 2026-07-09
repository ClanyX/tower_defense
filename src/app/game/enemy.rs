use crate::app::game::logic::CleanupGame;
use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    x: f32,
    y: f32,
    size: u32,
    hp: u32,
}

impl Enemy {
    pub fn new(_window_height: &f32) -> Enemy {
        let rnd = rand::random_range(0.0..f64::from(*_window_height));
        Enemy {
            x: -50.0,
            y: rnd as f32,
            size: 50,
            hp: 5,
        }
    }

    pub fn spawn_enemy(mut commands: Commands, enemy: Enemy) {
        commands.spawn((
            Sprite {
                color: Color::from(Srgba::RED),
                custom_size: Some(Vec2::new(enemy.size as f32, enemy.size as f32)),
                ..default()
            },
            CleanupGame,
            Transform::from_xyz(enemy.x, enemy.y, 0.0),
        ));
    }

    pub fn move_enemy() {}
}
