use super::*;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb_u8(23, 147, 209))) // #1793d1: Arch Linux
            .add_systems(Startup, (spawn_camera, spawn_ground, spawn_light))
            .add_plugins((BulletPlugin, DestroyPlugin, EnemyPlugin, PlayerPlugin));
    }
}
