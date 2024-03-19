use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod constants;
mod player;
mod mortality;

use mortality::MortalityPlugin;

use constants::GAMETIME_SCALE;

fn main() {
    App::new()
        // Plugins
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(bevy::log::LogPlugin {
                level: bevy::log::Level::DEBUG,
                filter: "wgpu=warn,bevy_ecs=info,naga=info,bevy_render=info,bevy_app=info".to_string(),
                ..default()
            }),
        )
        .add_plugins(EguiPlugin)
        .add_plugins(MortalityPlugin)
        // Resources
        .insert_resource(Time::<Fixed>::from_seconds(1.00 * GAMETIME_SCALE))
        // Events
        // Systems
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_time,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                debug_every_second,
            )
        )
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_time(mut time: ResMut<Time<Virtual>>) {
    time.set_relative_speed(GAMETIME_SCALE as f32);
}

pub fn debug_every_second() {
    //debug!("A second passes...");
}