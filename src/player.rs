use bevy::prelude::*;

// Components

#[derive(Debug, Clone, Component)]
pub struct Player{
    pub name: String,
}

/// pause or resume `Relative` time
fn toggle_pause(mut time: ResMut<Time<Virtual>>) {
    if time.is_paused() {
        time.unpause();
    } else {
        time.pause();
    }
}
