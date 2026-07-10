use crate::app::game::logic::CleanupGame;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub size: u32,
}

impl Player {
    pub fn new(window_width: &f32, _window_height: &f32) -> Player {
        Player {
            x: (window_width / 2.0) - (window_width * 0.1),
            y: 0.0,
            size: 50,
        }
    }

    pub fn spawn_player(commands: &mut Commands, player: Player) {
        commands.spawn((
            Sprite {
                color: Color::from(Srgba::GREEN),
                custom_size: Some(Vec2::new(player.size as f32, player.size as f32)),
                ..default()
            },
            CleanupGame,
            Transform::from_xyz(player.x, player.y, 0.0),
        ));
    }
}
