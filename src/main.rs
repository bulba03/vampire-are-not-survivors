use bevy::prelude::*;
pub mod player;
pub mod animation;
pub mod camera;
pub mod monster;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            animation::AnimationPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            monster::MonsterPlugin,
        ))
        .run();
}
