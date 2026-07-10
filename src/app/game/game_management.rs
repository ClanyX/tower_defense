use crate::app::game::enemy::Enemy;
use crate::app::game::player::Player;
use bevy::prelude::*;

// System to control player shooting
pub fn player_shooting(
    mut commands: Commands,
    time: Res<Time>,
    mut q_player: Query<(&Transform, &mut Player)>,
    mut q_enemies: Query<(Entity, &Transform, &mut Enemy)>,
) {
    // Only shoot if the player is spawned
    let (player_transform, mut player) = match q_player.single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    // If there are no enemies, do not shoot or tick reload timer
    if q_enemies.is_empty() {
        return;
    }

    // Tick the player's reload timer
    player.reload.tick(time.delta());

    // When the reload timer finishes, shoot the nearest enemy
    if player.reload.is_finished() {
        let player_pos = player_transform.translation;
        let mut nearest_enemy: Option<(Entity, f32)> = None;

        for (enemy_entity, enemy_transform, _) in &q_enemies {
            let distance = player_pos.distance(enemy_transform.translation);
            if let Some((_, min_dist)) = nearest_enemy {
                if distance < min_dist {
                    nearest_enemy = Some((enemy_entity, distance));
                }
            } else {
                nearest_enemy = Some((enemy_entity, distance));
            }
        }

        // Apply damage to the nearest enemy
        if let Some((enemy_entity, _)) = nearest_enemy
            && let Ok((_, _, mut enemy)) = q_enemies.get_mut(enemy_entity)
        {
            if enemy.hp > 0 {
                //TODO: change value depend on the player damage
                enemy.hp -= 1;
                // If enemy dies, despawn it (Bevy 0.19 automatically handles hierarchy cleanup)
                if enemy.hp == 0 {
                    commands.entity(enemy_entity).despawn();
                }
            }
            // Reset reload timer for the next shot
            player.reload.reset();
        }
    }
}
