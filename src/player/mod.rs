pub mod movement;

use bevy::time::{Timer, TimerMode};
use bevy::transform::components::Transform;
use bevy::sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout};
use bevy::prelude::default;
use bevy::math::{Vec2, Vec3};
use bevy::ecs::{component::Component, system::{Commands, Res, ResMut}};
use bevy::asset::{AssetServer, Assets};
use bevy::app::{Plugin, Startup, Update};
use crate::animation::{AnimationIndices, AnimationTimer};
use self::movement::handle_movement;

const SKELETON_WALK_ANIM: &str = "monster/Monsters_Creatures_Fantasy/Skeleton/Walk.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, handle_movement);
    }
}


#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub base_damage: f32,
    pub is_moving: bool
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
    ) { 
        let texture = asset_server.load(SKELETON_WALK_ANIM);
        let layout = TextureAtlasLayout::from_grid(Vec2::new(150.0, 150.0), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        
        commands.spawn(
            (   
                SpriteSheetBundle{
                    texture,
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    },
                    transform: Transform::from_scale(Vec3::splat(1.)),
                    ..default()
                
                },
                Player {
                    speed: 100.0,
                    base_damage: 10.,
                    is_moving: false
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating))
            )
        );
}