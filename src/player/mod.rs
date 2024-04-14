use bevy::{app::{Plugin, Startup, Update}, asset::{AssetServer, Assets}, ecs::{component::Component, system::{Commands, Res, ResMut}}, math::{Vec2, Vec3}, prelude::{default, Deref, DerefMut}, sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasLayout}, time::{Timer, TimerMode}, transform::components::Transform};

use crate::animation::{AnimationIndices, AnimationTimer};

use self::movement::handle_movement;


pub mod movement;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub base_damage: f32,
    pub is_moving: bool
}

fn spawn_player(mut commands: Commands,
                asset_server: Res<AssetServer>,
                mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let texture = asset_server.load("monster/Monsters_Creatures_Fantasy/Skeleton/Walk.png");
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
    ));
}


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, spawn_player)
        .add_systems(Update, handle_movement);
    }
}