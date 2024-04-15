use bevy::transform::components::Transform;
use bevy::sprite::{ImageScaleMode, SpriteBundle};
use bevy::prelude::default;
use bevy::math::Vec3;
use bevy::ecs::{query::{With, Without}, system::{Commands, Query, Res}};
use bevy::core_pipeline::core_2d::{Camera2d, Camera2dBundle};
use bevy::asset::AssetServer;
use bevy::app::{Plugin, Startup, Update};

use crate::player::Player;

const BG_GRID_TEXTURE: &str = "grid/dark/texture_03.png";

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(Startup, setup_cam)
            .add_systems(Update, follow_cam_by_player);
    }
}
fn setup_cam(
    mut commands: Commands,
    asset_server: Res<AssetServer>) 
    {
        commands.spawn(
            (
                    SpriteBundle {
                        texture: asset_server.load(BG_GRID_TEXTURE),
                        transform: Transform::from_xyz(0.0,0.,-100.)
                                    .with_scale(Vec3::splat(200.)),
                        ..default()
                    },
                    ImageScaleMode::Tiled { tile_x: true, tile_y: true, stretch_value: 0.01 }
                )
            );
        
        commands.spawn(Camera2dBundle::default());

}

fn follow_cam_by_player(
        mut _commands: Commands, 
        mut query: Query<&Transform, With<Player>>, 
        mut camera_q: Query<&mut Transform, (With<Camera2d>, Without<Player>)>) {
        
            let mut transform = camera_q.single_mut();
            let player_pos = query.single_mut();

            transform.translation.x = player_pos.translation.x;
            transform.translation.y = player_pos.translation.y;
}