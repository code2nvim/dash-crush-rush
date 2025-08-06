use bevy::prelude::*;

pub struct DestroyPlugin;

impl Plugin for DestroyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Destroy>();
    }
}

#[derive(Event)]
pub struct Destroy(pub Reason);

pub enum Reason {
    Enemy,
}
