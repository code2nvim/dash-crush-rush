use super::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_camera, spawn_ground, spawn_light, spawn_player),
        )
        .add_systems(Update, (move_player, rotate_player));
    }
}
