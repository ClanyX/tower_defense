use crate::app::game::logic::CleanupGame;
use bevy::prelude::*;
use rand::RngExt;

#[derive(Component)]
pub struct Enemy {
    //pub hp: u32,
    pub direction: Vec2,
    pub speed: f32,
}

impl Enemy {
    pub fn spawn_enemy(commands: &mut Commands, _window_height: &f32) {
        let mut rng = rand::rng();
        let rnd: f32 = rng.random_range(60.0..f32::from(*_window_height));
        println!("W height: {}", *_window_height);
        commands.spawn((
            Sprite {
                color: Color::from(Srgba::RED),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            CleanupGame,
            //maybe need to implement x and y and size
            Transform::from_xyz(-50.0, rnd, 0.0),
            Enemy {
                //hp: 5,
                direction: Vec2::new(1.0, 0.0),
                speed: 100.0,
            },
        ));
    }
}

pub fn move_enemy(mut query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in &mut query {
        let displacement = enemy.direction * enemy.speed * time.delta_secs();
        transform.translation += displacement.extend(0.0);
    }
}
