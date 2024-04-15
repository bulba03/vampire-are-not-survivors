
use bevy::prelude::*;
pub mod player;
pub mod animation;
pub mod camera;

// const MAP_SIZE: f32 = 1000.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins) 
        .add_plugins(
            (
                animation::AnimationPlugin,
                player::PlayerPlugin,
                camera::CameraPlugin
            )
        )
        .run();
}