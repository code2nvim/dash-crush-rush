use super::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, rotate_player))
            .add_systems(Update, (fire_bullet, locate_cursor))
            .insert_resource(Cursor(Vec3::new(0.0, 0.0, 0.0)))
            .insert_resource(FireTimer(Timer::from_seconds(
                cfg::bullet::INTERVAL,
                TimerMode::Repeating,
            )));
    }
}
