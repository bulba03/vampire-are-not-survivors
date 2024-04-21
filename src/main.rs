use bevy::prelude::*;
use bevy_xpbd_2d::{math::Vector, resources::Gravity};
use player::Player;
pub mod player;
pub mod animation;
pub mod camera;
pub mod monster;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        .insert_resource(Gravity(Vector::ZERO))
        .add_plugins((
            animation::AnimationPlugin,
            player::PlayerPlugin,
            camera::CameraPlugin,
            monster::MonsterPlugin,
        ))
        .run();
}

fn run_if_player_alive(pl_q: Query<&Player>) -> bool {
    pl_q.get_single().is_ok()
}
