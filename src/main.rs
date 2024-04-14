use bevy::prelude::*;
use bevy::diagnostic::LogDiagnosticsPlugin;
pub mod player;
pub mod animation;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins((animation::AnimationPlugin, player::PlayerPlugin))
        .add_systems(Startup, setup_cam)
        .run();
}

fn setup_cam(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}