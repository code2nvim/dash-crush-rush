use super::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_plugins(DestroyPlugin)
            .add_systems(
                Update,
                (
                    (move_player, leap_player, rotate_player, despawn_player),
                    (spawn_bullet, move_bullet, despawn_bullet),
                    (spawn_enemy, move_enemy, despawn_enemy),
                ),
            )
            .insert_resource(Bullets {
                first: true,
                timer: Timer::from_seconds(cfg::bullet::INTERVAL, TimerMode::Repeating),
            })
            .insert_resource(Enemies {
                corner: true,
                timer: Timer::from_seconds(cfg::enemy::INTERVAL, TimerMode::Repeating),
            });
    }
}
