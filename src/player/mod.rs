mod movement;
mod healthbar;
use bevy::time::{Timer, TimerMode};
use bevy::transform::components::Transform;
use bevy::sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout};
use bevy::prelude::default;
use bevy::math::{Vec2, Vec3};
use bevy::ecs::{component::Component, system::{Commands, Res, ResMut}};
use bevy::asset::{AssetServer, Assets};
use bevy::app::{Plugin, PostUpdate, Startup, Update};
use crate::animation::{AnimationIndices, AnimationTimer};
use self::healthbar::{spawn_healthbar, update_health_bar, HealthBar};
use self::movement::handle_movement;

const SKELETON_WALK_ANIM: &str = "monster/Monsters_Creatures_Fantasy/Skeleton/Walk.png";

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, handle_movement)
        .add_systems(PostUpdate, update_health_bar);
    }
}


#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub base_damage: f32,
    pub is_moving: bool,
}
#[derive(Component)]
pub struct Health {
    pub max: f32,
    pub current: f32
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
        
        let entity = commands.spawn(
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
                Health {
                    max: 100.,
                    current: 100.
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                HealthBar::default()
            )
        ).id();
        spawn_healthbar(commands, entity);
}