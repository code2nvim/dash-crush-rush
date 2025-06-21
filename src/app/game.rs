use super::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, rotate_player))
            .add_systems(Update, (fire_bullet, drive_bullet))
            .insert_resource(Fire {
                first: true,
                timer: Timer::from_seconds(cfg::bullet::INTERVAL, TimerMode::Repeating),
            });
    }
}
