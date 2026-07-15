use crate::app::game::logic::CleanupGame;
use bevy::prelude::*;

#[derive(Component)]
pub struct Tower {
    pub hp: i32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Tower {
    pub fn new(window_width: &f32, window_height: &f32) -> Tower {
        const TOWER_WIDTH: f32 = 160.0;
        const HUD_HEIGHT: f32 = 60.0;

        Tower {
            hp: 100,
            x: (window_width / 2.0) - (TOWER_WIDTH / 2.0),
            y: 0.0 - (HUD_HEIGHT / 2.0),
            width: TOWER_WIDTH,
            height: *window_height - HUD_HEIGHT,
        }
    }

    pub fn spawn_tower(commands: &mut Commands, tower: Tower) {
        commands.spawn((
            Sprite {
                color: Color::from(Srgba::BLUE),
                custom_size: Some(Vec2::new(tower.width, tower.height)),
                ..default()
            },
            CleanupGame,
            Transform::from_xyz(tower.x, tower.y, 0.0),
            tower,
        ));
    }
}
