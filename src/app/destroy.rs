use bevy::prelude::*;

pub struct DestroyPlugin;

impl Plugin for DestroyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DestroyEnemy>().add_event::<DestroyPlayer>();
    }
}

#[derive(Event)]
pub struct DestroyEnemy(pub Entity);

#[derive(Event)]
pub struct DestroyPlayer(pub PlayerReason);

pub enum PlayerReason {
    Enemy,
}
