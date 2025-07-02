use super::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    (move_player, reset_player, rotate_player, despawn_player),
                    (spawn_enemy, move_enemy, despawn_enemy),
                    (spawn_bullet, move_bullet, despawn_bullet),
                ),
            )
            .insert_resource(Fire {
                first: true,
                timer: Timer::from_seconds(cfg::bullet::INTERVAL, TimerMode::Repeating),
            })
            .insert_resource(Casual {
                vertical: true,
                timer: Timer::from_seconds(cfg::enemy::INTERVAL, TimerMode::Repeating),
            });
    }
}
